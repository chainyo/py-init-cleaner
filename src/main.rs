/// Entry point for the program that removes the __main__ block from all __init__.py files in a directory.
///
use clap::Parser;
use std::fs;
use std::io::{self, BufRead, Write};
use walkdir::WalkDir;

const IMPORT_PATTERNS: [&str; 3] = [
    // import module
    r"import\s[a-z]+(\.[a-z]+)*(?:\sas\s[a-z]+)?",
    // from mod(.submod...) import submodule (as alias)
    r"from\s[a-z]+(\.[a-z]+)*\simport\s[a-z]+(?:\sas\s[a-z]+)?",
    // from mod(.submod...) import {submod1, submod2, ...} on multiple lines,
    r"from\s+[a-z]+(?:\.[a-z]+)*\s+import\s+\{[^}]*\}",
];

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The directory to scan for __init__.py files.
    #[arg(short, long)]
    dir: String,
}

#[derive(Debug)]
struct AllExports {
    /// List of attributes exported in the __all__ statement
    exports: Vec<String>,
}

impl AllExports {
    fn new(exports: Vec<String>) -> Self {
        Self { exports }
    }

    fn from_imports(imports: PyImports) -> Self {
        let exports = imports.attributes.clone();
        Self::new(exports)
    }

    fn into_string(self) -> String {
        let mut all_statement_str = "__all__ = [\n".to_owned();
        let mut exports = self.exports;
        exports.sort();
        exports.dedup();
        for export in exports {
            all_statement_str.push_str(&format!("    \"{}\",\n", export));
        }
        all_statement_str.push_str("]\n");
        all_statement_str.to_owned()
    }
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
                    if parts.clone().count() > 1 {
                        attributes.push(parts.clone().last().unwrap().to_string());
                    } else if module.contains('.') {
                        let parts: Vec<&str> = module.split('.').collect();
                        attributes.push(parts[parts.len() - 1].to_string());
                    } else {
                        attributes.push(module.to_string());
                    }
                }
                "from" => {
                    let _ = parts.next(); // Skip the module name.
                    let _ = parts.next(); // Skip the "import" part.
                    for part in parts {
                        if part != "{" && part != "}" {
                            if part == "as" {
                                // Remove the last added attribute to add the alias later.
                                attributes.pop();
                            } else if let Some(p) = part.strip_suffix(',') {
                                attributes.push(p.to_string());
                            } else {
                                attributes.push(part.to_string());
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        PyImports { attributes }
    }
}

fn prepare_import_list(path: &str) -> Vec<String> {
    // Use regex to match all the import statements.
    let mut imports = Vec::new();
    let data = fs::read_to_string(path).unwrap();

    let whitespace_re = regex::Regex::new(r"\s+").unwrap();
    for pattern in IMPORT_PATTERNS.iter() {
        let re = regex::Regex::new(pattern).unwrap();
        for cap in re.captures_iter(&data) {
            let import_block = &cap[0];

            let cleaned_import_block = whitespace_re
                .replace_all(import_block, " ")
                .to_string()
                .trim()
                .to_string();

            imports.push(cleaned_import_block);
        }
    }
    imports
}

fn remove_main_and_all_blocks(lines: Vec<String>) -> Vec<String> {
    let mut cleaned_lines: Vec<String> = Vec::new();
    let mut inside_main_block = false;
    let mut inside_all_block = false;

    for line in lines {
        if line.trim_start().starts_with("if __name__ == '__main__':")
            || line
                .trim_start()
                .starts_with("if __name__ == \"__main__\":")
        {
            inside_main_block = true;
        } else if line.trim_start().starts_with("__all__ = [") {
            inside_all_block = true;
        } else if inside_main_block
            && !line.starts_with("    ")
            && !line.starts_with('\t')
            && !line.trim().is_empty()
        {
            inside_main_block = false;
            cleaned_lines.push(line);
        } else if inside_all_block
            && !line.starts_with("    ")
            && !line.starts_with('\t')
            && !line.trim().is_empty()
        {
            inside_all_block = false;
            cleaned_lines.push(line);
        } else if !inside_main_block && !inside_all_block {
            cleaned_lines.push(line);
        }
    }

    cleaned_lines
}

/// Clean the file by applying the following steps:
/// 1. Remove the __main__ block.
/// 2. Fix the imports and the __all__ variable.
/// 3. Write the cleaned lines back to the file.
///
/// # Arguments
/// * `path` - The path to the file to clean.
///
/// # Returns
/// An io::Result<()> indicating the success of the operation.
///
/// # Errors
/// If the file cannot be opened or written to.
///
fn clean_file(path: &str) -> io::Result<()> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    // Get lines
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    // Run remove_main_block
    let lines_no_main_block = remove_main_and_all_blocks(lines);
    // Prepare the __all__ statement from the imports
    let prepared_imports = prepare_import_list(path);
    let python_imports = PyImports::from_list_of_strings(prepared_imports);
    let all_statement_from_imports = AllExports::from_imports(python_imports);

    let mut file = fs::File::create(path)?;
    for line in lines_no_main_block {
        writeln!(file, "{}", line)?;
    }
    // Add the __all__ statement
    writeln!(file, "{}", all_statement_from_imports.into_string())?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    let dir = args.dir;

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
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
            vec![
                "datetime",
                "pd",
                "nn",
                "nn",
                "nn",
                "ndarray",
                "DataFrame",
                "Series",
                "String",
                "Int8"
            ]
        );
    }

    #[test]
    fn test_clean_file() {
        let dir = Builder::new().prefix("example").tempdir().unwrap();
        let file_path = dir.path().join("__init__.py");

        let content = r#"
import pandas as pd
import torch.data
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

__all__ = ["pd", "F"]
"#;

        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "{}", content).unwrap();

        let _path = file_path.to_str().unwrap();
        clean_file(_path).unwrap();

        let modified_content = fs::read_to_string(&file_path).unwrap();
        let expected_content = r#"
import pandas as pd
import torch.data
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

__all__ = [
    "F",
    "data",
    "nn",
    "pd",
    "pl",
]

"#;

        assert_eq!(modified_content, expected_content);
        dir.close().unwrap();
    }
}
