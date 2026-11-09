use std::io::{Read, Write};

use super::helpers::get_monoc_path;

pub(crate) fn init() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_dir = get_monoc_path()?;
    let _ = std::fs::create_dir_all(&config_dir)?;

    config_dir.push("token");

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) // Створити, якщо немає
        .open(config_dir)?;

    let mut token = String::new();

    let _ = file.read_to_string(&mut token)?;

    if !token.is_empty() {
        println!("token exists: {}", token.len());
        return Ok(());
    }

    println!("Треба додати токен: ");

    std::io::stdout().flush()?;

    let token = rpassword::read_password()?;

    if token.is_empty() {
        return Err(Box::new(std::io::Error::other("Empty token")));
    }

    let _ = writeln!(&mut file, "{}", token)?;

    Ok(())
}
