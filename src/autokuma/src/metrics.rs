use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use kuma_client::build::SHORT_VERSION;

use crate::entity::Entity;

const BUCKETS: &[f64] = &[0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0, 120.0];
const N_BUCKETS: usize = 8;

pub const ENTITY_TYPES: &[&str] =
    &["monitor", "docker_host", "notification", "tag", "status_page"];
const N_TYPES: usize = 5;

pub fn entity_type_index(entity: &Entity) -> usize {
    match entity {
        Entity::Monitor(_) => 0,
        Entity::DockerHost(_) => 1,
        Entity::Notification(_) => 2,
        Entity::Tag(_) => 3,
        Entity::StatusPage(_) => 4,
    }
}

struct HistogramState {
    sum: f64,
    count: u64,
    // Non-cumulative counts per bucket; rendered as cumulative in Prometheus format.
    bucket_counts: [u64; N_BUCKETS],
}

impl HistogramState {
    fn observe(&mut self, value: f64) {
        self.sum += value;
        self.count += 1;
        for (i, &upper) in BUCKETS.iter().enumerate() {
            if value <= upper {
                self.bucket_counts[i] += 1;
                break;
            }
        }
    }
}

pub struct Metrics {
    // Sync health
    pub last_sync_timestamp: AtomicI64,
    pub syncs_total_success: AtomicU64,
    pub syncs_total_error: AtomicU64,
    sync_duration: Mutex<HistogramState>,

    // Connection
    pub connected: AtomicBool,

    // Managed entities [monitor, docker_host, notification, tag, status_page]
    pub managed: [AtomicI64; N_TYPES],

    // Entity operations [monitor, docker_host, notification, tag, status_page]
    pub ops_created: [AtomicU64; N_TYPES],
    pub ops_updated: [AtomicU64; N_TYPES],
    pub ops_deleted: [AtomicU64; N_TYPES],

