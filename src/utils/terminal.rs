use std::{env, process::Command};

pub fn open_terminal(cmd: &str, terminal: Option<&str>) {
    let shell = env::var("SHELL").unwrap_or("/bin/bash".into());
    let cmd = format!("{}; exec {}", cmd, &shell);
    let home = env::var("HOME").unwrap_or("~".into());

    if let Some(term) = terminal {
        let _ = Command::new(term)
            .args(["-e", &shell, "-c", &cmd])
            .current_dir(&home)
            .spawn();
    }

    let terminals = vec![
        ("kitty", vec![&shell, "-c", &cmd]),
        ("alacritty", vec!["-e", &shell, "-c", &cmd]),
        ("gnome-terminal", vec!["--", &shell, "-c", &cmd]),
    ];

    for (term, args) in terminals {
        if Command::new(term)
            .args(&args)
            .current_dir(&home)
            .spawn()
            .is_ok()
        {
            return;
        }
    }

    eprintln!("No terminal found");
}
