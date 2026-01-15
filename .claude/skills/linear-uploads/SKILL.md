---
name: linear-uploads
description: Download attachments and images from Linear issues. Use when fetching screenshots, images, or file attachments from Linear comments or descriptions.
allowed-tools: Bash
---

# Linear Uploads

Download attachments and images from Linear issues using `linear-cli`.

## Download to File

```bash
# Download image/attachment to file
linear-cli up fetch "https://uploads.linear.app/..." -f image.png

# Download screenshot
linear-cli up fetch "https://uploads.linear.app/abc/def/screenshot.png" -f /tmp/screenshot.png
```

## Output to Stdout

```bash
# Pipe to other tools
linear-cli up fetch "https://uploads.linear.app/..." | base64

# Pipe to file
linear-cli up fetch "https://uploads.linear.app/..." > file.png
```

## Finding Upload URLs

Upload URLs are found in:
- Issue descriptions
- Comments (use `linear-cli cm list ISSUE_ID --output json`)

URLs follow pattern: `https://uploads.linear.app/{org}/{upload}/{filename}`

## Tips

- Requires valid Linear API key
- Use `-f` / `--file` to specify output filename
- Without `-f`, outputs raw bytes to stdout (for piping)
