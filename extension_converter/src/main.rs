use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

const IMAGE_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "webp", "bmp", "gif", "tiff", "tif", "ico",
];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage 1 (Single File): {} <file> <new_extension>", args[0]);
        eprintln!("Usage 2 (Directory):   {} <directory> <old_ext> <new_ext>", args[0]);
        eprintln!();
        eprintln!("Supported conversions:");
        eprintln!("  Image ↔ Image : png, jpg, jpeg, webp, bmp, gif, tiff, ico");
        eprintln!("  Image → PDF   : png/jpg/... → pdf   (needs img2pdf or magick)");
        eprintln!("  PDF   → Image : pdf → png/jpg        (needs pdftoppm)");
        eprintln!("  DOCX  ↔ PDF   : docx ↔ pdf           (needs libreoffice)");
        eprintln!("  EPUB  → PDF   : epub → pdf            (needs calibre)");
        eprintln!();
        eprintln!("Note: Quotes are optional even if filenames have spaces!");
        eprintln!("Example: {} My Book Name.epub pdf", args[0]);
        process::exit(1);
    }

    let raw_last = args.last().unwrap();
    let new_ext = if let Some(ext) = Path::new(raw_last).extension().and_then(|e| e.to_str()) {
        ext.to_lowercase()
    } else {
        raw_last.trim_start_matches('.').to_lowercase()
    };

    // 1. Try Batch Directory Mode first if >= 4 arguments and directory exists
    if args.len() >= 4 {
        let dir_candidate_str = args[1..args.len() - 2].join(" ");
        let dir_path = Path::new(&dir_candidate_str);
        if dir_path.is_dir() {
            let old_ext = args[args.len() - 2].trim_start_matches('.').to_lowercase();
            ensure_dependencies(&old_ext, &new_ext);
            batch_convert(dir_path, &old_ext, &new_ext);
            return;
        }
    }

    // 2. Single File Mode
    let file_candidate_str = args[1..args.len() - 1].join(" ");
    let file_path = Path::new(&file_candidate_str);

    if file_path.exists() {
        if file_path.is_dir() {
            eprintln!("Error: '{}' is a directory. For directory batch conversion, provide both old and new extensions.", file_candidate_str);
            eprintln!("Example: {} ./my_folder png jpg", args[0]);
            process::exit(1);
        }

        let old_ext = file_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        ensure_dependencies(&old_ext, &new_ext);

        println!("⏳ Converting {:?} → .{}...", file_path.file_name().unwrap(), new_ext);
        match convert_file(file_path, &new_ext) {
            Ok(output) => println!(
                "✅ Converted: {:?} → {:?}",
                file_path.file_name().unwrap(),
                output.file_name().unwrap()
            ),
            Err(e) => {
                eprintln!("❌ {}", e);
                process::exit(1);
            }
        }
    } else {
        eprintln!("Error: File '{}' does not exist.", file_candidate_str);
        process::exit(1);
    }
}

fn ensure_dependencies(old_ext: &str, new_ext: &str) {
    let old_is_image = is_image(old_ext);
    let new_is_image = is_image(new_ext);

    let needs_img2pdf = old_is_image && new_ext == "pdf";
    let needs_pdftoppm = old_ext == "pdf" && new_is_image;
    let needs_libreoffice = (old_ext == "docx" && new_ext == "pdf")
        || (old_ext == "pdf" && new_ext == "docx");
    let needs_ebook_convert = old_ext == "epub" && new_ext == "pdf";

    if needs_img2pdf {
        if !command_exists("img2pdf") && !command_exists("magick") && !command_exists("convert") {
            println!("📦 'img2pdf' not found. Installing via pip...");
            if run_install("pip3", &["install", "--break-system-packages", "img2pdf"]).is_err() {
                if run_install("pip", &["install", "--break-system-packages", "img2pdf"]).is_err() {
                    println!("⚠️  pip install img2pdf failed. Installing ImageMagick as fallback...");
                    run_install("sudo", &["apt", "install", "-y", "imagemagick"]).ok();
                }
            }
        }
    }

    if needs_pdftoppm && !command_exists("pdftoppm") {
        println!("📦 'pdftoppm' not found. Installing poppler-utils...");
        run_install("sudo", &["apt", "install", "-y", "poppler-utils"]).ok();
    }

    if needs_libreoffice && !command_exists("libreoffice") {
        println!("📦 'libreoffice' not found. Installing...");
        run_install("sudo", &["apt", "install", "-y", "libreoffice"]).ok();
    }

    if needs_ebook_convert && !command_exists("ebook-convert") {
        println!("📦 'ebook-convert' not found. Installing calibre...");
        run_install("sudo", &["apt", "install", "-y", "calibre"]).ok();
    }
}

