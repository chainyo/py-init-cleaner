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
