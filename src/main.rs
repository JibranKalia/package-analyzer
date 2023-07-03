use clap::{command, Parser};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;

// cd <monorepo_path> && git ls-files | grep "package.json$" > paths.txt
// cat paths.txt | xargs cargo run -- --base <monorepo_path> --paths

#[derive(Parser)]
#[command(version)]
#[command(about = "Compares package.json in monorepo against the base and identifies any discrepancies", long_about = None)]
struct Cli {
    /// Absolute path to the base directory
    #[arg(long, short)]
    base: std::path::PathBuf,

    /// Relative paths to the package.json files to check
    #[arg(long, short, num_args = 1..)]
    paths: Vec<std::path::PathBuf>,
}

#[derive(Debug, Deserialize)]
struct PackageJson {
    dependencies: Vec<PackageDependencies>,
}

impl PackageJson {
    fn parse_package_json(file_content: &str) -> Result<Self, anyhow::Error> {
        let json_value: PackageJson = serde_json::from_str(file_content)?;
        Ok(json_value)
    }
}

#[derive(Debug, Clone, Deserialize)]
struct PackageDependencies {
    name: String,
    version: String,
}

impl PackageDependencies {
    fn strip_caret(&self) -> Self {
        match self.version.strip_prefix("^") {
            Some(stripped) => Self {
                name: self.name.clone(),
                version: stripped.to_string(),
            },
            None => self.clone(),
        }
    }
}

fn read_file(file_path: &Path) -> Option<PackageJson> {
    let file_content = fs::read_to_string(file_path).ok()?;
    let json_value = match PackageJson::parse_package_json(&file_content) {
        Ok(value) => value,
        Err(_) => {
            panic!(
                "Failed to parse the package.json on this path: {}",
                file_path.display()
            );
        }
    };

    json_value.dependencies.iter().for_each(|p| {
        p.strip_caret();
    });

    Some(json_value)
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

fn process_each_package_json(
    package_path: &Path,
    base_path: &Path,
    root_details: &PackageDependencies,
) -> Option<String> {
    let current_deps = read_file(package_path)?;
    let mismatches = compare_dependencies(&root_details, &current_deps);

    if !mismatches.is_empty() {
        let result: Vec<String> = mismatches
            .iter()
            .map(|mismatch| {
                let path_to_display = package_path.strip_prefix(base_path).unwrap().display();
                format!(
                    "Error in `{}` for package: {}. Expected: {} but got: {}",
                    mismatch.key, path_to_display, mismatch.root_value, mismatch.current_value
                )
            })
            .collect();

        Some(result.join("\n"))
    } else {
        None
    }
}

fn main() {
    let args = Cli::parse();

    let base_path = args.base;

    let root_package = base_path.join("package.json");

    let to_check: Vec<PathBuf> = args
        .paths
        .iter()
        .map(|path_buf| base_path.join(path_buf).clone())
        .collect();

    let root_deps = match read_file(root_package.as_path()) {
        Some(deps) => deps,
        None => {
            eprintln!(
                "Failed to parse the root package.json on this path: {}",
                root_package.display()
            );
            std::process::exit(1);
        }
    };

    for path in to_check {
        match process_each_package_json(path.as_path(), &base_path, &root_deps) {
            Some(message) => {
                println!("{}", message);
            }
            None => (),
        }
    }
}
