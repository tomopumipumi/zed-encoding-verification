use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let base_dir_str = if args.len() > 1 {
        &args[1]
    } else {
        "target_files"
    };

    let base_path = Path::new(base_dir_str);
    let originals_dir = base_path.join("originals");
    let saved_dir = base_path.join("saved");

    if !originals_dir.exists() || !saved_dir.exists() {
        eprintln!("❌ Error: Directories not found.");
        eprintln!("  Expected: {:?} and {:?}", originals_dir, saved_dir);
        eprintln!("  Usage: cargo run -- <ParentDirectory>");
        process::exit(1);
    }

    println!("📂 Target Directory: {:?}", base_path);
    println!("---------------------------------------------------");

    let mut passed_count = 0;
    let mut failed_count = 0;

    let entries = fs::read_dir(originals_dir)?;

    for entry in entries {
        let entry = entry?;
        let original_path = entry.path();

        if !original_path.is_file() {
            continue;
        }

        let file_name = match entry.file_name().into_string() {
            Ok(name) => {
                if name.starts_with(".") {
                    continue;
                }
                name
            }
            Err(_) => continue,
        };

        let saved_path = saved_dir.join(&file_name);

        match verify_file(&original_path, &saved_path, &file_name) {
            Ok(true) => {
                println!("✅ PASS: {}", file_name);
                passed_count += 1;
            }
            Ok(false) => {
                println!("❌ FAIL: {}", file_name);
                failed_count += 1;
            }
            Err(e) => {
                println!("⚠️ SKIP: {} ({})", file_name, e);
                failed_count += 1;
            }
        }
    }

    println!("---------------------------------------------------");
    println!("Total: {}", passed_count + failed_count);
    println!("Passed: {}", passed_count);
    if failed_count > 0 {
        eprintln!("Failed: {}", failed_count);
        process::exit(1);
    } else {
        println!("All files matched perfectly!");
    }

    Ok(())
}

fn verify_file(path_a: &PathBuf, path_b: &PathBuf, filename: &str) -> io::Result<bool> {
    if !path_b.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Saved file not found",
        ));
    }

    let bytes_a = read_file(path_a)?;
    let bytes_b = read_file(path_b)?;

    if bytes_a.len() != bytes_b.len() {
        eprintln!(
            "   [!] Size mismatch: Original={} vs Saved={}",
            bytes_a.len(),
            bytes_b.len()
        );

        return Ok(false);
    }

    let mut mismatch_count = 0;
    for (i, (byte_a, byte_b)) in bytes_a.iter().zip(bytes_b.iter()).enumerate() {
        if byte_a != byte_b {
            if mismatch_count < 3 {
                eprintln!(
                    "   [!] {} mismatch at {:#08x}: A={:02x}, B={:02x}",
                    filename, i, byte_a, byte_b
                );
            }

            // eprintln!("   --- Original Bytes ({}) ---", filename);
            // eprintln!("{:02x?}", bytes_a);

            // eprintln!("   --- Saved Bytes ({}) ---", filename);
            // eprintln!("{:02x?}", bytes_b);

            mismatch_count += 1;
        }
    }

    if mismatch_count > 0 {
        eprintln!("   [!] Total mismatched bytes: {}", mismatch_count);
        return Ok(false);
    }

    Ok(true)
}

fn read_file(path: &Path) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}
