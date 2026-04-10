use crate::enums::AppInfo;
use gtk::Image;
use std::fs;
use std::path::Path;

pub fn list_applications() -> Vec<AppInfo> {
    let mut apps = vec![];
    let mut paths = vec![
        "/usr/share/applications".to_string(),
        "/usr/local/share/applications".to_string(),
    ];

    if let Ok(home) = std::env::var("HOME") {
        paths.push(format!("{}/.local/share/applications", home));
    }

    if let Ok(xdg_data_dirs) = std::env::var("XDG_DATA_DIRS") {
        for dir in xdg_data_dirs.split(':') {
            let path = format!("{}/applications", dir);
            if !paths.contains(&path) {
                paths.push(path);
            }
        }
    }

    for path in paths {
        let entries = match fs::read_dir(path) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "desktop") {
                continue;
            }

            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            if let Some(app) = parse_desktop_file(&content) {
                apps.push(app);
            }
        }
    }

    apps
}

fn parse_desktop_file(content: &str) -> Option<AppInfo> {
    let mut name = None;
    let mut exec = None;
    let mut icon = None;

    for line in content.lines() {
        if name.is_none()
            && let Some(stripped) = line.strip_prefix("Name=")
            && !stripped.contains("URL Handler")
        {
            name = Some(stripped.to_string());
        }

        if exec.is_none()
            && let Some(stripped) = line.strip_prefix("Exec=")
        {
            let exec_clean = stripped
                .split_whitespace()
                .filter(|s| !s.starts_with('%'))
                .collect::<Vec<_>>()
                .join(" ");
            exec = Some(exec_clean);
        }
        if icon.is_none()
            && let Some(stripped) = line.strip_prefix("Icon=")
        {
            icon = Some(stripped.to_string());
        }

        if name.is_some() && exec.is_some() && icon.is_some() {
            break;
        }
    }

    Some(AppInfo {
        name: name?,
        exec: exec?,
        icon,
    })
}

pub fn load_icon(icon_name: &str, size: i32) -> Image {
    let image = if icon_name.contains('/')
        || icon_name.ends_with(".png")
        || icon_name.ends_with(".svg")
        || icon_name.ends_with(".xpm")
    {
        if Path::new(icon_name).exists() {
            Image::from_file(icon_name)
        } else {
            Image::new()
        }
    } else {
        Image::from_icon_name(icon_name)
    };

    image.set_pixel_size(size);
    image
}