    // Deletion grace period queue
    pub pending_deletion: AtomicI64,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            last_sync_timestamp: AtomicI64::new(-1),
            syncs_total_success: AtomicU64::new(0),
            syncs_total_error: AtomicU64::new(0),
            sync_duration: Mutex::new(HistogramState {
                sum: 0.0,
                count: 0,
                bucket_counts: [0; N_BUCKETS],
            }),
            connected: AtomicBool::new(false),
            managed: std::array::from_fn(|_| AtomicI64::new(0)),
            ops_created: std::array::from_fn(|_| AtomicU64::new(0)),
            ops_updated: std::array::from_fn(|_| AtomicU64::new(0)),
            ops_deleted: std::array::from_fn(|_| AtomicU64::new(0)),
            pending_deletion: AtomicI64::new(0),
        }
    }

    pub fn record_sync_success(&self, duration: Duration) {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        self.last_sync_timestamp.store(ts, Ordering::Relaxed);
        self.syncs_total_success.fetch_add(1, Ordering::Relaxed);
        self.connected.store(true, Ordering::Relaxed);
        if let Ok(mut h) = self.sync_duration.lock() {
            h.observe(duration.as_secs_f64());
        }
    }

    pub fn record_sync_error(&self, is_connection_error: bool) {
        self.syncs_total_error.fetch_add(1, Ordering::Relaxed);
        if is_connection_error {
            self.connected.store(false, Ordering::Relaxed);
        }
    }

    pub fn inc_created(&self, entity: &Entity) {
        self.ops_created[entity_type_index(entity)].fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_updated(&self, entity: &Entity) {
        self.ops_updated[entity_type_index(entity)].fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_deleted(&self, entity: &Entity) {
        self.ops_deleted[entity_type_index(entity)].fetch_add(1, Ordering::Relaxed);
    }

    pub fn set_managed_counts(&self, counts: [i64; N_TYPES]) {
        for (i, count) in counts.iter().enumerate() {
            self.managed[i].store(*count, Ordering::Relaxed);
        }
    }

    pub fn is_healthy(&self, sync_interval: f64) -> bool {
        let ts = self.last_sync_timestamp.load(Ordering::Relaxed);
        if ts < 0 {
            return false;
        }
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        (now - ts) as f64 <= sync_interval * 3.0
    }

    pub fn render_prometheus(&self) -> String {
        let mut out = String::with_capacity(2048);

        // autokuma_info
        out.push_str(&format!(
            "# HELP autokuma_info AutoKuma build information\n\
             # TYPE autokuma_info gauge\n\
             autokuma_info{{version=\"{}\"}}\t1\n\n",
            SHORT_VERSION
        ));

        // last_sync_timestamp
        let ts = self.last_sync_timestamp.load(Ordering::Relaxed);
        out.push_str(&format!(
            "# HELP autokuma_last_sync_timestamp_seconds Unix timestamp of the last successful sync cycle\n\
             # TYPE autokuma_last_sync_timestamp_seconds gauge\n\
             autokuma_last_sync_timestamp_seconds\t{ts}\n\n"
        ));

        // syncs_total
        let success = self.syncs_total_success.load(Ordering::Relaxed);
        let errors = self.syncs_total_error.load(Ordering::Relaxed);
        out.push_str(&format!(
            "# HELP autokuma_syncs_total Total number of completed sync cycles\n\
             # TYPE autokuma_syncs_total counter\n\
             autokuma_syncs_total{{status=\"success\"}}\t{success}\n\
             autokuma_syncs_total{{status=\"error\"}}\t{errors}\n\n"
        ));

        // sync_duration histogram
        if let Ok(h) = self.sync_duration.lock() {
            out.push_str(
                "# HELP autokuma_sync_duration_seconds Duration of sync cycles in seconds\n\
                 # TYPE autokuma_sync_duration_seconds histogram\n",
            );
            let mut cumulative = 0u64;
            for (i, &upper) in BUCKETS.iter().enumerate() {
                cumulative += h.bucket_counts[i];
                out.push_str(&format!(
                    "autokuma_sync_duration_seconds_bucket{{le=\"{upper}\"}}\t{cumulative}\n"
                ));
            }
            out.push_str(&format!(
                "autokuma_sync_duration_seconds_bucket{{le=\"+Inf\"}}\t{}\n\
                 autokuma_sync_duration_seconds_sum\t{}\n\
                 autokuma_sync_duration_seconds_count\t{}\n\n",
                h.count, h.sum, h.count
            ));
        }

        // connected
        let connected = self.connected.load(Ordering::Relaxed) as u8;
        out.push_str(&format!(
            "# HELP autokuma_uptime_kuma_connected Whether AutoKuma is connected to Uptime Kuma (1=connected, 0=disconnected)\n\
             # TYPE autokuma_uptime_kuma_connected gauge\n\
             autokuma_uptime_kuma_connected\t{connected}\n\n"
        ));

        // entities_managed
        out.push_str(
            "# HELP autokuma_entities_managed Number of entities currently managed by AutoKuma\n\
             # TYPE autokuma_entities_managed gauge\n",
        );
        for (i, &label) in ENTITY_TYPES.iter().enumerate() {
            let v = self.managed[i].load(Ordering::Relaxed);
            out.push_str(&format!(
                "autokuma_entities_managed{{type=\"{label}\"}}\t{v}\n"
            ));
        }
        out.push('\n');

        // entity_operations_total
        out.push_str(
            "# HELP autokuma_entity_operations_total Total number of entity CRUD operations\n\
             # TYPE autokuma_entity_operations_total counter\n",
        );
        for (i, &label) in ENTITY_TYPES.iter().enumerate() {
            let c = self.ops_created[i].load(Ordering::Relaxed);
            let u = self.ops_updated[i].load(Ordering::Relaxed);
            let d = self.ops_deleted[i].load(Ordering::Relaxed);
            out.push_str(&format!(
                "autokuma_entity_operations_total{{type=\"{label}\",op=\"create\"}}\t{c}\n\
                 autokuma_entity_operations_total{{type=\"{label}\",op=\"update\"}}\t{u}\n\
                 autokuma_entity_operations_total{{type=\"{label}\",op=\"delete\"}}\t{d}\n"
            ));
        }
        out.push('\n');

        // pending_deletion
        let pending = self.pending_deletion.load(Ordering::Relaxed);
        out.push_str(&format!(
            "# HELP autokuma_entities_pending_deletion Number of entities waiting in the deletion grace period queue\n\
             # TYPE autokuma_entities_pending_deletion gauge\n\
             autokuma_entities_pending_deletion\t{pending}\n"
        ));

        out
    }
}
