# 📝 task-cli

`task-cli` is a **Rust-based CLI application** to manage your tasks directly from the terminal.  
Fast, simple, and effective, it helps you keep track of tasks with different statuses.

---

## 🚀 Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/Glindel/todolist.git
cd task-cli
cargo build --release
```

The binary will be available in `target/release/task-cli.`

## 📌 Usage

Here are the available commands:

### ➕ Add a task

```bash
task-cli add "Write the documentation"
```

### 📋 List tasks

List all tasks:
```bash
task-cli list
```

List tasks by status (`todo`, `in-progress`, `done`):
```bash
task-cli list todo
```

### 🔄 Update a task status

Mark a task as a specific status (`todo`, `in-progress`, `done`):

```bash
task-cli mark-todo 1
```

### 🔄 Update a task

```bash
task-cli update 
```

### ❌ Delete a task

```bash
task-cli delete 1
```
