use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let builddir_gresource = "builddir/data/rustfy.gresource";
    let target_gresource = "data/rustfy.gresource";

    if Path::new(builddir_gresource).exists() {
        fs::copy(builddir_gresource, target_gresource)
            .expect("Failed to copy gresource from builddir");
        println!("cargo:warning=Using pre-built gresource from meson builddir");
    } else {
        let status = Command::new("glib-compile-resources")
            .args(&[
                "--sourcedir=builddir/data",
                "--target=data/rustfy.gresource",
                "data/rustfy.gresource.xml",
            ])
            .status();

        match status {
            Ok(s) if s.success() => {}
            Ok(s) => {
                eprintln!("cargo:warning=glib-compile-resources returned non-zero exit code ({}). Skipping automatic resource compilation.", s);
                eprintln!("cargo:warning=Please run 'meson setup builddir && meson compile -C builddir' or provide a pre-built data/rustfy.gresource.");
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    eprintln!("cargo:warning=glib-compile-resources not found in PATH. Skipping automatic resource compilation.");
                    eprintln!("cargo:warning=On Windows you can install the GTK/GLib tooling (for example via MSYS2) or build resources with Meson.");
                } else {
                    eprintln!("cargo:warning=Failed to run glib-compile-resources: {}. Skipping automatic resource compilation.", e);
                }
                eprintln!("cargo:warning=Ensure data/rustfy.gresource exists or run Meson to build it before running the app.");
            }
        }
    }

    println!("cargo:rerun-if-changed=builddir/data/rustfy.gresource");
    println!("cargo:rerun-if-changed=data/rustfy.gresource.xml");
}
