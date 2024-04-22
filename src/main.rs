
use clap::Parser;
use regex::Regex;
use std::fs;
use std::io;
use walkdir::WalkDir;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The directory to scan for __init__.py files.
    #[arg(short, long)]
    dir: String,
}

fn clean_file(path: &str) -> io::Result<()> {
    let data = fs::read_to_string(path)?;

    let re = Regex::new(r#"if __name__ == ['\"]__main__['\"]:\s*\n( +.*\n)*"#).unwrap();
    let cleaned = re.replace_all(&data, "");

    fs::write(path, cleaned.as_ref())?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    let dir = args.dir;

    for entry in WalkDir::new(&dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().display().to_string();
        if path.ends_with("__init__.py") {
            clean_file(&path).unwrap();
        }
    }
}
