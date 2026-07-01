use std::fs;

use anyhow::Context;
use constants::{
    INSTALL_BIN_DIR, PROMPT_DEFAULT, PROMPT_SECTION, PROMPT_SECTION_PROMPT,
    PROMPT_SECTION_PROMPT_COLOR_KEY,
};
use helpers::io::expand_path;
// use helpers::io::home_dir;

// const PATH_EXPORT: &str = r#"export PATH="$HOME/.local/bin:$PATH""#;

fn main() -> anyhow::Result<()> {
    // let bin_path = resolve_bin_path()?;
    let dest = expand_path(INSTALL_BIN_DIR);

    // install_binary(&bin_path, &dest)?;
    create_default_config()?;
    // ensure_path_in_shell_profile()?;

    println!("Installed dsh to {}", dest.display());
    println!(
        "Config created at {}",
        expand_path(constants::CONFIG_PATH).display()
    );
    println!();
    println!("Restart your shell or run:");
    println!("  source ~/.bashrc   # bash");
    println!("  source ~/.zshrc    # zsh");
    println!();
    println!("Then start the shell with: dsh");

    Ok(())
}

// fn resolve_bin_path() -> anyhow::Result<PathBuf> {
//     let args: Vec<String> = env::args().collect();
//     if let Some(path) = args.get(1) {
//         let path = PathBuf::from(path);
//         if path.is_file() {
//             return Ok(path);
//         }
//         bail!("binary not found: {}", path.display());
//     }
//
//     if let Ok(path) = env::var("DSH_BIN") {
//         let path = PathBuf::from(&path);
//         if path.is_file() {
//             return Ok(path);
//         }
//         bail!("DSH_BIN does not point to a file: {}", path.display());
//     }
//
//     if let Ok(exe) = env::current_exe() {
//         if let Some(dir) = exe.parent() {
//             for name in ["core", "dsh"] {
//                 let candidate = dir.join(name);
//                 if candidate.is_file() {
//                     return Ok(candidate);
//                 }
//             }
//         }
//     }
//
//     let workspace_target = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/release/dsh");
//     if workspace_target.is_file() {
//         return Ok(workspace_target);
//     }
//
//     let debug_target = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/debug/dsh");
//     if debug_target.is_file() {
//         return Ok(debug_target);
//     }
//
//     bail!(
//         "could not find dsh binary.\n\
//          Build it first:  cargo build --release -p core\n\
//          Or pass the path:  install /path/to/dsh"
//     );
// }

// fn install_binary(src: &Path, dest: &Path) -> anyhow::Result<()> {
//     if let Some(parent) = dest.parent() {
//         fs::create_dir_all(parent)
//             .with_context(|| format!("failed to create {}", parent.display()))?;
//     }
//
//     fs::copy(src, dest)
//         .with_context(|| format!("failed to copy {} to {}", src.display(), dest.display()))?;
//
//     let mut perms = fs::metadata(dest)?.permissions();
//     perms.set_mode(0o755);
//     fs::set_permissions(dest, perms)?;
//
//     Ok(())
// }

fn create_default_config() -> anyhow::Result<()> {
    let config_path = expand_path(constants::CONFIG_PATH);

    if config_path.exists() {
        println!(
            "Config already exists at {}, skipping",
            config_path.display()
        );
        return Ok(());
    }

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }

    let contents = format!(
        "[{PROMPT_SECTION}]\n\
         {PROMPT_SECTION_PROMPT} = \"{PROMPT_DEFAULT}\"\n\
         {PROMPT_SECTION_PROMPT_COLOR_KEY} = \"White\"\n"
    );

    fs::write(&config_path, contents)
        .with_context(|| format!("failed to write {}", config_path.display()))?;

    Ok(())
}

// fn ensure_path_in_shell_profile() -> anyhow::Result<()> {
//     let profile = shell_profile_path()?;
//     let contents = fs::read_to_string(&profile).unwrap_or_default();
//
//     if contents.contains(".local/bin") {
//         println!("~/.local/bin already in {}", profile.display());
//         return Ok(());
//     }
//
//     let mut file = fs::OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open(&profile)
//         .with_context(|| format!("failed to open {}", profile.display()))?;
//
//     if !contents.is_empty() && !contents.ends_with('\n') {
//         writeln!(file)?;
//     }
//
//     writeln!(file, "\n# Added by dsh install")?;
//     writeln!(file, "{PATH_EXPORT}")?;
//
//     println!("Added ~/.local/bin to PATH in {}", profile.display());
//
//     Ok(())
// }

// fn shell_profile_path() -> anyhow::Result<PathBuf> {
//     let home = PathBuf::from(home_dir().context("HOME is not set")?);
//
//     let shell = env::var("SHELL").unwrap_or_default();
//     let profile = if shell.contains("zsh") {
//         home.join(".zshrc")
//     } else if shell.contains("bash") {
//         home.join(".bashrc")
//     } else {
//         home.join(".profile")
//     };
//
//     Ok(profile)
// }
