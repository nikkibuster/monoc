use std::io::Read;

pub(crate) fn get_monoc_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Читаємо змінну HOME
    let home = std::env::var("HOME").map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Змінна середовища HOME не знайдена",
        )
    })?;

    let mut path = std::path::PathBuf::from(home);
    path.push(".config");
    path.push("monoc");

    Ok(path)
}

pub(crate) fn get_token() -> Result<String, Box<dyn std::error::Error>> {
    let mut path = get_monoc_path()?;

    path.push("token");

    let mut file = std::fs::OpenOptions::new().read(true).open(path)?;

    let mut token = String::new();

    let _ = file.read_to_string(&mut token)?;

    if token.is_empty() {
        return Err(Box::new(std::io::Error::other(
            "token required run init command first",
        )));
    };

    Ok(token.trim().into())
}
