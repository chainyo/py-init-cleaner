/// Entry point for the program that removes the __main__ block from all __init__.py files in a directory.
///
use clap::Parser;
use std::fs;
use std::io::{self, BufRead, Write};
use walkdir::WalkDir;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The directory to scan for __init__.py files.
    #[arg(short, long)]
    dir: String,
}

fn clean_file(path: &str) -> io::Result<()> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    let mut inside_main_block = false;

    for line_result in reader.lines() {
        let line = line_result?;
        if line.trim_start().starts_with("if __name__ == '__main__':")
            || line.trim_start().starts_with("if __name__ == \"__main__\":") {
                inside_main_block = true;
            } else if inside_main_block
                && !line.starts_with("    ")
                && !line.starts_with("\t")
                && !line.trim().is_empty() {
                    inside_main_block = false;
                    lines.push(line);
            } else if !inside_main_block {
                lines.push(line);
            }
    }

    let mut file = fs::File::create(path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

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
