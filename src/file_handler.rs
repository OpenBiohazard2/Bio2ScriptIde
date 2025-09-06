use crate::fileio::rdt::RDTHeader;
use crate::fileio::rdt_scd::parse_rdt_scd_stream;
use crate::fileio::opcode_data::init_opcode_documentation;
use crate::fileio::utils::read_file;
use std::collections::HashMap;
use std::path::Path;

/// Contains all data loaded from an RDT file
#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct RdtFileData {
    pub init_script: Vec<String>,
    pub main_script: Vec<String>,
    pub init_raw: String,
    pub main_raw: String,
    pub opcode_docs: HashMap<String, String>,
}

/// Handles file loading and parsing operations
pub struct FileHandler;

impl FileHandler {
    /// Loads and parses an RDT file, returning the parsed data
    pub fn load_rdt_file(file_path: &Path) -> Result<RdtFileData, String> {
        let filename = file_path.display().to_string();
        
        let contents = read_file(filename)
            .map_err(|e| format!("File read error: {}", e))?;
            
        let header = RDTHeader::from(&contents)
            .map_err(|e| format!("Invalid RDT file format: {}", e))?;
        
        let init_script_offset = header.offsets[16];
        let exec_script_offset = header.offsets[17];

        let opcode_docs = init_opcode_documentation();
        let (init_script, init_raw) = parse_rdt_scd_stream(&contents, init_script_offset);
        let (main_script, main_raw) = parse_rdt_scd_stream(&contents, exec_script_offset);

        Ok(RdtFileData {
            init_script,
            main_script,
            init_raw,
            main_raw,
            opcode_docs,
        })
    }
}
