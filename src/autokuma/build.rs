use std::{env, fs, path::PathBuf};

fn main() {
    let migrations_dir = "src/migrations";
    println!("cargo:rerun-if-changed={migrations_dir}");

    let mut versions: Vec<u32> = fs::read_dir(migrations_dir)
        .expect("src/migrations directory not found")
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name();
            let stem = name.to_str()?.strip_suffix(".rs")?;
            let n: u32 = stem.strip_prefix('v')?.parse().ok()?;
            Some(n)
        })
        .collect();
    versions.sort_unstable();

    let mut out = String::new();
    for n in &versions {
        out.push_str(&format!(
            "mod v{n} {{\n    \
             include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/migrations/v{n}.rs\"));\n\
             }}\n"
        ));
    }

    let vec_entries = versions
        .iter()
        .map(|n| format!("v{n}::migrate"))
        .collect::<Vec<_>>()
        .join(", ");
    out.push_str(&format!(
        "pub(crate) static MIGRATIONS: LazyLock<Vec<MigrationFn>> =\n    \
         LazyLock::new(|| vec![{vec_entries}]);\n\
         pub(crate) static CURRENT_VERSION: LazyLock<i32> = \
         LazyLock::new(|| MIGRATIONS.len() as i32);\n"
    ));

    let dest = PathBuf::from(env::var("OUT_DIR").unwrap()).join("migrations_registry.rs");
    fs::write(dest, out).unwrap();
}
