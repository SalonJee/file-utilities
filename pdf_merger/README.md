# pdf_merger

A fast CLI tool written in Rust that merges all PDF files inside a folder into a single `merged.pdf`, saved in that same folder.

---

## Requirements

Make sure these are installed before anything:

```bash
# Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# pdfunite (the merge engine — from poppler-utils)
sudo apt install poppler-utils       # Ubuntu / Debian
sudo dnf install poppler-utils       # Fedora
sudo pacman -S poppler               # Arch
```

---

## Option 1 — Install Globally (Recommended)

Install once, use from anywhere.

```bash
# 1. Go into the project folder
cd /home/salon-timsina/Documents/projects/pdf_related_works/pdf_merger

# 2. Install the binary globally
cargo install --path .
```

That's it. The binary is now available system-wide as `pdf_merger`.

### Usage

```bash
pdf_merger /path/to/your/folder
```

### Examples

```bash
# Merge all PDFs in Downloads/reports/
pdf_merger ~/Downloads/reports/

# Merge all PDFs in a project folder
pdf_merger ~/Documents/invoices/

# Works from any directory
pdf_merger /mnt/data/scans/
```

### Output

The merged file is always saved as `merged.pdf` inside the folder you passed:

```
Found 3 PDF(s):
  chapter1.pdf
  chapter2.pdf
  chapter3.pdf

Done! Saved → /path/to/your/folder/merged.pdf
```

> **Note:** If `merged.pdf` already exists in that folder, it gets overwritten.
> The tool automatically skips it when scanning for input files.

---

## Option 2 — Compile and Run Directly (No Install)

Use this if you don't want to install the binary globally.

```bash
# 1. Go into the project folder
cd /home/salon-timsina/Documents/projects/pdf_related_works/pdf_merger

# 2. Compile (only needed once, or after code changes)
cargo build --release
```

The compiled binary is now at:
```
target/release/pdf_merger
```

### Usage

```bash
# From inside the project folder
./target/release/pdf_merger /path/to/your/folder

# From anywhere using the full path
/home/salon-timsina/Documents/projects/pdf_related_works/pdf_merger/target/release/pdf_merger ~/Downloads/pdfs/
```

### Example

```bash
cd /home/salon-timsina/Documents/projects/pdf_related_works/pdf_merger
./target/release/pdf_merger ~/Downloads/pdfs_of_us/
```

---

## Comparison

| | Global Install | Direct Binary |
|---|---|---|
| **Command** | `pdf_merger <folder>` | `./target/release/pdf_merger <folder>` |
| **Works from anywhere** | ✅ Yes | ❌ Need full path |
| **Setup command** | `cargo install --path .` | `cargo build --release` |
| **Recommended for** | Everyday use | Testing / one-off use |

---

## Updating After Code Changes

If you edit `src/main.rs` and want to apply the changes:

```bash
# Global install — recompile + reinstall
cargo install --path .

# Direct binary — just recompile
cargo build --release
```

---

## How It Works

1. Scans the given folder for all `.pdf` files (sorted alphabetically)
2. Skips any existing `merged.pdf` to avoid self-including a previous run
3. Calls `pdfunite` (poppler) under the hood to perform the merge
4. Saves the result as `merged.pdf` in the same folder
