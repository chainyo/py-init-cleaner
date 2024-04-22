"""This file is used for testing the CLI commands."""

import polars

# Create a DataFrame
df = polars.DataFrame({
    "a": [1, 2, 3],
    "b": [4, 5, 6],
    "c": [7, 8, 9]
})

# Create a Series
series = polars.Series("a", [1, 2, 3])


def test_cli():
    """Test the CLI commands."""
    # Test the DataFrame CLI command
    df_cli = polars.DataFrameCLI(df)
    assert df_cli == df

    # Test the Series CLI command
    series_cli = polars.SeriesCLI(series)
    assert series_cli == series


if __name__ == "__main__":
    test_cli()
