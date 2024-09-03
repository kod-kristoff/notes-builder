use std::{
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use env_logger::Env;
use eyre::WrapErr;
use notes_builder::NoteGenerator;
use walkdir::WalkDir;

fn main() {
    setup_logging();
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(2);
    }
}

fn setup_logging() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
}

fn run() -> eyre::Result<()> {
    log::debug!("Starting to compile notes");
    let mut args = env::args();
    let input_path = if let Some(path) = args.nth(1) {
        PathBuf::from(&path)
    } else {
        PathBuf::from(".")
    };
    let output_path = Path::new("./output");
    let static_path = input_path.join("static");
    let content_path = input_path.join("content");
    let out_static_dir = output_path.join("static");

    log::debug!("output_path = {}", output_path.display());

    log::info!("Copying static to output");
    copy_dir_all(&static_path, &out_static_dir).wrap_err_with(|| {
        format!(
            "Error when copying {} to {}",
            static_path.display(),
            out_static_dir.display()
        )
    })?;

    log::info!("Create Notes from content='{}'", content_path.display());
    build_notes(&content_path, &output_path)?;
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> eyre::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in
        fs::read_dir(src).wrap_err_with(|| format!("Failed read dir '{}'", src.display()))?
    {
        let entry = entry?;
        let ty = entry.file_type()?;
        let entry_path = entry.path();
        let dst_entry_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry_path, &dst_entry_path)?;
        } else {
            fs::copy(&entry_path, &dst_entry_path).wrap_err_with(|| {
                format!(
                    "Failed copy '{}' to '{}'",
                    entry_path.display(),
                    dst_entry_path.display()
                )
            })?;
        }
    }
    Ok(())
}

pub fn build_notes(src: &Path, dst: &Path) -> eyre::Result<()> {
    let mut note_generator = NoteGenerator::new();
    fs::create_dir_all(&dst)?;
    for entry in WalkDir::new(src) {
        let entry = entry?;

        let from = entry.path();
        let to = dst.join(from.strip_prefix(src)?);
        if entry.file_type().is_dir() {
            log::debug!("creating dir {}", to.display());
            fs::create_dir_all(&to)?;
        } else {
            match from.extension().and_then(OsStr::to_str) {
                Some("md") => {
                    log::debug!("generate note from {}", from.display());
                    note_generator.create_note(&from, &to);
                    // fs::copy(&from, &to).wrap_err_with(|| {
                    //     format!("Failed copy '{}' to '{}'", from.display(), to.display())
                    // })?;
                    dbg!(&note_generator);
                }
                _ => {
                    log::debug!("copy {} => {}", from.display(), to.display());
                    fs::copy(&from, &to).wrap_err_with(|| {
                        format!("Failed copy '{}' to '{}'", from.display(), to.display())
                    })?;
                }
            }
        }
    }
    Ok(())
}
