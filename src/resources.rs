use gtk::gio;
use std::path::Path;

const BASE_RESOURCE_PATHS: &[&str] = &[
    "builddir/data/rustfy.gresource",
    "/usr/local/share/rustfy/rustfy.gresource",
    "/usr/share/rustfy/rustfy.gresource",
];

fn candidate_paths() -> Vec<String> {
    let mut paths: Vec<String> = BASE_RESOURCE_PATHS.iter().map(|s| s.to_string()).collect();

    #[cfg(target_os = "windows")]
    {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                paths.push(
                    dir.join("data")
                        .join("rustfy.gresource")
                        .to_string_lossy()
                        .into_owned(),
                );
                paths.push(dir.join("rustfy.gresource").to_string_lossy().into_owned());
            }
        }

        paths.push("C:\\Program Files\\rustfy\\rustfy.gresource".to_string());
        paths.push("C:\\ProgramData\\rustfy\\rustfy.gresource".to_string());
    }

    paths
}

fn find_resource_path() -> Option<String> {
    candidate_paths()
        .into_iter()
        .find(|p| Path::new(p).exists())
}

pub fn load_resources() -> Result<(), String> {
    let tried = candidate_paths();
    let path = find_resource_path().ok_or_else(|| {
        format!(
            "Resource file not found. Tried: {:?}\nRun: meson compile -C builddir (or ensure rustfy.gresource is next to the executable on Windows)",
            tried
        )
    })?;

    let resource = gio::Resource::load(&path)
        .map_err(|e| format!("Failed to load resource from {}: {}", path, e))?;

    gio::resources_register(&resource);

    #[cfg(debug_assertions)]
    eprintln!("✓ Loaded resources from: {}", path);

    Ok(())
}
