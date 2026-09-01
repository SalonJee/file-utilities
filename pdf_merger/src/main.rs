use std::{env, path::PathBuf, process};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("Usage: pdf_merger <folder>");
        eprintln!("  Merges all PDFs in <folder> → <folder>/merged.pdf");
        process::exit(1);
    }

    let folder = PathBuf::from(&args[0]);

    if !folder.is_dir() {
        eprintln!("Error: '{}' is not a directory.", folder.display());
        process::exit(1);
    }

    // Collect + sort all .pdf files (skip merged.pdf itself)
    let mut pdf_files: Vec<PathBuf> = std::fs::read_dir(&folder)
        .unwrap_or_else(|e| { eprintln!("Cannot read folder: {}", e); process::exit(1); })
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.is_file()
                && p.extension().and_then(|s| s.to_str())
                    .map(|s| s.eq_ignore_ascii_case("pdf"))
                    .unwrap_or(false)
                && p.file_name().and_then(|s| s.to_str()) != Some("merged.pdf")
        })
        .collect();

    pdf_files.sort();

    if pdf_files.is_empty() {
        eprintln!("No PDF files found in '{}'.", folder.display());
        process::exit(1);
    }

    if pdf_files.len() < 2 {
        eprintln!("Need at least 2 PDF files to merge.");
        process::exit(1);
    }

    println!("Found {} PDF(s):", pdf_files.len());
    for f in &pdf_files {
        println!("  {}", f.file_name().unwrap().to_string_lossy());
    }

    let output = folder.join("merged.pdf");

    // Build: pdfunite input1.pdf input2.pdf ... merged.pdf
    let status = process::Command::new("pdfunite")
        .args(pdf_files.iter().map(|p| p.as_os_str()))
        .arg(&output)
        .status()
        .unwrap_or_else(|e| {
            eprintln!("Failed to run pdfunite: {}", e);
            eprintln!("Install it with: sudo apt install poppler-utils");
            process::exit(1);
        });

    if !status.success() {
        eprintln!("pdfunite failed with exit code: {:?}", status.code());
        process::exit(1);
    }

    println!("\nDone! Saved → {}", output.display());
}
