use std::process::exit;
use std::path::Path;
use cargo_toml::*;

fn main() {
    let mut manifest = Manifest::from_path("Cargo.toml")
        .unwrap_or_else(|e| {
            eprintln!("manifest not found: {e}");
            exit(1);
        });
    manifest
        .complete_from_path(Path::new("."))
        .unwrap_or_else(|e| {
            eprintln!("library data not found: {e}");
            exit(1);
        });
    if let Some(product) = manifest.lib {
        println!("{}", product.name.unwrap());
    } else {
        eprintln!("not a lib crate");
        exit(1);
    }
}
