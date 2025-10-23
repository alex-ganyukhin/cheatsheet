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

## 🧩 Implementation Roadmap

### **1. Project Setup**
- [ ] Initialize cargo workspace (`cargo new cheatsheet --bin`)  
- [ ] Add dependencies:  
  - `clap` (CLI parsing)  
  - `serde`, `toml` (storage)  
  - `fuzzy-matcher` (fuzzy search)  
  - `ratatui` / `crossterm` (TUI, later)  
- [ ] Setup config path in `~/.config/cheatsheet/`  

---

### **2. Data Model & Storage**
- [ ] Define `Entry` struct: `{ title, command, description, tags }`  
- [ ] Implement file-based storage (TOML)  
- [ ] CRUD operations: add, list, remove  
- [ ] Handle duplicate prevention + autosave  

---

### **3. Fuzzy Search Engine**
- [ ] Integrate fuzzy matching library  
- [ ] Implement `search(term)` → ranked results  
- [ ] Support partial matches and case insensitivity  
- [ ] Display match results with score  

---

### **4. Command-Line Interface**
- [ ] Build `add`, `list`, `remove`, `search` subcommands  
- [ ] Use `clap` derive API  
- [ ] Add colored output via `colored` crate  
- [ ] Add help and version flags  

---

### **5. Interactive Search (TUI)**
- [ ] Initialize TUI screen with `ratatui`  
- [ ] Display list of entries dynamically filtered as user types  
- [ ] Add navigation (↑↓, Enter, Esc)  
- [ ] Integrate with existing search engine  
- [ ] Persist last used entry  

---

### **6. AI Search (Natural Language)**
- [ ] Add optional embedding model loader (ONNX or remote API)  
- [ ] Generate vector embeddings for each entry (title + description)  
- [ ] Build local ANN index for fast retrieval  
- [ ] Implement hybrid ranker (semantic + fuzzy)  
- [ ] Return top candidate with rationale  

---

### **7. Polish & Distribution**
- [ ] Write usage examples and help text  
- [ ] Add `--init` command to create starter cheat sheet  
- [ ] Add tests for storage and search logic  
- [ ] Publish binary to crates.io  
- [ ] Optional: prebuilt binaries via GitHub Actions  

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

---

## 📍 Roadmap Summary

| Milestone | Focus | Core Outcome |
|------------|--------|--------------|
| ✅ Base | Store and retrieve commands | CLI + file storage |
| ⏳ Extended | Interactive search | TUI + history |
| 🔮 AI Search | Natural-language lookup | Smart command matching |
