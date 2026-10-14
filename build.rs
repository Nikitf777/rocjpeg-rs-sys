use std::env;
use std::path::{Path, PathBuf};

fn contains_rocjpeg(dir: &Path) -> bool {
    ["librocjpeg.so", "librocjpeg.dylib", "rocjpeg.lib"]
        .iter()
        .any(|name| dir.join(name).exists())
}

fn main() {
    println!("cargo:rerun-if-env-changed=ROCJPEG_LIB_DIR");
    println!("cargo:rerun-if-env-changed=ROCM_PATH");

    let lib_dir = env::var_os("ROCJPEG_LIB_DIR").map(PathBuf::from).or_else(|| {
        let rocm_path = env::var_os("ROCM_PATH").unwrap_or_else(|| "/opt/rocm".into());
        ["lib", "lib64", "bin"]
            .iter()
            .map(|sub| PathBuf::from(&rocm_path).join(sub))
            .find(|dir| contains_rocjpeg(dir))
    });

    match lib_dir {
        Some(dir) => {
            println!("cargo:rustc-link-search=native={}", dir.display());
            println!("cargo:rustc-link-lib=dylib=rocjpeg");
        }
        None => {
            println!("cargo:warning=librocjpeg not found; set ROCJPEG_LIB_DIR or ROCM_PATH. Trying to link against system paths anyway.");
            println!("cargo:rustc-link-lib=dylib=rocjpeg");
        }
    }
}
