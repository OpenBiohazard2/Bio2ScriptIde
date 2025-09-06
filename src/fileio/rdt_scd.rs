use num;
use std::collections::HashMap;
use crate::fileio::opcode_data::*;

/// Type sizes in bytes for parameter parsing
const TYPE_SIZES: &[(&str, usize)] = &[
    ("u8", 1), ("i8", 1), ("u16", 2), 
    ("i16", 2), ("u32", 4), ("i32", 4)
];

/// Parse type information from a format term
/// 
/// Examples:
/// - "4u16" -> ("u16", 8)  // 4 elements × 2 bytes = 8 bytes
/// - "2i32" -> ("i32", 8)  // 2 elements × 4 bytes = 8 bytes  
/// - "1u8"  -> ("u8", 1)   // 1 element × 1 byte = 1 byte
fn parse_type_info(term: &str) -> Result<(&str, usize), String> {
    for &(suffix, bytes_per_element) in TYPE_SIZES {
        if term.ends_with(suffix) {
            let qty: usize = term.strip_suffix(suffix)
                .ok_or("Invalid format string")?
                .parse()
                .map_err(|_| "Invalid number format")?;
            return Ok((suffix, qty * bytes_per_element));
        }
    }
    Err(format!("Unknown type suffix in: {}", term))
}

/// Parse bytes based on type suffix
fn parse_bytes_by_type(data: &[u8], type_suffix: &str) -> Result<Vec<i32>, String> {
    match type_suffix {
        "u8" => Ok(data.iter().map(|&x| x as i32).collect()),
        "i8" => Ok(data.iter().map(|&x| (x as i8) as i32).collect()),
        "u16" => {
            Ok(data.chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]) as i32)
                .collect())
        },
        "i16" => {
            Ok(data.chunks_exact(2)
                .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]) as i32)
                .collect())
        },
        "u32" => {
            Ok(data.chunks_exact(4)
                .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as i32)
                .collect())
        },
        "i32" => {
            Ok(data.chunks_exact(4)
                .map(|chunk| i32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as i32)
                .collect())
        },
        _ => Err(format!("Unknown type suffix: {}", type_suffix)),
    }
}

fn parse_function_params(raw_function_params: &[u8], params_format: String) -> Result<Vec<i32>, String> {
    if params_format.is_empty() {
        if !raw_function_params.is_empty() {
            return Err("Expected empty params but got data".to_string());
        }
        return Ok(Vec::new());
    }

    let mut result = Vec::new();
    let mut offset = 0;
    
    for term in params_format.split(',') {
        let (type_suffix, bytes_needed) = parse_type_info(term)?;
        
        let data_slice = &raw_function_params[offset..offset + bytes_needed];
        let parsed_values = parse_bytes_by_type(data_slice, type_suffix)?;
        result.extend(parsed_values);
        
        offset += bytes_needed;
    }
    
    if raw_function_params.len() != offset {
        return Err("Mismatch between expected and actual parameter length".to_string());
    }
    
    Ok(result)
}

/// Parse opcode parameters and return formatted code/raw lines
fn parse_opcode_parameters(
    file_contents: &[u8],
    offset: usize,
    num_bytes: u8,
    info: &OpcodeInfo,
) -> Result<(String, String), String> {
    let raw_code_line = &file_contents[(offset - num_bytes as usize)..offset];
    let raw_function_param_values = &raw_code_line[1..raw_code_line.len()];
    
    let raw_line = format!("{:02x?}", raw_code_line);
    
    match parse_function_params(raw_function_param_values, info.function_params.clone()) {
        Ok(params) => {
            let function_name = &info.name;
            let code_line = if params.is_empty() {
                format!("{}()", function_name)
            } else {
                format!("{}({:?})", function_name, params)
            };
            Ok((code_line, raw_line))
        }
        Err(e) => {
            let error_code = format!("Error parsing parameters: {}", e);
            Err(format!("{}|{}", error_code, raw_line))
        }
    }
}

