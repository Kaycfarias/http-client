# 📂 History Storage

## Where is history saved?

The HTTP Client automatically saves your request history to a JSON file on your local disk. The location depends on your operating system:

### Linux

```
~/.config/http-client/history.json
```

### macOS

```
~/Library/Application Support/http-client/history.json
```

### Windows

```
%APPDATA%\http-client\history.json
```

## File Format

The history is stored as a JSON array with the following structure:

```json
[
  {
    "request": {
      "method": "GET",
      "url": "https://api.example.com/users",
      "headers": [
        {
          "key": "Content-Type",
          "value": "application/json",
          "enabled": true
        }
      ],
      "query_params": [],
      "body": "",
      "body_type": "None",
      "timeout_ms": 30000
    },
    "response": {
      "status": 200,
      "status_text": "OK",
      "body": "{\"users\": [...]}",
      "headers": {
        "content-type": "application/json",
        "content-length": "1234"
      },
      "duration_ms": 156
    },
    "timestamp": 1738521600
  }
]
```

## Features

- ✅ **Auto-save**: History is saved automatically after each request
- ✅ **Auto-load**: History is loaded when the application starts
- ✅ **Size limit**: Only the last 50 requests are kept
- ✅ **Clear history**: Clearing history also deletes the file content
- ✅ **Persistent across restarts**: Your history survives app restarts

## Manual Management

### View the history file

**Linux/macOS:**

```bash
cat ~/.config/http-client/history.json | jq
```

**Windows (PowerShell):**

```powershell
Get-Content $env:APPDATA\http-client\history.json | ConvertFrom-Json
```

### Backup your history

**Linux/macOS:**

```bash
cp ~/.config/http-client/history.json ~/backup-history.json
```

**Windows:**

```powershell
Copy-Item $env:APPDATA\http-client\history.json backup-history.json
```

### Clear history manually

**Linux/macOS:**

```bash
rm ~/.config/http-client/history.json
```

**Windows:**

```powershell
Remove-Item $env:APPDATA\http-client\history.json
```

## Technical Details

- **Format**: JSON (UTF-8)
- **Serialization**: Uses `serde_json` for safe serialization
- **Directory discovery**: Uses `dirs` crate for cross-platform paths
- **Max items**: 50 (oldest items are automatically removed)
- **File size**: Typically 50-500KB depending on response sizes

## Privacy & Security

⚠️ **Important**: The history file is stored in **plain text** on your disk. This means:

- ✅ Request URLs are saved
- ✅ Headers (including auth tokens) are saved
- ✅ Request/response bodies are saved
- ⚠️ **Do not share** the history file if it contains sensitive data
- 💡 Use "Clear History" before sharing your computer

## Troubleshooting

### History not loading?

1. Check if the file exists:

   ```bash
   ls -la ~/.config/http-client/
   ```

2. Check file permissions:

   ```bash
   chmod 644 ~/.config/http-client/history.json
   ```

3. Verify JSON is valid:
   ```bash
   cat ~/.config/http-client/history.json | jq
   ```

### History not saving?

1. Check directory permissions:

   ```bash
   mkdir -p ~/.config/http-client
   chmod 755 ~/.config/http-client
   ```

2. Check disk space:
   ```bash
   df -h ~
   ```

### Corrupted history file?

If the app crashes on startup due to a corrupted history file:

```bash
# Delete the corrupted file
rm ~/.config/http-client/history.json

# The app will create a new one on next request
```

---

**Pro Tip**: You can share history files between computers by copying the `history.json` file! 📤
