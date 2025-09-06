use std::fs::File;
use std::io::Read;
use std::path::Path;

// general function for any file type
pub fn read_file(mut file_name: String) -> Result<Vec<u8>, String> {
    file_name = file_name.replace("/", "");
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
