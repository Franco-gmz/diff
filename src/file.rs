use std::fs;

/// It receives a path to a file, opens it and returns the content in a vector by lines.
/// If the opening fails, it returns a message as String.
pub fn read_file_lines(filename: &str) -> Result<Vec<String>, String> {
    let mut content = Vec::new();
    let lines: String = match fs::read_to_string(filename) {
        Ok(readlines) => readlines,
        Err(_) => return Err("No se pudo abrir el archivo".to_string()),
    };
    if lines != ""{
        let mut lines: Vec<&str> = lines.split('\n').collect();
        for line in lines.iter_mut() {
            content.push(line.to_string());
        }
    }
    Ok(content)
}

#[cfg(test)]
mod test{
    use crate::file::read_file_lines;


    #[test]
    fn couldnt_open(){
        let non_existent_file = "name";
        let expected_message = "No se pudo abrir el archivo".to_string();
        assert_eq!(Err(expected_message), read_file_lines(&non_existent_file));
    }

    #[test]
    fn empty_file(){
        let empty_file_path = "./assets/empty.txt";
        let expected_length = 0;
        let lines = read_file_lines(&empty_file_path).unwrap();
        println!("{:?}", lines);
        assert_eq!(expected_length, lines.len());
    }
}
