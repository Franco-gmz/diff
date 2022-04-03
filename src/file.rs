use std::fs;

///A structure which contains the content of a file by lines
pub struct File {
    lines: Vec<String>,
}

impl File {
    /// It receives a path to a file, opens it and saves the content. If the opening fails, it returns a
    /// message as String.
    pub fn new(filename: &str) -> Result<Self, String> {
        let mut content = Vec::new();
        let lines: String = match fs::read_to_string(filename) {
            Ok(readlines) => readlines,
            Err(_) => return Err("No se pudo abrir el archivo".to_string()),
        };
        let mut lines: Vec<&str> = lines.split('\n').collect();
        for line in lines.iter_mut() {
            content.push(line.to_string());
        }
        Ok(File { lines: content })
    }
    ///It returns the content of the opened file.
    pub fn content(&self) -> Vec<String> {
        self.lines.clone()
    }
}
