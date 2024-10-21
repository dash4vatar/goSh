use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::error::Error;
use zip::read::ZipArchive;

fn load_lines_from_file(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    Ok(lines)
}

fn try_password(zip_path: &str, password: &str) -> Result<bool, Box<dyn Error>> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if file.is_file() {
            let _ = file.extract(&mut std::fs::create_dir_all("extracted")?)?;
            return Ok(true);
        }
    }
    
    Ok(false)
}

fn main() -> Result<(), Box<dyn Error>> {
    let zip_path = "secret.zip"; 
    let passwords = load_lines_from_file("passwd.txt")?;

    for password in &passwords {
        match try_password(zip_path, password) {
            Ok(true) => {
                println!("Password found: {}", password);
                break; 
            }
            Ok(false) => {}
            Err(e) => {
                eprintln!("Error trying password {}: {}", password, e);
            }
        }
    }

    Ok(())
}
