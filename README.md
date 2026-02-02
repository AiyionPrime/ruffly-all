# that's ruffly-all

## Answering whats necessary to migrate your project to ruff right now.

### A glorified version of this oneliner

```bash
ruff check --isolated --select 'ALL' --output-format concise 2>/dev/null | cut -d ' ' -f2 | grep '^[A-Za-z]' | sort | tr -d '0123456789' | uniq"
```
