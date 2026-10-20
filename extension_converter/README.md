# Extension Converter

A Rust CLI tool that **actually converts** files between formats — not just renaming extensions.

Supports image-to-image conversions natively, and uses standard system tools for PDF and DOCX.

---

## Supported Conversions

| From → To | Method | External Tool Needed |
|---|---|---|
| **Image ↔ Image** (png, jpg, jpeg, webp, bmp, gif, tiff, ico) | Native (Rust `image` crate) | None |
| **Image → PDF** | External | `img2pdf` or ImageMagick |
| **PDF → Image** | External | `pdftoppm` (poppler-utils) |
| **DOCX → PDF** | External | LibreOffice |
| **PDF → DOCX** | External | LibreOffice |
| **EPUB → PDF** | External | Calibre (`ebook-convert`) |

---

## Requirements

```bash
# For image ↔ image: nothing extra needed!

# For image → PDF (pick one):
pip install img2pdf
# or
sudo apt install imagemagick

# For PDF → image:
sudo apt install poppler-utils

# For DOCX ↔ PDF:
sudo apt install libreoffice

# For EPUB → PDF:
sudo apt install calibre
```

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
