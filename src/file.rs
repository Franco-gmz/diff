use std::fs;

/// It receives a path to a file, opens it and returns the content in a vector by lines.
/// If the opening fails, it returns a message as String.
pub fn read_file_lines(filename: &str) -> Result<Vec<String>, String> {
    let mut content = Vec::new();
    let lines: String = match fs::read_to_string(filename) {
        Ok(readlines) => readlines,
        Err(_) => return Err("No se pudo abrir el archivo".to_string()),
    };
    let mut lines: Vec<&str> = lines.split('\n').collect();
    for line in lines.iter_mut() {
        content.push(line.to_string());
    }
    Ok(content)
}
