use crate::fileio::opcode_data::*;
use num;
use std::collections::HashMap;

/// A fixed-width numeric parameter type that can appear in an opcode's parameter format
/// string (e.g. the "u16" in "4u16"). Replaces matching on bare type-suffix strings: the
/// byte size and the byte-parsing logic for each type are defined exactly once, here.
#[derive(Clone, Copy)]
enum ParamType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
}

impl ParamType {
    /// All variants paired with the format-string suffix that names them.
    const ALL: [(&'static str, ParamType); 6] = [
        ("u8", ParamType::U8),
        ("i8", ParamType::I8),
        ("u16", ParamType::U16),
        ("i16", ParamType::I16),
        ("u32", ParamType::U32),
        ("i32", ParamType::I32),
    ];

    fn byte_size(self) -> usize {
        match self {
            Self::U8 | Self::I8 => 1,
            Self::U16 | Self::I16 => 2,
            Self::U32 | Self::I32 => 4,
        }
    }

    /// Parses `data` as a back-to-back sequence of this type, each element widened to `i32`.
    fn parse(self, data: &[u8]) -> Vec<i32> {
        match self {
            Self::U8 => data.iter().map(|&x| x as i32).collect(),
            Self::I8 => data.iter().map(|&x| (x as i8) as i32).collect(),
            Self::U16 => data
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]) as i32)
                .collect(),
            Self::I16 => data
                .chunks_exact(2)
                .map(|c| i16::from_le_bytes([c[0], c[1]]) as i32)
                .collect(),
            Self::U32 => data
                .chunks_exact(4)
                .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]) as i32)
                .collect(),
            Self::I32 => data
                .chunks_exact(4)
                .map(|c| i32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect(),
        }
    }
}

/// Parse type information from a format term
///
/// Examples:
/// - "4u16" -> (U16, 8)  // 4 elements × 2 bytes = 8 bytes
/// - "2i32" -> (I32, 8)  // 2 elements × 4 bytes = 8 bytes
/// - "1u8"  -> (U8, 1)   // 1 element × 1 byte = 1 byte
fn parse_type_info(term: &str) -> Result<(ParamType, usize), String> {
    for &(suffix, param_type) in &ParamType::ALL {
        if let Some(qty_str) = term.strip_suffix(suffix) {
            let qty: usize = qty_str.parse().map_err(|_| "Invalid number format")?;
            return Ok((param_type, qty * param_type.byte_size()));
        }
    }
    Err(format!("Unknown type suffix in: {}", term))
}

fn parse_function_params(
    raw_function_params: &[u8],
    params_format: &str,
) -> Result<Vec<i32>, String> {
    if params_format.is_empty() {
        if !raw_function_params.is_empty() {
            return Err("Expected empty params but got data".to_string());
        }
        return Ok(Vec::new());
    }

    let mut result = Vec::new();
    let mut offset = 0;

    for term in params_format.split(',') {
        let (param_type, bytes_needed) = parse_type_info(term)?;

        let data_slice = &raw_function_params[offset..offset + bytes_needed];
        result.extend(param_type.parse(data_slice));

        offset += bytes_needed;
    }

    if raw_function_params.len() != offset {
        return Err("Mismatch between expected and actual parameter length".to_string());
    }

    Ok(result)
}

/// A failure to parse an opcode's parameters. The raw hex line is still worth showing to the
/// user even when parsing failed, so it travels alongside the message as its own field rather
/// than being packed into the message string and split back out by the caller.
struct ParamParseError {
    message: String,
    raw_line: String,
}

/// Parse opcode parameters and return formatted code/raw lines
fn parse_opcode_parameters(
    file_contents: &[u8],
    offset: usize,
    num_bytes: u8,
    info: &OpcodeInfo,
) -> Result<(String, String), ParamParseError> {
    let raw_code_line = &file_contents[(offset - num_bytes as usize)..offset];
    let raw_function_param_values = &raw_code_line[1..raw_code_line.len()];

    let raw_line = format!("{:02x?}", raw_code_line);

    match parse_function_params(raw_function_param_values, &info.function_params) {
        Ok(params) => {
            let function_name = &info.name;
            let code_line = if params.is_empty() {
                format!("{}()", function_name)
            } else {
                format!("{}({:?})", function_name, params)
            };
            Ok((code_line, raw_line))
        }
        Err(e) => Err(ParamParseError {
            message: format!("Error parsing parameters: {}", e),
            raw_line,
        }),
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

                    match parse_opcode_parameters(file_contents, *offset, num_bytes, info) {
                        Ok((code_line, raw_line)) => {
                            // Sleep contains sleep and sleeping commands
                            // The sleep command is [0x9 0xa u8 u8], where 0x9 is the sleep command and 0xa is the sleeping command
                            if *x == Opcode::Sleep {
                                *offset -= (num_bytes - 1) as usize;
                            }
                            Ok((code_line, raw_line))
                        }
                        Err(e) => Ok((e.message, e.raw_line)),
                    }
                }
                None => Ok((
                    format!("Unknown opcode {}", opcode_byte),
                    format!("{:02x}", opcode_byte),
                )),
            }
        }
        None => Ok((
            format!("Unknown opcode {}", opcode_byte),
            format!("{:02x}", opcode_byte),
        )),
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

