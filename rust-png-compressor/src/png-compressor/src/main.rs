use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat};

/// Parse a human-friendly size string like "500KB", "2MB", "1024" (bytes) into bytes.
fn parse_size(s: &str) -> Result<u64> {
    let s = s.trim().to_uppercase();
    if let Some(n) = s.strip_suffix("GB") {
        Ok(n.trim().parse::<u64>()? * 1024 * 1024 * 1024)
    } else if let Some(n) = s.strip_suffix("MB") {
        Ok(n.trim().parse::<u64>()? * 1024 * 1024)
    } else if let Some(n) = s.strip_suffix("KB") {
        Ok(n.trim().parse::<u64>()? * 1024)
    } else {
        Ok(s.parse::<u64>()?)
    }
}

/// Try lossless PNG re-compression with oxipng at increasing effort levels.
/// Returns true if the file now fits within max_bytes.
fn try_oxipng(path: &Path, max_bytes: u64) -> Result<bool> {
    let mut opts = oxipng::Options::default();
    // Use high compression with the current oxipng API.
    opts.deflater = oxipng::Deflater::Libdeflater { compression: 12 };
    opts.optimize_alpha = true;
    opts.strip = oxipng::StripChunks::Safe;

    oxipng::optimize(
        &oxipng::InFile::Path(path.to_path_buf()),
        &oxipng::OutFile::Path {
            path: Some(path.to_path_buf()),
            preserve_attrs: false,
        },
        &opts,
    )
        .with_context(|| format!("oxipng failed on {}", path.display()))?;

    let size = fs::metadata(path)?.len();
    Ok(size <= max_bytes)
}

/// Reduce image dimensions by `scale` factor (0.0–1.0) and re-save as PNG.
fn resize_and_save(path: &Path, img: &DynamicImage, scale: f32) -> Result<()> {
    let new_w = ((img.width() as f32) * scale).round() as u32;
    let new_h = ((img.height() as f32) * scale).round() as u32;
    let resized = img.resize(new_w, new_h, image::imageops::FilterType::Lanczos3);
    resized
        .save_with_format(path, ImageFormat::Png)
        .with_context(|| format!("Failed to save resized image to {}", path.display()))?;
    Ok(())
}

fn process_file(png_path: &Path, max_bytes: u64) -> Result<()> {
    let original_size = fs::metadata(png_path)?.len();
    println!(
        "[{}] original size: {} bytes",
        png_path.display(),
        original_size
    );

    if original_size <= max_bytes {
        println!("  → already within limit, skipping.");
        return Ok(());
    }

    // --- Backup -------------------------------------------------------
    let backup_path: PathBuf = {
        let stem = png_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let backup_name = format!("{}.backup.png", stem);
        png_path.with_file_name(backup_name)
    };

    fs::copy(png_path, &backup_path)
        .with_context(|| format!("Failed to backup to {}", backup_path.display()))?;
    println!("  → backed up to {}", backup_path.display());

    // --- Step 1: lossless oxipng re-compression -----------------------
    if try_oxipng(png_path, max_bytes)? {
        let new_size = fs::metadata(png_path)?.len();
        println!(
            "  → lossless compression succeeded: {} bytes (saved {} bytes)",
            new_size,
            original_size.saturating_sub(new_size)
        );
        return Ok(());
    }

    println!("  → lossless compression not enough, trying lossy resize…");

    // --- Step 2: lossy – progressive downscaling ----------------------
    // Restore from backup before each attempt so we always resize the original.
    let original_img = image::open(&backup_path)
        .with_context(|| format!("Failed to open backup {}", backup_path.display()))?;

    // Try scales: 90%, 80%, 70%, …, 10%
    for percent in (1..=9u32).rev() {
        let scale = percent as f32 / 10.0;

        // Restore original pixels before resizing
        fs::copy(&backup_path, png_path)
            .with_context(|| "Failed to restore from backup before resize")?;

        resize_and_save(png_path, &original_img, scale)?;

        // Apply lossless optimisation on top of the resized image
        let _ = try_oxipng(png_path, max_bytes); // ignore inner error; check size below

        let new_size = fs::metadata(png_path)?.len();
        if new_size <= max_bytes {
            println!(
                "  → resized to {}% → {} bytes (saved {} bytes)",
                percent * 10,
                new_size,
                original_size.saturating_sub(new_size)
            );
            return Ok(());
        }
    }

    // If even 10% didn't fit, warn but leave the smallest version in place.
    let final_size = fs::metadata(png_path)?.len();
    eprintln!(
        "  ⚠ Warning: could not compress '{}' below {} bytes (best: {} bytes). \
         Backup retained.",
        png_path.display(),
        max_bytes,
        final_size
    );

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!(
            "Usage: {} <directory> <max_size>\n\
             \n\
             <directory>  Path containing PNG files to compress.\n\
             <max_size>   Maximum allowed file size, e.g. 500KB, 2MB, 1048576 (bytes).\n\
             \n\
             For each PNG the tool will:\n\
               1. Skip it if it is already within the limit.\n\
               2. Create a backup file (<name>.backup.png) next to the original.\n\
               3. Try lossless PNG re-compression (oxipng).\n\
               4. If still too large, progressively downscale (90% … 10%) until it fits.",
            args[0]
        );
        process::exit(1);
    }

    let dir = Path::new(&args[1]);
    let max_bytes = match parse_size(&args[2]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Invalid size argument '{}': {}", args[2], e);
            process::exit(1);
        }
    };

    if !dir.is_dir() {
        eprintln!("'{}' is not a directory.", dir.display());
        process::exit(1);
    }

    println!(
        "Processing PNGs in '{}' | target max size: {} bytes\n",
        dir.display(),
        max_bytes
    );

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Cannot read directory '{}': {}", dir.display(), e);
            process::exit(1);
        }
    };

    let mut processed = 0usize;
    let mut errors = 0usize;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            // Skip backup files we created ourselves
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with(".backup.png") {
                    continue;
                }
            }
            if path.extension().and_then(|e| e.to_str()).map(|e| e.eq_ignore_ascii_case("png"))
                == Some(true)
            {
                processed += 1;
                if let Err(e) = process_file(&path, max_bytes) {
                    eprintln!("Error processing '{}': {:?}", path.display(), e);
                    errors += 1;
                }
            }
        }
    }

    println!("\nDone. Processed {} file(s), {} error(s).", processed, errors);
    if errors > 0 {
        process::exit(1);
    }
}
