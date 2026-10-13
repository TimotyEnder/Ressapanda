use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os != "windows" {
        return;
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));

    // Resource inputs that should trigger a rebuild when changed.
    println!("cargo:rerun-if-changed=building/appicon.rc");
    println!("cargo:rerun-if-changed=Assets/application_icon.ico");

    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    match target_env.as_str() {
        // Native Windows (GNU toolchain via MinGW): use windres to compile the .rc file.
        "gnu" => {
            let object_path = out_dir.join("appicon.o");
            let status = Command::new("windres")
                .current_dir(&manifest_dir)
                .args([
                    "building/appicon.rc",
                    "-O",
                    "coff",
                    "-o",
                    object_path.to_str().expect("object path"),
                ])
                .status()
                .expect(
                    "failed to run `windres`. Install MinGW-w64 (provides windres) and ensure it is on PATH",
                );

            if !status.success() {
                panic!("windres failed to compile building/appicon.rc");
            }

            println!("cargo:rustc-link-arg={}", object_path.display());
        }

        // Native Windows (MSVC toolchain): use the Windows SDK resource compiler if available.
        "msvc" => {
            let resource_path = out_dir.join("appicon.res");
            let status = Command::new("rc.exe")
                .current_dir(&manifest_dir)
                .args([
                    "/fo",
                    resource_path.to_str().expect("resource path"),
                    "building/appicon.rc",
                ])
                .status();

            match status {
                Ok(s) if s.success() => {}
                _ => panic!(
                    "could not run `rc.exe`. Install the Windows SDK (provides rc.exe) and ensure it is on PATH"
                ),
            }

            println!("cargo:rustc-link-arg={}", resource_path.display());
        }

        _ => {}
    }
}