fn read_function_offsets(
    file_contents: &[u8],
    start_offset: u32,
) -> Result<Vec<u16>, &'static str> {
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

        let next_offset =
            u16::from_le_bytes([file_contents[offset_pos], file_contents[offset_pos + 1]]);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn param_type_byte_size() {
        assert_eq!(ParamType::U8.byte_size(), 1);
        assert_eq!(ParamType::I8.byte_size(), 1);
        assert_eq!(ParamType::U16.byte_size(), 2);
        assert_eq!(ParamType::I16.byte_size(), 2);
        assert_eq!(ParamType::U32.byte_size(), 4);
        assert_eq!(ParamType::I32.byte_size(), 4);
    }

    #[test]
    fn param_type_parse_widens_and_sign_extends() {
        assert_eq!(ParamType::U8.parse(&[0xff]), vec![255]);
        assert_eq!(ParamType::I8.parse(&[0xff]), vec![-1]);
        assert_eq!(ParamType::U16.parse(&[0xff, 0xff]), vec![65535]);
        assert_eq!(ParamType::I16.parse(&[0xff, 0xff]), vec![-1]);
        assert_eq!(ParamType::U32.parse(&[0x01, 0x00, 0x00, 0x00]), vec![1]);
        assert_eq!(ParamType::I32.parse(&[0xff, 0xff, 0xff, 0xff]), vec![-1]);
        // multiple back-to-back elements
        assert_eq!(ParamType::U8.parse(&[1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn parse_type_info_reads_quantity_and_type() {
        assert_eq!(parse_type_info("4u16").unwrap().1, 8);
        assert!(matches!(parse_type_info("4u16").unwrap().0, ParamType::U16));
        assert_eq!(parse_type_info("2i32").unwrap().1, 8);
        assert!(matches!(parse_type_info("2i32").unwrap().0, ParamType::I32));
        assert_eq!(parse_type_info("1u8").unwrap().1, 1);
        assert!(matches!(parse_type_info("1u8").unwrap().0, ParamType::U8));
    }

    #[test]
    fn parse_type_info_rejects_unknown_suffix() {
        assert!(parse_type_info("4u64").is_err());
        assert!(parse_type_info("").is_err());
    }

    #[test]
    fn parse_function_params_empty_format_expects_no_data() {
        assert_eq!(parse_function_params(&[], "").unwrap(), Vec::<i32>::new());
        assert!(parse_function_params(&[1, 2], "").is_err());
    }

    #[test]
    fn parse_function_params_reads_multiple_terms_in_order() {
        // "1u8,1u16" = one u8 byte followed by one little-endian u16
        let data = [0x05, 0x34, 0x12];
        let params = parse_function_params(&data, "1u8,1u16").unwrap();
        assert_eq!(params, vec![5, 0x1234]);
    }

    #[test]
    fn parse_function_params_rejects_length_mismatch() {
        // format expects 1 byte but 2 are given
        assert!(parse_function_params(&[1, 2], "1u8").is_err());
    }

    #[test]
    fn read_function_offsets_reads_self_terminating_table() {
        // The first u16 (4) is itself the byte-length of the offsets table, telling us the
        // table holds two entries (4 bytes / 2 bytes each): [4, 10].
        let data = [0x04, 0x00, 0x0a, 0x00];
        let offsets = read_function_offsets(&data, 0).unwrap();
        assert_eq!(offsets, vec![4, 10]);
    }

    #[test]
    fn read_function_offsets_reads_multiple_entries() {
        // Table length (6) covers 3 entries: [6, 8, 10].
        let data = [0x06, 0x00, 0x08, 0x00, 0x0a, 0x00];
        let offsets = read_function_offsets(&data, 0).unwrap();
        assert_eq!(offsets, vec![6, 8, 10]);
    }

    #[test]
    fn read_function_offsets_errors_when_data_too_short() {
        assert!(read_function_offsets(&[], 0).is_err());
        // Claims a table longer than the buffer actually holds.
        let data = [0x06, 0x00];
        assert!(read_function_offsets(&data, 0).is_err());
    }
}
