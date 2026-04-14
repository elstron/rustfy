use gtk::gio;

const RESOURCE_PATHS: &[&str] = &[
    "builddir/data/rustfy.gresource",
    "/usr/local/share/rustfy/rustfy.gresource",
    "/usr/share/rustfy/rustfy.gresource",
];

fn find_resource_path() -> Option<String> {
    RESOURCE_PATHS
        .iter()
        .find(|path| std::path::Path::new(path).exists())
        .map(|s| s.to_string())
}

pub fn load_resources() -> Result<(), String> {
    let path = find_resource_path().ok_or_else(|| {
        format!(
            "Resource file not found. Tried: {:?}\nRun: meson compile -C builddir",
            RESOURCE_PATHS
        )
    })?;

    let resource = gio::Resource::load(&path)
        .map_err(|e| format!("Failed to load resource from {}: {}", path, e))?;

    gio::resources_register(&resource);

    #[cfg(debug_assertions)]
    eprintln!("✓ Loaded resources from: {}", path);

    Ok(())
}
