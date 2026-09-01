# Extension Converter

A lightning-fast, zero-dependency Rust CLI tool to quickly change file extensions for either a single file or an entire directory of files. 

It does not change the internal file format (e.g., it doesn't convert image pixels to PDF data); it strictly renames the text of the extension itself.

## Usage

### 1. Single File Mode
Change the extension of one specific file.
```bash
extension_converter /path/to/file.old_ext new_ext
```
**Example:**
```bash
extension_converter ~/Documents/notes.txt md
# Renames 'notes.txt' to 'notes.md'
```

### 2. Batch Mode (Directory)
Change the extension of **all** matching files inside a specific folder.
```bash
extension_converter /path/to/folder old_ext new_ext
```
**Example:**
```bash
extension_converter ~/Images jpeg jpg
# Renames all '.jpeg' files in ~/Images to '.jpg'
```

---

## Global Install

To install this tool so you can use it from absolutely anywhere on your system, open your terminal inside this project folder and run:

```bash
cargo install --path .
```

*That's it! You can now use the `extension_converter` command from any directory.*
