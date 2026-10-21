use crate::app_state::{encode_value, AppDBTables, AppState, DeleteEntry};
use crate::error::{Error, Result};
use crate::name::EntitySelector;
use futures_util::future::BoxFuture;
use kuma_client::Client;

pub fn migrate<'a>(
    tables: &'a AppDBTables,
    _state: &'a AppState,
    _kuma: &'a Client,
) -> BoxFuture<'a, Result<()>> {
    Box::pin(async move {
        let tree = &tables.to_delete.tree;
        let mut batch = sled::Batch::default();
        for entry in tree.iter() {
            let (old_key, old_value): (sled::IVec, sled::IVec) =
                entry.map_err(|e| Error::InternalError(e.to_string()))?;
            let entity = v0::decode_entity_selector(&old_key)?;
            let delete_entry = v0::decode_delete_entry(&old_value)?;
            batch.remove(&old_key);
            batch.insert(encode_value(entity)?, encode_value(delete_entry)?);
        }
        tree.apply_batch(batch)?;
        Ok(())
    })
}

/// Decodes the legacy v0 bincode-encoded database entries.
///
/// bincode 2.0 `standard()` config: little-endian, zig-zag varint integers,
/// BincodeLen (same varint scheme) for sequence lengths.
///
/// Varint: 0–250 = 1 byte; 251 + 2 LE bytes for ≤65535; 252 + 4 LE bytes for
/// ≤u32::MAX; 253 + 8 LE bytes for larger values.
/// Signed i32: zig-zag encoded as unsigned, then varint.
mod v0 {
    use super::*;
    use std::str;

    fn read_uint(b: &[u8]) -> Result<(u64, &[u8])> {
        match b {
            [v, rest @ ..] if *v <= 250 => Ok((*v as u64, rest)),
            [251, lo, hi, rest @ ..] => Ok((u16::from_le_bytes([*lo, *hi]) as u64, rest)),
            [252, b0, b1, b2, b3, rest @ ..] => {
                Ok((u32::from_le_bytes([*b0, *b1, *b2, *b3]) as u64, rest))
            }
            [253, b0, b1, b2, b3, b4, b5, b6, b7, rest @ ..] => Ok((
                u64::from_le_bytes([*b0, *b1, *b2, *b3, *b4, *b5, *b6, *b7]),
                rest,
            )),
            _ => Err(Error::InternalError("truncated bincode varint".to_owned())),
        }
    }

    fn read_i32(b: &[u8]) -> Result<(i32, &[u8])> {
        let (v, rest) = read_uint(b)?;
        Ok((((v >> 1) as i32) ^ -((v & 1) as i32), rest))
    }

    fn read_string(b: &[u8]) -> Result<(String, &[u8])> {
        let (len, rest) = read_uint(b)?;
        let len = len as usize;
        let s = str::from_utf8(
            rest.get(..len)
                .ok_or_else(|| Error::InternalError("truncated bincode string".to_owned()))?,
        )
        .map_err(|e| Error::InternalError(format!("bincode utf8: {e}")))?
        .to_owned();
        Ok((s, &rest[len..]))
    }

    pub fn decode_entity_selector(b: &[u8]) -> Result<EntitySelector> {
        let (tag, rest) = read_uint(b)?;
        Ok(match tag {
            0 => {
                let (s, r) = read_string(rest)?;
                let (i, _) = read_i32(r)?;
                EntitySelector::Monitor(s, i)
            }
            1 => {
                let (s, r) = read_string(rest)?;
                let (i, _) = read_i32(r)?;
                EntitySelector::Notification(s, i)
            }
            2 => {
                let (s, r) = read_string(rest)?;
                let (i, _) = read_i32(r)?;
                EntitySelector::DockerHost(s, i)
            }
            3 => {
                let (s, r) = read_string(rest)?;
                let (i, _) = read_i32(r)?;
                EntitySelector::Tag(s, i)
            }
            4 => {
                let (s1, r) = read_string(rest)?;
                let (s2, _) = read_string(r)?;
                EntitySelector::StatusPage(s1, s2)
            }
            _ => {
                return Err(Error::InternalError(format!(
                    "unknown EntitySelector discriminant {tag}"
                )))
            }
        })
    }

    pub fn decode_delete_entry(b: &[u8]) -> Result<DeleteEntry> {
        let (dt_str, rest) = read_string(b)?;
        let delete_at = chrono::DateTime::parse_from_rfc3339(&dt_str)
            .map_err(|e| Error::InternalError(format!("bincode datetime: {e}")))?
            .with_timezone(&chrono::Utc);
        Ok(DeleteEntry {
            delete_at,
            entity: decode_entity_selector(rest)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;

    fn bincode_encode<V: serde::Serialize>(v: &V) -> Vec<u8> {
        bincode::serde::encode_to_vec(v, bincode::config::standard()).unwrap()
    }

    fn assert_entity_roundtrip(entity: &EntitySelector) {
        let bytes = bincode_encode(entity);
        let decoded = v0::decode_entity_selector(&bytes).unwrap();
        assert_eq!(*entity, decoded);

        let postcard_bytes = postcard::to_allocvec(&decoded).unwrap();
        let final_decoded: EntitySelector = postcard::from_bytes(&postcard_bytes).unwrap();
        assert_eq!(*entity, final_decoded);
    }

    #[test]
    fn entity_selector_all_variants() {
        assert_entity_roundtrip(&EntitySelector::Monitor("test".to_string(), 42));
        assert_entity_roundtrip(&EntitySelector::Monitor("".to_string(), 0));
        assert_entity_roundtrip(&EntitySelector::Monitor("x".to_string(), i32::MAX));
        assert_entity_roundtrip(&EntitySelector::Monitor("y".to_string(), i32::MIN));
        assert_entity_roundtrip(&EntitySelector::Monitor("z".to_string(), -1));
        assert_entity_roundtrip(&EntitySelector::Notification("notif".to_string(), 1));
        assert_entity_roundtrip(&EntitySelector::DockerHost("host".to_string(), 99));
        assert_entity_roundtrip(&EntitySelector::Tag("tag".to_string(), 7));
        assert_entity_roundtrip(&EntitySelector::StatusPage(
            "page".to_string(),
            "slug".to_string(),
        ));
        assert_entity_roundtrip(&EntitySelector::StatusPage("".to_string(), "".to_string()));
    }

    #[test]
    fn delete_entry_roundtrip() {
        let cases = vec![
            DeleteEntry {
                delete_at: DateTime::from_timestamp(1_700_000_000, 0).unwrap(),
                entity: EntitySelector::Monitor("test".to_string(), 42),
            },
            DeleteEntry {
                delete_at: DateTime::from_timestamp(1_700_000_000, 123_456_789).unwrap(),
                entity: EntitySelector::StatusPage("page".to_string(), "slug".to_string()),
            },
            DeleteEntry {
                delete_at: DateTime::from_timestamp(0, 0).unwrap(),
                entity: EntitySelector::Tag("t".to_string(), 0),
            },
        ];

        for entry in &cases {
            let bytes = bincode_encode(entry);
            let decoded = v0::decode_delete_entry(&bytes).unwrap();
            assert_eq!(entry.delete_at, decoded.delete_at);
            assert_eq!(entry.entity, decoded.entity);
        }
    }
}
