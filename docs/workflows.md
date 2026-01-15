# Example Workflows

## Daily Workflow

```bash
# Start your day - check your assigned issues
linear-cli i list --assignee me

# Pick an issue and start working on it
linear-cli i start LIN-123 --checkout

# ... do your work ...

# Update the issue status when done
linear-cli i update LIN-123 -s Done
```

## Creating and Managing Issues

```bash
# Create a new bug report
linear-cli i create "Login button not working" -t ENG -p 2 -s "Backlog"

# Add a label to the issue
linear-cli bulk label "Bug" -i LIN-456

# Assign it to yourself and start working
linear-cli i start LIN-456 --checkout

# Add a comment with your findings
linear-cli cm create LIN-456 -b "Root cause: Missing null check in auth handler"

# Mark as done when fixed
linear-cli i update LIN-456 -s Done
```

## Git Integration Workflow

```bash
# Start working on an issue (assigns to you, sets "In Progress")
linear-cli i start LIN-123 --checkout

# ... make your changes ...

# Create a PR linked to the issue
linear-cli g pr LIN-123

# Or create a draft PR
linear-cli g pr LIN-123 --draft --web
```

## Project Setup Workflow

```bash
# Compare local code folders with Linear projects
linear-cli sy status

# Create Linear projects for folders that don't exist
linear-cli sy push -t ENG --dry-run    # Preview first
linear-cli sy push -t ENG              # Create projects

# Add labels to organize projects
linear-cli p add-labels PROJECT_ID LABEL1 LABEL2
```
