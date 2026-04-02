//pub mod applications;
pub mod css;
//pub mod web_browser;
//pub mod workspaces;

pub fn launch_app(app_cmd: &str) {
    let _ = std::process::Command::new("setsid")
        .arg("sh")
        .arg("-c")
        .arg(app_cmd)
        .current_dir(std::env::var("HOME").unwrap())
        .spawn();
}
