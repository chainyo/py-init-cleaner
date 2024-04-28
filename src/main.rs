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

#[derive(Debug)]
struct PyImports {
    /// List of attributes imported in the file.
    attributes: Vec<String>,
}

impl PyImports {
    fn from_list_of_strings(lines: Vec<String>) -> Self {
        let mut attributes = Vec::new();
        for line in lines {
            let mut parts = line.split_whitespace();
            // If the line starts with import, then the next part is the module name.
            match parts.next().unwrap() {
                "import" => {
                    // Check if import module as alias. If so, add the alias to the attributes.
                    let module = parts.next().unwrap();
                    if module.contains("as") {
                        let _ = parts.next(); // Skip the "as" part.
                        let alias = parts.next().unwrap();
                        attributes.push(alias.to_string());
                    } else if module.contains(".") {
                        let parts: Vec<&str> = module.split(".").collect();
                        attributes.push(parts[parts.len() - 1].to_string());
                    } else {
                        attributes.push(module.to_string());
                    }
                }
                "from" => {
                    let _ = parts.next(); // Skip the module name.
                    let _ = parts.next(); // Skip the "import" part.
                    for part in parts {
                        if part.ends_with(",") {
                            attributes.push(part[..part.len() - 1].to_string());
                        } else {
                            attributes.push(part.to_string());
                        }
                    }
                }
                _ => {}
            }
        }

        PyImports { attributes }
    }
}

fn prepare_import_list(lines: Vec<String>) -> Vec<String> {
    let mut import_lines = Vec::new();
    let mut multiline_imports = Vec::new();
    let mut in_multiline_import = false;

    for line in lines {
        if line.starts_with("import") {
            import_lines.push(line);
        } else if line.starts_with("from") {
            // Check if line ends with { which means it is a multiline import.
            if line.ends_with("{") {
                multiline_imports.push(line);
                in_multiline_import = true;
            } else {
                import_lines.push(line);
            }
        } else if in_multiline_import {
            if line.ends_with("}") {
                multiline_imports.push(line);
                in_multiline_import = false;
                import_lines.push(multiline_imports.join(" "));
                multiline_imports.clear();
            } else {
                multiline_imports.push(line);
            }
        }
    }
    import_lines
}

fn remove_main_block(lines: Vec<String>) -> Vec<String> {
    let mut cleaned_lines: Vec<String> = Vec::new();
    let mut inside_main_block = false;

    for line in lines {
        if line.trim_start().starts_with("if __name__ == '__main__':")
            || line.trim_start().starts_with("if __name__ == \"__main__\":") {
                inside_main_block = true;
            } else if inside_main_block
                && !line.starts_with("    ")
                && !line.starts_with("\t")
                && !line.trim().is_empty() {
                    inside_main_block = false;
                    cleaned_lines.push(line);
            } else if !inside_main_block {
                cleaned_lines.push(line);
            }
    }

    cleaned_lines
}

fn clean_file(path: &str) -> io::Result<()> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    // Get lines
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    // Run remove_main_block
    let cleaned_lines = remove_main_block(lines);
    // Fix imports
    let prepared_imports = prepare_import_list(cleaned_lines.clone());
    println!("{:?}", prepared_imports);
    let pyimports = PyImports::from_list_of_strings(prepared_imports);
    println!("{:?}", pyimports.attributes);

    let mut file = fs::File::create(path)?;
    for line in cleaned_lines {
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
    use tempfile::Builder;

    #[test]
    fn test_pyimports() {
        let lines = vec![
            "import datetime".to_string(),
            "import pandas as pd".to_string(),
            "import torch.nn".to_string(),
            "import torch.functional.nn as nn".to_string(),
            "import torch.functional.nn".to_string(),
            "from numpy impprt ndarray".to_string(),
            "from polars import DataFrame, Series, String, Int8".to_string(),
        ];
        let pyimports = PyImports::from_list_of_strings(lines);
        assert_eq!(
            pyimports.attributes,
            vec!["datetime", "pd", "nn", "nn", "nn", "ndarray", "DataFrame", "Series", "String", "Int8"]
        );
    }

    #[test]
    fn test_clean_file() {
        let dir = Builder::new().prefix("example").tempdir().unwrap();
        let file_path = dir.path().join("__init__.py");

        let content = r#"
import pandas as pd
from torch import { 
    nn,
    functional as F,
}
# Create a Series
series = pd.Series([1, 2, 3], name="a")

def test_cli(df, series) -> None:
    """Test the CLI commands."""
    df_cli = pd.DataFrameCLI(df)
    assert df_cli == df

    series_cli = pd.SeriesCLI(series)
    assert series_cli == series

# Path: tests/sub-module/test_cli.py
if __name__ == "__main__":
    test_cli()
    print("Hello")

    # Path: tests/sub-module/__init__.py
    a = 1
    b = 2

# Other stuff
import polars as pl
import pandas as pd

# Create a DataFrame
df = pl.DataFrame({
    "a": [1, 2, 3],
    "b": [4, 5, 6],
})
"#;

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "{}", content).unwrap();

    let _path = file_path.to_str().unwrap();
    clean_file(_path).unwrap();

    let modified_content = fs::read_to_string(&file_path).unwrap();
    let expected_content = r#"
import pandas as pd
from torch import { 
    nn,
    functional as F,
}
# Create a Series
series = pd.Series([1, 2, 3], name="a")

def test_cli(df, series) -> None:
    """Test the CLI commands."""
    df_cli = pd.DataFrameCLI(df)
    assert df_cli == df

    series_cli = pd.SeriesCLI(series)
    assert series_cli == series

# Path: tests/sub-module/test_cli.py
# Other stuff
import polars as pl
import pandas as pd

# Create a DataFrame
df = pl.DataFrame({
    "a": [1, 2, 3],
    "b": [4, 5, 6],
})

"#;

    assert_eq!(modified_content, expected_content);
    dir.close().unwrap();
    }
}