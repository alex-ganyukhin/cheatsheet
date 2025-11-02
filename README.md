# Cheat Sheet

A simple and fast **Rust command-line app** that helps you create, manage, and search your own cheat sheets.
Perfect for developers who need quick reminders of commands, syntax, or snippets.

---

## 🚀 Overview

**Rust Cheat Sheet** lets you save useful commands (e.g., `bash`, `git`, `docker`, etc.)
and look them up later using simple fuzzy search or even natural language.

---

## ✨ Features

### **Milestone 1 — Base**
> Focus: user-defined commands and direct lookups

- [ ] Create and edit your own cheat sheet entries
- [ ] Fuzzy search for a command (e.g. `cheatsheet chmod`)
- [ ] Load from / save to a local file (`~/.cheatsheet.toml`)
- [ ] Display matching entries in the terminal
- [ ] Basic CLI help and subcommands via `clap`

**Example**
```bash
cheatsheet add "chmod" "Change file permissions"
cheatsheet chmod
```

---

### **Milestone 2 — Extended**
> Focus: interactive search (like `Ctrl+R`)

- [ ] Launch an interactive terminal search UI (`cheatsheet`)
- [ ] Scroll through matches, filter as you type
- [ ] Select entry with Enter, copy or print it
- [ ] Maintain search history and usage frequency
- [ ] Shell integration (bash/zsh/fish)

**Example**
```bash
cheatsheet
# Opens interactive fuzzy search
```

---

### **Milestone 3 — AI Search**
> Focus: natural-language search

- [ ] Accept free-form queries like “show largest files”
- [ ] Match them against cheat sheet entries using embeddings
- [ ] Return best-fitting command with explanation
- [ ] Work offline with small local model (optional remote fallback)

**Example**
```bash
cheatsheet ai "list largest files in current folder"
# → du -ah . | sort -rh | head -20
```

---

## 🧠 Example Workflow

```bash
# Add some entries
cheatsheet add "ls" "List directory contents"
cheatsheet add "grep" "Search for text in files"

# Find something
cheatsheet grep

# Interactive search
cheatsheet

# Ask AI
cheatsheet ai "find all text files containing TODO"
```
