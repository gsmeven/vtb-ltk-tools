use ltk_mod_project::modpkg::ModpkgFormat;
use ltk_mod_project::{PackageFormat, ProjectPacker};
use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process;

fn print_help() {
    println!("vtb-packer - LeagueToolkit modpkg pack CLI (via ltk_mod_project)");
    println!("Usage:");
    println!("  pack [-c <config_or_dir>] [-f <output_name>] [-o <output_dir>] [--format modpkg]");
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let Some(cmd) = args.next() else {
        print_help();
        return Ok(());
    };

    if cmd == "--help" || cmd == "-h" {
        print_help();
        return Ok(());
    }
    if cmd == "--version" || cmd == "-V" {
        println!("vtb-packer {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    if cmd != "pack" {
        return Err(format!("Unsupported command: {cmd}. Use 'pack' or '--help'.").into());
    }

    let mut config_or_dir: Option<PathBuf> = None;
    let mut output_name: Option<String> = None;
    let mut output_dir: PathBuf = PathBuf::from(".");
    let mut format = String::from("modpkg");

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            "-c" | "--config" | "--config-path" => {
                let val = args.next().ok_or("Missing value for -c/--config")?;
                config_or_dir = Some(PathBuf::from(val));
            }
            "-f" | "--file-name" | "--filename" => {
                let val = args.next().ok_or("Missing value for -f/--file-name")?;
                output_name = Some(val);
            }
            "-o" | "--output" | "--output-dir" => {
                let val = args.next().ok_or("Missing value for -o/--output")?;
                output_dir = PathBuf::from(val);
            }
            "--format" => {
                let val = args.next().ok_or("Missing value for --format")?;
                format = val.to_ascii_lowercase();
            }
            other => {
                return Err(format!("Unknown argument: {other}").into());
            }
        }
    }

    if format != "modpkg" {
        return Err(format!("Unsupported format: {format}. Only 'modpkg' is supported.").into());
    }

    let project_dir: PathBuf = match config_or_dir {
        Some(p) if p.is_file() => {
            let parent = p.parent().unwrap_or_else(|| Path::new("."));
            if parent.as_os_str().is_empty() {
                PathBuf::from(".")
            } else {
                parent.to_path_buf()
            }
        }
        Some(p) => p,
        None => PathBuf::from("."),
    };

    let project_dir_str = project_dir
        .to_str()
        .ok_or("Project directory path is not valid UTF-8")?;

    let packer = ProjectPacker::from_dir(project_dir_str)?;
    fs::create_dir_all(&output_dir)?;

    let file_name = match output_name {
        Some(stem) => {
            if stem.to_ascii_lowercase().ends_with(".modpkg") {
                stem
            } else {
                format!("{stem}.modpkg")
            }
        }
        None => packer
            .project()
            .package_file_name(None, PackageFormat::Modpkg),
    };

    let output_path = output_dir.join(file_name);
    let file = File::create(&output_path)?;
    let report = packer.pack(ModpkgFormat::new(file))?;

    println!(
        "Packed {} (ignored {} files)",
        output_path.display(),
        report.ignored_count()
    );
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
