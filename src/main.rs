use regex::Regex;
use std::fs;
use std::io::{self, Write};

fn clean_file(path: &str) -> io::Result<()> {
    let data = fs::read_to_string(path)?;
    let re = Regex::new(r#"if __name__ == ['\"]__main__['\"]:\s*"#).unwrap();
    let cleaned = re.replace_all(&data, "");

    fs::write(path, cleaned.as_ref())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    for file in &args[1..] {
        if let Err(e) = clean_file(file) {
            writeln!(io::stderr(), "Error processing {}: {}", file, e).unwrap();
            std::process::exit(1);
        }
    }
}
