# logchain

A personal CLI logbook — My deployment project to pass time (I wish I had started it earlier), learn a bit of rust, and refresh programming/computer science fundamentals. This project is focused on practical skill-building, not just following tutorials — using real project structure, data handling, and eventually expanding toward backend and cloud deployment. I am finding that Rust is an amazing language to use and learn.

**logchain** is a simple command-line tool designed to track daily notes, ideas, and progress.  
It was created as a personal project to sharpen skills in:

- Building a realistic CLI application
- Handling file I/O, JSON serialization, and project structure in Rust
- Future goal of adding server implementation to get better experience preparing for cloud deployment.

## What it does

**_This is the intended end state with logchain as a CLI tool_**

| Command                  | Description                 |
| :----------------------- | :-------------------------- |
| `logchain new "message"` | Create a new log entry      |
| `logchain list`          | Display today's log entries |
| `logchain tag "tag"`     | Add tag to newest log entry |

A logchain folder is automatically created. In it will be the logs folder that will contain daily logs in json files.

### Current Build Examples

```bash
logchain new "Started CLI project"
```

Creates a log file like

```json
{
  "date": "2025-04-26",
  "logs": [
    {
      "time": "08:50:16",
      "message": "Started CLI project",
      "tags": []
    }
  ]
}
```

To add a tag to the last entry:

```bash
logchain tag "Feature"
```

To view logs:

```bash
$ logchain list
Daily log for <C:\Example\Repo>: 2025-04-26
------------------------------------------------------------------------------------------------------------------------
[08:50:16] Started CLI project ["Feature"]
```
