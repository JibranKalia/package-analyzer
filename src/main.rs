use clap::{command, Parser};
use serde::Deserialize;
use serde_json::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(version)]
#[command(about = "Does awesome things", long_about = None)]
struct Cli {
    #[arg(long, short)]
    path: std::path::PathBuf,
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

fn compare_dependencies(root: &PackageDependencies, current: &PackageDependencies) -> Option<String> {
    for (key, value) in current {
        if root.get(key) != Some(value) {
          return Some(key.clone());
        }
    }
    None
}


fn find_child_package_json(path: &Path, root: &PackageDependencies) -> Option<String>{
    let root_dir = path.parent().unwrap();
    for entry in WalkDir::new(root_dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() && entry.file_name().to_str() == Some("package.json") {
            println!("Checking {}", entry.path().display());
            let current_deps = read_file(entry.path()).unwrap();
            match compare_dependencies(&root, &current_deps) {
              Some(mismatch) => println!("Mismatch: {}", mismatch),
              None => println!("No mismatch"),
            }
            return Some("test".to_string());
        }
    }
    None

}


fn main() {
    let args = Cli::parse();

    let root_package_json = args.path.clone();
    let path = Path::new(&root_package_json);

    let root_deps = match read_file(path) {
        Some(deps) => deps,
        None => {
            eprintln!("Failed to parse the root package.json on this path: {}", path.display());
            std::process::exit(1);
        }
    };

    find_child_package_json(path, &root_deps);
}