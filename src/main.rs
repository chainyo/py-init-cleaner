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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::Builder

    #[test]
    fn test_clean_file() {
        let dir = Builder::new().prefix("example").tempdir().unwrap();
        let file_path = dir.path().join("__init__.py");

        let content = r#"""
"""Another testing file"""

import pandas as pd

# Create a DataFrame
df = pd.DataFrame({
    "a": [1, 2, 3],
    "b": [4, 5, 6],
    "c": [7, 8, 9]
})

# Create a Series
series = pd.Series([1, 2, 3], name="a")

def test_cli():
    """Test the CLI commands."""
    # Test the DataFrame CLI command
    df_cli = pd.DataFrameCLI(df)
    assert df_cli == df

    # Test the Series CLI command
    series_cli = pd.SeriesCLI(series)
    assert series_cli == series


# Path: tests/sub-module/test_cli.py
if __name__ == "__main__":
    test_cli()
    print("Hello")

    # Path: tests/sub-module/__init__.py
    a = 1
    b = 2
    c = a + b

# Other stuff
import polars as pl
import pandas as pd

# Create a DataFrame
df = pl.DataFrame({
    "a": [1, 2, 3],
    "b": [4, 5, 6],
    "c": [7, 8, 9]
})
"""#;

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "{}", content).unwrap();

    let _path = file_path.to_str().unwrap();
    clean_file(_path).unwrap();

    let modified_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(modified_content, r#"""
"""Another testing file"""

import pandas as pd

# Create a DataFrame
df = pd.DataFrame({
    "a": [1, 2, 3],
    "b": [4, 5, 6],
    "c": [7, 8, 9]
})

# Create a Series
series = pd.Series([1, 2, 3], name="a")

def test_cli():
    """Test the CLI commands."""
    # Test the DataFrame CLI command
    df_cli = pd.DataFrameCLI(df)
    assert df_cli == df

    # Test the Series CLI command
    series_cli = pd.SeriesCLI(series)
    assert series_cli == series

# Other stuff
import polars as pl
import pandas as pd

# Create a DataFrame
df = pl.DataFrame({
    "a": [1, 2, 3],
    "b": [4, 5, 6],
    "c": [7, 8, 9]
})
"""#);

    dir.close().unwrap();
    }
}