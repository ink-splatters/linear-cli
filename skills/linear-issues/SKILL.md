---
name: linear-issues
description: Manage Linear issues - list, create, update, start/stop work. Use when working with Linear issues, viewing tasks, creating bugs, or updating issue status.
allowed-tools: Bash
---

# Linear Issues

Manage Linear.app issues using `linear-cli`.

## List Issues

```bash
linear-cli i list                          # All issues
linear-cli i list -t ENG                   # Filter by team
linear-cli i list -s "In Progress"         # Filter by status
linear-cli i list --assignee me            # My issues only
linear-cli i list --output json --compact  # JSON for parsing
linear-cli i list --output json --fields identifier,title,state.name
```

## Get Issue

```bash
linear-cli i get LIN-123                   # Single issue
linear-cli i get LIN-1 LIN-2 LIN-3         # Multiple issues
linear-cli i get LIN-123 --output json     # JSON output
```

## Create Issue

```bash
# Basic
linear-cli i create "Title" -t TEAM

# With options
linear-cli i create "Bug" -t ENG -p 1           # Priority 1=urgent
linear-cli i create "Task" -t ENG -a me         # Assign to self
linear-cli i create "Fix" -t ENG -l bug -l urgent  # With labels
linear-cli i create "Due" -t ENG --due tomorrow # Due date

# Agent patterns
linear-cli i create "Bug" -t ENG --id-only      # Return only ID
linear-cli i create "Test" -t ENG --dry-run     # Preview only
cat desc.md | linear-cli i create "Title" -t ENG -d -  # Pipe description
```

## Update Issue

```bash
linear-cli i update LIN-123 -s Done        # Change status
linear-cli i update LIN-123 -p 2           # Priority (2=high)
linear-cli i update LIN-123 -a "John"      # Assign to user
linear-cli i update LIN-123 -l bug         # Add label
linear-cli i update LIN-123 --due +3d      # Due in 3 days
linear-cli i update LIN-123 --id-only      # Return only ID
```

## Start/Stop Work

```bash
# Start: assigns to you, sets "In Progress", creates git branch
linear-cli i start LIN-123 --checkout

# Stop: unassigns, resets status
linear-cli i stop LIN-123
```

## Comments

```bash
linear-cli cm list LIN-123                 # List comments
linear-cli cm list LIN-123 --output json   # JSON output
linear-cli cm create LIN-123 -b "Fixed"    # Add comment
```

## Context (Current Issue)

```bash
linear-cli context                         # Get issue from git branch
linear-cli context --output json           # JSON output
```

## Agent Flags

| Flag | Purpose |
|------|---------|
| `--output json` | JSON output |
| `--compact` | No formatting |
| `--fields a,b` | Select fields |
| `--quiet` | No decoration |
| `--id-only` | Return ID only |
| `--dry-run` | Preview only |

## Exit Codes

- `0` = Success
- `1` = Error
- `2` = Not found
- `3` = Auth error
- `4` = Rate limited

## Priority Values

`1`=Urgent, `2`=High, `3`=Normal, `4`=Low, `0`=None

## Due Date Shortcuts

`today`, `tomorrow`, `+3d`, `+2w`, `monday`, `eow`, `eom`, `2024-03-15`