fn command_exists(cmd: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {}", cmd))
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn run_install(cmd: &str, args: &[&str]) -> Result<(), String> {
    match Command::new(cmd).args(args).output() {
        Ok(out) if out.status.success() => Ok(()),
        Ok(out) => Err(format!(
            "Command '{} {:?}' failed: {}",
            cmd,
            args,
            String::from_utf8_lossy(&out.stderr)
        )),
        Err(e) => Err(format!("Failed to run '{} {:?}': {}", cmd, args, e)),
    }
}

// ─── Core conversion logic ──────────────────────────────────────────────────

fn convert_file(input: &Path, new_ext: &str) -> Result<PathBuf, String> {
    let old_ext = input
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let mut output = input.to_path_buf();
    output.set_extension(new_ext);

    if old_ext == new_ext || (old_ext == "jpg" && new_ext == "jpeg") || (old_ext == "jpeg" && new_ext == "jpg") {
        return Err(format!("Source and target are both .{}, skipping.", new_ext));
    }

    let old_is_image = is_image(&old_ext);
    let new_is_image = is_image(new_ext);

    match (old_ext.as_str(), new_ext, old_is_image, new_is_image) {
        (_, _, true, true) => {
            let img = image::open(input)
                .map_err(|e| format!("Failed to open image: {}", e))?;
            img.save(&output)
                .map_err(|e| format!("Failed to save as .{}: {}", new_ext, e))?;
        }

        (_, "pdf", true, _) => {
            image_to_pdf(input, &output)?;
        }

        ("pdf", _, _, true) => {
            pdf_to_image(input, &output, new_ext)?;
        }

        ("docx", "pdf", _, _) => {
            libreoffice_convert(input, "pdf")?;
        }

        ("pdf", "docx", _, _) => {
            libreoffice_convert(input, "docx")?;
        }

        ("epub", "pdf", _, _) => {
            ebook_convert(input, &output)?;
        }

        _ => {
            return Err(format!(
                "Conversion from .{} to .{} is not supported.",
                old_ext, new_ext
            ));
        }
    }

    Ok(output)
}

fn is_image(ext: &str) -> bool {
    IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str())
}

// ─── Image → PDF ────────────────────────────────────────────────────────────

fn image_to_pdf(input: &Path, output: &Path) -> Result<(), String> {
    if let Ok(out) = Command::new("img2pdf")
        .arg(input)
        .arg("-o")
        .arg(output)
        .output()
    {
        if out.status.success() {
            return Ok(());
        }
    }

    if let Ok(out) = Command::new("magick")
        .arg(input)
        .arg(output)
        .output()
    {
        if out.status.success() {
            return Ok(());
        }
    }

    if let Ok(out) = Command::new("convert")
        .arg(input)
        .arg(output)
        .output()
    {
        if out.status.success() {
            return Ok(());
        }
    }

    Err(
        "Image → PDF requires 'img2pdf' or ImageMagick (magick/convert). Dependencies should have been auto-installed."
            .into(),
    )
}

// ─── PDF → Image ────────────────────────────────────────────────────────────

fn pdf_to_image(input: &Path, output: &Path, format: &str) -> Result<(), String> {
    let stem = output
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let parent = output.parent().unwrap_or(Path::new("."));

    let fmt_flag = match format {
        "jpg" | "jpeg" => "-jpeg",
        "png" => "-png",
        "tiff" | "tif" => "-tiff",
        _ => "-png",
    };

    let result = Command::new("pdftoppm")
        .arg("-singlefile")
        .arg(fmt_flag)
        .arg(input)
        .arg(parent.join(stem))
        .output();

    match result {
        Ok(out) if out.status.success() => Ok(()),
        Ok(out) => Err(format!(
            "pdftoppm failed: {}",
            String::from_utf8_lossy(&out.stderr)
        )),
        Err(_) => Err(
            "PDF → Image requires 'pdftoppm' (poppler-utils). Dependencies should have been auto-installed."
                .into(),
        ),
    }
}

// ─── DOCX ↔ PDF (LibreOffice) ───────────────────────────────────────────────

fn libreoffice_convert(input: &Path, target_format: &str) -> Result<(), String> {
    let parent = input.parent().unwrap_or(Path::new("."));

    let result = Command::new("libreoffice")
        .arg("--headless")
        .arg("--convert-to")
        .arg(target_format)
        .arg("--outdir")
        .arg(parent)
        .arg(input)
        .output();

    match result {
        Ok(out) if out.status.success() => Ok(()),
        Ok(out) => Err(format!(
            "LibreOffice failed: {}",
            String::from_utf8_lossy(&out.stderr)
        )),
        Err(_) => Err(
            "DOCX ↔ PDF requires LibreOffice. Dependencies should have been auto-installed."
                .into(),
        ),
    }
}

// ─── EPUB → PDF (Calibre) ───────────────────────────────────────────────────

fn ebook_convert(input: &Path, output: &Path) -> Result<(), String> {
    let result = Command::new("ebook-convert")
        .arg(input)
        .arg(output)
        .output();

    match result {
        Ok(out) if out.status.success() => Ok(()),
        Ok(out) => Err(format!(
            "ebook-convert failed: {}",
            String::from_utf8_lossy(&out.stderr)
        )),
        Err(_) => Err(
            "EPUB → PDF requires Calibre (ebook-convert). Dependencies should have been auto-installed."
                .into(),
        ),
    }
}

// ─── Batch conversion ───────────────────────────────────────────────────────

fn batch_convert(dir: &Path, old_ext: &str, new_ext: &str) {
    let mut count = 0;
    let mut errors = 0;

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext.to_string_lossy().to_lowercase() == old_ext {
                            match convert_file(&path, new_ext) {
                                Ok(output) => {
                                    println!(
                                        "✅ {:?} → {:?}",
                                        path.file_name().unwrap(),
                                        output.file_name().unwrap()
                                    );
                                    count += 1;
                                }
                                Err(e) => {
                                    eprintln!(
                                        "❌ {:?}: {}",
                                        path.file_name().unwrap(),
                                        e
                                    );
                                    errors += 1;
                                }
                            }
                        }
                    }
                }
            }
            println!("\nDone! {} converted, {} failed.", count, errors);
        }
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            process::exit(1);
        }
    }
}