/// Process a single opcode and return formatted lines
fn process_opcode(
    file_contents: &[u8],
    opcode: &Option<Opcode>,
    opcode_byte: u8,
    offset: &mut usize,
    opcode_info_map: &HashMap<Opcode, OpcodeInfo>,
) -> Result<(String, String), String> {
    match opcode {
        Some(x) => {
            match opcode_info_map.get(x) {
                Some(info) => {
                    let num_bytes = info.instruction_size;
                    *offset += (num_bytes - 1) as usize;

                    match parse_opcode_parameters(
                        file_contents,
                        *offset,
                        num_bytes,
                        info,
                    ) {
                        Ok((code_line, raw_line)) => {
                            // Sleep contains sleep and sleeping commands
                            // The sleep command is [0x9 0xa u8 u8], where 0x9 is the sleep command and 0xa is the sleeping command
                            if *x == Opcode::Sleep {
                                *offset -= (num_bytes - 1) as usize;
                            }
                            Ok((code_line, raw_line))
                        }
                        Err(error_msg) => {
                            let parts: Vec<&str> = error_msg.split('|').collect();
                            Ok((parts[0].to_string(), parts[1].to_string()))
                        }
                    }
                }
                None => {
                    Ok((format!("Unknown opcode {}", opcode_byte), format!("{:02x}", opcode_byte)))
                }
            }
        }
        None => {
            Ok((format!("Unknown opcode {}", opcode_byte), format!("{:02x}", opcode_byte)))
        }
    }
}

fn parse_single_function(
    file_contents: &[u8],
    start_offset: u32,
    function_offset: u16,
    function_length: usize,
    function_index: usize,
    opcode_info_map: &HashMap<Opcode, OpcodeInfo>,
) -> (Vec<String>, Vec<String>) {
    let mut code_lines = Vec::new();
    let mut raw_code_lines = Vec::new();
    
    let mut function_cur_offset = start_offset as usize + function_offset as usize;
    code_lines.push(format!("Start Function {}:", function_index));
    raw_code_lines.push(format!("Start Function {}:", function_index));
    
    for _line_num in 0..function_length {
        let opcode_byte = file_contents[function_cur_offset];
        let opcode = &num::FromPrimitive::from_u8(opcode_byte);
        function_cur_offset += 1;
        
        match process_opcode(
            file_contents,
            opcode,
            opcode_byte,
            &mut function_cur_offset,
            opcode_info_map,
        ) {
            Ok((code_line, raw_line)) => {
                code_lines.push(code_line);
                raw_code_lines.push(raw_line);
            }
            Err(e) => {
                code_lines.push(format!("Error: {}", e));
                raw_code_lines.push(format!("{:02x}", opcode_byte));
                continue;
            }
        }
        
        // Check for EvtEnd after processing
        if let Some(Opcode::EvtEnd) = opcode {
            break;
        }
    }
    
    code_lines.push(format!("End Function {}\n", function_index));
    raw_code_lines.push(format!("End Function {}\n", function_index));
    
    (code_lines, raw_code_lines)
}

fn read_function_offsets(file_contents: &[u8], start_offset: u32) -> Result<Vec<u16>, &'static str> {
    // Check if we have enough data to read the first offset
    if file_contents.len() < (start_offset + 2) as usize {
        return Err("File too short to read function offsets");
    }
    
    let mut function_offsets = Vec::new();
    
    // Read the first offset
    let first_offset = u16::from_le_bytes([
        file_contents[start_offset as usize],
        file_contents[(start_offset + 1) as usize],
    ]);
    function_offsets.push(first_offset);
    
    // Read remaining offsets
    for i in (2..first_offset).step_by(2) {
        let offset_pos = start_offset as usize + i as usize;
        if file_contents.len() < offset_pos + 2 {
            return Err("File too short to read all function offsets");
        }
        
        let next_offset = u16::from_le_bytes([
            file_contents[offset_pos],
            file_contents[offset_pos + 1],
        ]);
        function_offsets.push(next_offset);
    }
    
    Ok(function_offsets)
}

// SCD file is within RDT
pub fn parse_rdt_scd_stream(file_contents: &[u8], start_offset: u32) -> (Vec<String>, String) {
    let opcode_info_map = init_opcode_info_map();

    let function_offsets = match read_function_offsets(file_contents, start_offset) {
        Ok(offsets) => offsets,
        Err(_) => {
            // If we can't read offsets, return empty results
            return (Vec::new(), "Error reading function offsets".to_string());
        }
    };

    let function_offsets_count = function_offsets.len();

    let mut code_lines = Vec::new();
    let mut raw_code_lines = Vec::new();

    for i in 0..function_offsets_count {
        let function_length = match i {
            i if i == function_offsets_count - 1 => file_contents.len() - start_offset as usize,
            _ => (function_offsets[i + 1] - function_offsets[i]).into(),
        };

        let (mut function_code_lines, mut function_raw_lines) = parse_single_function(
            file_contents,
            start_offset,
            function_offsets[i],
            function_length,
            i,
            &opcode_info_map,
        );
        
        code_lines.append(&mut function_code_lines);
        raw_code_lines.append(&mut function_raw_lines);
    }
    (code_lines, raw_code_lines.join("\n"))
}
