# Python Init Cleaner

Pre-commit hook to clean your python __init__.py files automatically.

## Installation

Add this to your `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: https://github.com/chainyo/py-init-cleaner
    rev: "v2.0.0"
    hooks:
      - id: py-init-cleaner
        args: ["--dir", "src"] # 👈 Change the 'src' argument to the name of the folder you want to check
```