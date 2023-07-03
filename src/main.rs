use clap::{command, Parser};
use serde::Deserialize;
use serde_json::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version)]
#[command(about = "Does awesome things", long_about = None)]
struct Cli {
    #[arg(long, short)]
    base: std::path::PathBuf,

    #[arg(long, short, num_args = 1..)]
    paths: Vec<std::path::PathBuf>,

    #[arg(long, short)]
    test: bool,
}

#[derive(Debug, Deserialize)]
struct PackageJson {
    dependencies: HashMap<String, String>,
}

type PackageDependencies = HashMap<String, String>;

fn parse_package_json(file_content: &str) -> Result<PackageJson> {
    let json_value: PackageJson = serde_json::from_str(file_content)?;
    Ok(json_value)
}

fn read_file(file_path: &Path) -> Option<PackageDependencies> {
    let file_content = fs::read_to_string(file_path).ok()?;
    let json_value = parse_package_json(&file_content).ok()?;
    let deps = json_value.dependencies.clone();
    Some(deps)
}

struct Mismatch {
    key: String,
    root_value: String,
    current_value: String,
}

fn compare_dependencies(
    root: &PackageDependencies,
    current: &PackageDependencies,
) -> Vec<Mismatch> {
    let mut mismatches = Vec::new();

    for (key, value) in current {
        match root.get(key) {
            Some(val) => {
                if val != value {
                    mismatches.push(Mismatch {
                        key: key.clone(),
                        root_value: val.clone(),
                        current_value: value.clone(),
                    });
                }
            }
            None => (),
        }
    }
    mismatches
}

fn find_child_package_json(path: &Path, root_details: &PackageDependencies) -> Option<String> {
    let current_deps = read_file(path)?;
    let mismatches = compare_dependencies(&root_details, &current_deps);

    if !mismatches.is_empty() {
        let result = mismatches
            .iter()
            .map(|mismatch| {
                format!(
                    "Expected: {} but found: {} in {}",
                    mismatch.root_value, mismatch.current_value, mismatch.key
                )
            })
            .collect::<Vec<_>>();

        Some(result.join("\n"))
    } else {
        None
    }
}

fn main() {
    let args = Cli::parse();

    let base_path = args.base;

    let root_package: &std::path::PathBuf = &base_path.join("package.json");
    let root_package_path = root_package.as_path();

    let to_check: Vec<PathBuf> = args
        .paths
        .iter()
        .map(|path_buf| base_path.join(path_buf).clone())
        .collect();

    println!("Here are the paths to check: {:?}", to_check);

    let root_deps = match read_file(root_package_path) {
        Some(deps) => deps,
        None => {
            eprintln!(
                "Failed to parse the root package.json on this path: {}",
                root_package_path.display()
            );
            std::process::exit(1);
        }
    };

    for path in to_check {
        match find_child_package_json(path.as_path(), &root_deps) {
            Some(message) => {
                println!("{}", message);
                // std::process::exit(1);
            }
            None => (),
        }
    }
}
