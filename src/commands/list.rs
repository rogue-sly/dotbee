use colored::Colorize;
use std::path::Path;
use walkdir::WalkDir;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Available hosts:");

    let root = Path::new("hosts");

    for entry in WalkDir::new(root)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        let relative = path.strip_prefix(root)?;
        let depth = relative.components().count();
        let indent = "  ".repeat(depth);

        let name = relative.file_name().unwrap().to_string_lossy();

        if let 1 = depth {
            println!("{}{}", indent, name.green().bold())
        }
    }

    Ok(())
}
