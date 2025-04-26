# logchain

A personal CLI logbook — built for development practice, learning, and tracking work.

---

## What is logchain?

**logchain** is a simple command-line tool designed to track daily notes, ideas, and progress.  
It was created as a personal project to sharpen skills in:

- Building a realistic CLI application
- Handling file I/O, JSON serialization, and project structure in Rust
- Laying the foundation for more advanced topics like server deployment and cloud integration

The tool is fully usable locally, but part of the goal while building it was to also explore deployment considerations — thinking about how real-world software might evolve from a local tool into a networked service.

---

## Why Build It?

Learn Rust and make use of my time while I am deployed with the Military. This is a project I wanted to be passionate about and spend extra time refreshing my practice.
I wanted a project that:

- Reinforces my understanding of Rust by using it in a realistic setting
- Practices working through the full development lifecycle: MVP → iteration → polish
- Allows space to grow into backend development and cloud deployment
- Results in something practical that I could actually use personally across different projects

Rather than just following tutorials, logchain is the result of working through design, problem solving, and refactoring based on real use cases.

---

## How It Works (MVP)

| Command                  | Description                 |
| :----------------------- | :-------------------------- |
| `logchain new "message"` | Create a new log entry      |
| `logchain list`          | Display today's log entries |

Logs are automatically saved into a `logs/` directory as JSON files organized by date.

Example log structure:

```json
{
  "date": "2025-04-25",
  "logs": [
    {
      "time": "08:50",
      "message": "Started CLI project",
      "tags": []
    }
  ]
}
```

---

## Future Plans

- Add support for tagging and filtering log entries
- Build a basic terminal UI (TUI) interface for browsing and searching
- Implement optional remote server sync and authentication
- Explore Dockerization and deploying a small API backend
- Continue improving usability, configuration, and portability

This project is intentionally left open-ended to encourage continuous learning and feature exploration.

---
