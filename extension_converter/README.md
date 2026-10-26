# Extension Converter

A Rust CLI tool that **actually converts** files between formats — not just renaming extensions.

Supports image-to-image conversions natively, and uses standard system tools for PDF and DOCX.

---

## Supported Conversions

| From → To | Method | External Tool Needed |
|---|---|---|
| **Image ↔ Image** (png, jpg, jpeg, webp, bmp, gif, tiff, ico) | Native (Rust `image` crate) | None |
| **Image → PDF** | External | `img2pdf` or ImageMagick (auto-installed) |
| **PDF → Image** | External | `pdftoppm` (poppler-utils, auto-installed) |
| **DOCX → PDF** | External | LibreOffice (auto-installed) |
| **PDF → DOCX** | External | LibreOffice (auto-installed) |
| **EPUB → PDF** | External | Calibre (`ebook-convert`, auto-installed) |

---

## Requirements

The tool **auto-installs missing dependencies** on first use — you don't need to install anything beforehand.

It checks for required system tools before each conversion and installs any that are missing (via `apt` or `pip`). This happens once per missing tool.

```bash
# For image ↔ image: nothing extra needed!

# For image → PDF: img2pdf (pip) or ImageMagick (apt) — auto-installed
# For PDF → image: poppler-utils (apt) — auto-installed
# For DOCX ↔ PDF: libreoffice (apt) — auto-installed
# For EPUB → PDF: calibre (apt) — auto-installed
```

> **Note:** Auto-install uses `sudo apt install -y` and `pip3 install --break-system-packages`. Make sure you have `sudo` access (passwordless sudo recommended for unattended installs).

---

## Usage

### 1. Single File Mode

```bash
extension_converter <file> <new_extension>
```

> **Tip:** You **don't** need quotes/inverted commas even if the filename contains spaces!

**Examples:**

```bash
# Convert with spaces (no quotes needed!)
extension_converter Last Night at the Telegraph Club.epub pdf

# Convert PNG to JPEG (real pixel conversion)
extension_converter photo.png jpg

# Convert an image to PDF
extension_converter diagram.png pdf

# Convert PDF to PNG
extension_converter document.pdf png

# Convert DOCX to PDF
extension_converter report.docx pdf
```

### 2. Batch Mode (Directory)

Convert **all** matching files inside a folder.

```bash
extension_converter <directory> <old_ext> <new_ext>
```

**Examples:**

```bash
# Convert all PNGs in a folder to JPEGs
extension_converter ~/Images png jpg

# Convert all TIFFs to PNGs
extension_converter ~/scans tiff png

# Convert all DOCXs to PDFs
extension_converter ~/Documents docx pdf
```

### Output

```
✅ Converted: "photo.png" → "photo.jpg"
✅ Converted: "diagram.png" → "diagram.jpg"

Done! 2 converted, 0 failed.
```

> **Note:** The original file is kept — the converted file is saved alongside it.

---

## Global Install

```bash
cargo install --path .
```

That's it! `extension_converter` is now available from any directory.

---

## Updating After Code Changes

```bash
cargo install --path .
```

---

## How It Works

1. Detects the source and target formats from the file extensions
2. **Image ↔ Image**: Uses the Rust `image` crate to decode the source and re-encode in the target format (real pixel data conversion)
3. **Image → PDF**: Shells out to `img2pdf` or ImageMagick `magick`
4. **PDF → Image**: Shells out to `pdftoppm` (from poppler-utils)
5. **DOCX ↔ PDF**: Shells out to LibreOffice in headless mode
6. Unsupported conversions are rejected with a clear error message
