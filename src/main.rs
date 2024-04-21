use regex::Regex;
use std::fs;
use std::io::{self, Write};
use walkdir::WalkDir;

fn clean_file(path: &str) -> io::Result<()> {
    let data = fs::read_to_string(path)?;
    // Updated regex to match the if statement and any indented lines that follow
    let re = Regex::new(r#"if __name__ == ['\"]__main__['\"]:\s*\n( +.*\n)*"#).unwrap();
    let cleaned = re.replace_all(&data, "");

    fs::write(path, cleaned.as_ref())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let target_directory = if args.len() > 1 { &args[1] } else { "." };

    for entry in WalkDir::new(target_directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_name().to_string_lossy() == "__init__.py")
    {
        let path = entry.path().to_str().unwrap();
        if let Err(e) = clean_file(path) {
            writeln!(io::stderr(), "Error processing {}: {}", path, e)?;
            std::process::exit(1);
        }
    }

    Ok(())
}
