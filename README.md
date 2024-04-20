# Python Init Cleaner

Pre-commit hook to clean your python __init__.py files automatically.

## Installation

Add this to your `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: https://github.com/chainyo/py-init-cleaner
    rev: main
    hooks:
      - id: py-init-cleaner
```