use std::env;
use std::iter::IntoIterator;
use std::path::{Path, PathBuf};

fn list_files_in_directory(directory: &str) -> impl Iterator<Item = PathBuf> {
    Path::new(&format!("{}/acpica/src", directory))
        .read_dir()
        .expect("acpica C source not present at [root]/acpica/src")
        .into_iter()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_type().unwrap().is_file())
        .map(|file| file.path().to_path_buf())
}

fn main() {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=acpica.h");

    let c_files = list_files_in_directory(&root)
        .filter(|path| path.extension().map_or(false, |f| f == "c"))
        .inspect(|path| println!("cargo:rerun-if-changed={}", path.display()))
        .collect::<Vec<_>>();

    list_files_in_directory(&root)
        .filter(|path| path.extension().map_or(false, |f| f == "h"))
        .for_each(|path| println!("cargo:rerun-if-changed={}", path.display()));

    cc::Build::new()
        .no_default_flags(true)
        .flag("-nostdlib")
        .flag("-m64")
        .flag("-nodefaultlibs")
        .flag("-Wno-unused-parameter")
        .flag("-g")
        .flag("-fno-stack-protector")
        .static_flag(true)
        .files(c_files)
        .compile("libapcica.a");

    let bindings = bindgen::Builder::default()
        .header(&format!("{}/acpica.h", root))
        .ctypes_prefix("::cty")
        .use_core()
        .generate()
        .expect("Unable to generate acpica bindings");

    let out_path = PathBuf::from(&out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings");
}
