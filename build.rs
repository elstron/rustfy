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
            _ => {
                panic!("glib-compile-resources failed. Please run 'meson setup builddir && meson compile -C builddir' first.");
            }
        }
    }

    println!("cargo:rerun-if-changed=builddir/data/rustfy.gresource");
    println!("cargo:rerun-if-changed=data/rustfy.gresource.xml");
}
