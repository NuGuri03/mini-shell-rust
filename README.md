# Mini Shell Rust

A simple Unix-like shell implemented in Rust.  
This project aims to deepen understanding of system programming and the Rust language by building a functional command-line interface.

---

## 🛠 Build and Run
```bash
cargo build
cargo run
```

---

## ✨ Features

- **Built-in Commands**
  - `cd <dir>`: Change directory
  - `pwd`: Print current working directory
  - `exit`: Exit the shell

- **External Command Execution**
  - Run typical system commands such as `ls`, `date`, `cat` etc.

- **Input Parsing**
  - Split user input into tokens (`Vec<String>`) for processing

- **Pipes (`|`)**
  - Support for piping output between commands

- **I/O Redirection**
  - Input: `< input.txt`
  - Output: `> output.txt`

- **Background Execution**
  - Append `&` to run processes in the background

- **Command History**
  - Save and recall previously executed commands

- **Startup Configuration**
Load commands from `.myshellrc` at launch

---

## 📁 Project Structure
```bash
mini-shell-rust/
├── src/
│   ├── shell.rs
│   ├── shell/
│   │   ├── executor.rs 
│   │   ├── history.rs 
│   │   ├── parser.rs 
│   │   └── redirect.rs   
│   ├── io.rs
│   ├── io/
│   │   ├── prompt.rs 
│   │   └── raw_io.rs   
│   └── main.rs   
├── .gitignore        
├── Cargo.lock        
├── Cargo.toml        
└── README.md         
```

---