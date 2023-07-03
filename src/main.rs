use clap::{command, Parser};
use serde::Deserialize;
use serde_json::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
// use walkdir;
// use walkdir::WalkDir;

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
type ParseResult<T> = std::result::Result<T, DoubleError>;
#[derive(Debug, Clone)]
struct DoubleError;


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

// fn compare_dependencies(root: &PackageDependencies, current: &PackageDependencies) -> bool {
//     for (key, value) in current {
//         if root.get(key) != Some(value) {
//             return false;
//         }
//     }
//     true
// }

fn main() {
    let args = Cli::parse();

    let root_package_json = args.path.clone();
    let path = Path::new(&root_package_json);

    let _root_deps = match read_file(path) {
        Some(deps) => Some(deps),
        None => {
            eprintln!("Failed to parse the root package.json on this path: {}", path.display());
            std::process::exit(1);
        }
    };
}

// for entry in WalkDir::new("root") {
//     let entry = entry.unwrap();
//     if entry.file_type().is_file() && entry.file_name().to_str() == Some("package.json") {
//         let current_deps = read_package_json(entry.path().to_str().unwrap()).unwrap();
//         if !compare_dependencies(&root_deps, &current_deps) {
//             println!(
//                 "Alert! Different versions of packages found in {}",
//                 entry.path().display()
//             );
//         }
//     }
// }

// fn main() {
//     let content = std::fs::read_to_string(&args.path).expect("could not read file");

//     // print!("Pattern: {}, Path: {}", args.pattern, args.path.display());

//     // let f = File::open(&args.path).unwrap();
//     // let reader = BufReader::new(f);

//     for line in content.lines() {
//       // let line = line_result.unwrap();
//       if line.contains(&args.pattern) {
//           println!("Found match: {}", line);
//       }
//     }
// }
