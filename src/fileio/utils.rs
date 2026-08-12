use std::fs::File;
use std::io::Read;
use std::path::Path;

// general function for any file type
pub fn read_file(file_name: String) -> Result<Vec<u8>, String> {
    if file_name.is_empty() {
        return Err("No file specified".to_string());
    }

    let path = Path::new(&file_name);
    if !path.exists() {
        return Err(format!("File not found: {}", file_name));
    }

    let mut file_content = Vec::new();
    let mut file = File::open(&file_name)
        .map_err(|e| format!("Unable to open file '{}': {}", file_name, e))?;
    file.read_to_end(&mut file_content)
        .map_err(|e| format!("Unable to read file '{}': {}", file_name, e))?;
    Ok(file_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn rejects_empty_file_name() {
        assert!(read_file(String::new()).is_err());
    }

    #[test]
    fn rejects_missing_file() {
        let result = read_file("this_file_should_not_exist_12345.rdt".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn reads_existing_file_contents() {
        let mut path = std::env::temp_dir();
        path.push("bio2_script_ide_test_read_file.bin");
        let path_str = path.display().to_string();

        fs::write(&path, [1u8, 2, 3, 4]).expect("failed to write temp file");

        let result = read_file(path_str);

        fs::remove_file(&path).ok();

        assert_eq!(result.unwrap(), vec![1, 2, 3, 4]);
    }
}
