use num;
use num_derive::FromPrimitive;
use std::collections::HashMap;

#[derive(Eq, FromPrimitive, Hash, PartialEq)]
pub enum Opcode {
    NoOp = 0,
    EvtEnd = 1,
    EvtNext = 2,
    EvtChain = 3,
    EvtExec = 4,
    EvtKill = 5,
    IfStart = 6,
    ElseStart = 7,
    EndIf = 8,
    Sleep = 9,
    Sleeping = 10,
    Wsleep = 11,
    Wsleeping = 12,
    ForStart = 13,
    ForEnd = 14,
    WhileStart = 15,
    WhileEnd = 16,
    DoStart = 17,
    DoEnd = 18,
    Switch = 19,
    Case = 20,
    EndSwitch = 22,
    Goto = 23,
    GoSub = 24,
    Break = 26,
    WorkCopy = 29,
    CheckBit = 33,
    SetBit = 34,
    Compare = 35,
    Save = 36,
    Copy = 37,
    Calc = 38,
    SceRnd = 40,
    CutChg = 41,
    CutOld = 42,
    MessageOn = 43,
    AotSet = 44,
    ObjModelSet = 45,
    WorkSet = 46,
    SpeedSet = 47,
    AddSpeed = 48,
    AddAspeed = 49,
    PosSet = 50,
    DirSet = 51,
    MemberSet = 52,
    MemberSet2 = 53,
    SeOn = 54,
    ScaIdSet = 55,
    DirCk = 57,
    SceEsprOn = 58,
    DoorAotSet = 59,
    CutAuto = 60,
    MemberCopy = 61,
    MemberCmp = 62,
    PlcMotion = 63,
    PlcDest = 64,
    PlcNeck = 65,
    PlcRet = 66,
    SceEmSet = 68,
    AotReset = 70,
    AotOn = 71,
    CutReplace = 75,
    SceEsprKill = 76,
    ItemAotSet = 78,
    SceBgmControl = 81,
    SceEspr3dOn = 84,
    SceBgmtblSet = 87,
    PlcRot = 88,
    XaOn = 89,
    PlcCnt = 91,
    XaVol = 95,
    SceItemLost = 98,
    SceEsprOn2 = 100,
    PlcStop = 102,
    AotSet4p = 103,
    LightPosSet = 106,
    LightKidoSet = 107,
    PartsSet = 110,
    ScePartsBomb = 122,
    ScePartsDown = 123,
}

pub struct OpcodeInfo {
    pub instruction_size: u8,
    pub name: String,
}

fn init_opcode_info_map() -> HashMap<Opcode, OpcodeInfo> {
    let instruction_size: HashMap<Opcode, OpcodeInfo> = HashMap::from([
        (
            Opcode::NoOp,
            OpcodeInfo {
                instruction_size: 1,
                name: "NoOp".to_string(),
            },
        ),
        (
            Opcode::EvtEnd,
            OpcodeInfo {
                instruction_size: 1,
                name: "EvtEnd".to_string(),
            },
        ),
        (
            Opcode::EvtNext,
            OpcodeInfo {
                instruction_size: 1,
                name: "EvtNext".to_string(),
            },
        ),
        (
            Opcode::EvtChain,
            OpcodeInfo {
                instruction_size: 4,
                name: "EvtChain".to_string(),
            },
        ),
        (
            Opcode::EvtExec,
            OpcodeInfo {
                instruction_size: 4,
                name: "EvtExec".to_string(),
            },
        ),
        (
            Opcode::EvtKill,
            OpcodeInfo {
                instruction_size: 2,
                name: "EvtKill".to_string(),
            },
        ),
        (
            Opcode::IfStart,
            OpcodeInfo {
                instruction_size: 4,
                name: "IfStart".to_string(),
            },
        ),
        (
            Opcode::ElseStart,
            OpcodeInfo {
                instruction_size: 4,
                name: "ElseStart".to_string(),
            },
        ),
        (
            Opcode::EndIf,
            OpcodeInfo {
                instruction_size: 1,
                name: "EndIf".to_string(),
            },
        ),
        (
            Opcode::Sleep,
            OpcodeInfo {
                instruction_size: 4,
                name: "Sleep".to_string(),
            },
        ),
        // 10
        (
            Opcode::Sleeping,
            OpcodeInfo {
                instruction_size: 3,
                name: "Sleeping".to_string(),
            },
        ),
        (
            Opcode::Wsleep,
            OpcodeInfo {
                instruction_size: 1,
                name: "Wsleep".to_string(),
            },
        ),
        (
            Opcode::Wsleeping,
            OpcodeInfo {
                instruction_size: 1,
                name: "Wsleeping".to_string(),
            },
        ),
        (
            Opcode::ForStart,
            OpcodeInfo {
                instruction_size: 6,
                name: "ForStart".to_string(),
            },
        ),
        (
            Opcode::ForEnd,
            OpcodeInfo {
                instruction_size: 2,
                name: "ForEnd".to_string(),
            },
        ),
        (
            Opcode::WhileStart,
            OpcodeInfo {
                instruction_size: 4,
                name: "WhileStart".to_string(),
            },
        ),
        (
            Opcode::WhileEnd,
            OpcodeInfo {
                instruction_size: 2,
                name: "WhileEnd".to_string(),
            },
        ),
        (
            Opcode::DoStart,
            OpcodeInfo {
                instruction_size: 4,
                name: "DoStart".to_string(),
            },
        ),
        (
            Opcode::DoEnd,
            OpcodeInfo {
                instruction_size: 2,
                name: "DoEnd".to_string(),
            },
        ),
        (
            Opcode::Switch,
            OpcodeInfo {
                instruction_size: 4,
                name: "Switch".to_string(),
            },
        ),
        // 20
        (
            Opcode::Case,
            OpcodeInfo {
                instruction_size: 6,
                name: "Case".to_string(),
            },
        ),
        (
            Opcode::EndSwitch,
            OpcodeInfo {
                instruction_size: 2,
                name: "EndSwitch".to_string(),
            },
        ),
        (
            Opcode::Goto,
            OpcodeInfo {
                instruction_size: 6,
                name: "Goto".to_string(),
            },
        ),
        (
            Opcode::GoSub,
            OpcodeInfo {
                instruction_size: 2,
                name: "Gosub".to_string(),
            },
        ),
        (
            Opcode::Break,
            OpcodeInfo {
                instruction_size: 2,
                name: "Break".to_string(),
            },
        ),
        (
            Opcode::WorkCopy,
            OpcodeInfo {
                instruction_size: 4,
                name: "WorkCopy".to_string(),
            },
        ),
        (
            Opcode::CheckBit,
            OpcodeInfo {
                instruction_size: 4,
                name: "CheckBit".to_string(),
            },
        ),
        (
            Opcode::SetBit,
            OpcodeInfo {
                instruction_size: 4,
                name: "SetBit".to_string(),
            },
        ),
        (
            Opcode::Compare,
            OpcodeInfo {
                instruction_size: 6,
                name: "Compare".to_string(),
            },
        ),
        (
            Opcode::Save,
            OpcodeInfo {
                instruction_size: 4,
                name: "Save".to_string(),
            },
        ),
        (
            Opcode::Copy,
            OpcodeInfo {
                instruction_size: 3,
                name: "Copy".to_string(),
            },
        ),
        (
            Opcode::Calc,
            OpcodeInfo {
                instruction_size: 6,
                name: "Calc".to_string(),
            },
        ),
        // 40
        (
            Opcode::SceRnd,
            OpcodeInfo {
                instruction_size: 1,
                name: "SceRnd".to_string(),
            },
        ),
        (
            Opcode::CutChg,
            OpcodeInfo {
                instruction_size: 2,
                name: "CutChg".to_string(),
            },
        ),
        (
            Opcode::CutOld,
            OpcodeInfo {
                instruction_size: 1,
                name: "CutOld".to_string(),
            },
        ),
        (
            Opcode::MessageOn,
            OpcodeInfo {
                instruction_size: 6,
                name: "MessageOn".to_string(),
            },
        ),
        (
            Opcode::AotSet,
            OpcodeInfo {
                instruction_size: 20,
                name: "AotSet".to_string(),
            },
        ),
        (
            Opcode::ObjModelSet,
            OpcodeInfo {
                instruction_size: 38,
                name: "ObjModelSet".to_string(),
            },
        ),
        (
            Opcode::WorkSet,
            OpcodeInfo {
                instruction_size: 3,
                name: "WorkSet".to_string(),
            },
        ),
        (
            Opcode::SpeedSet,
            OpcodeInfo {
                instruction_size: 4,
                name: "SpeedSet".to_string(),
            },
        ),
        (
            Opcode::AddSpeed,
            OpcodeInfo {
                instruction_size: 1,
                name: "AddSpeed".to_string(),
            },
        ),
        (
            Opcode::AddAspeed,
            OpcodeInfo {
                instruction_size: 1,
                name: "AddAspeed".to_string(),
            },
        ),
        // 50
        (
            Opcode::PosSet,
            OpcodeInfo {
                instruction_size: 8,
                name: "PosSet".to_string(),
            },
        ),
        (
            Opcode::DirSet,
            OpcodeInfo {
                instruction_size: 8,
                name: "DirSet".to_string(),
            },
        ),
        (
            Opcode::MemberSet,
            OpcodeInfo {
                instruction_size: 4,
                name: "MemberSet".to_string(),
            },
        ),
        (
            Opcode::MemberSet2,
            OpcodeInfo {
                instruction_size: 3,
                name: "MemberSet2".to_string(),
            },
        ),
        (
            Opcode::SeOn,
            OpcodeInfo {
                instruction_size: 12,
                name: "SeOn".to_string(),
            },
        ),
        (
            Opcode::ScaIdSet,
            OpcodeInfo {
                instruction_size: 4,
                name: "ScaIdSet".to_string(),
            },
        ),
        (
            Opcode::DirCk,
            OpcodeInfo {
                instruction_size: 8,
                name: "DirCk".to_string(),
            },
        ),
        (
            Opcode::SceEsprOn,
            OpcodeInfo {
                instruction_size: 16,
                name: "SceEsprOn".to_string(),
            },
        ),
        (
            Opcode::DoorAotSet,
            OpcodeInfo {
                instruction_size: 32,
                name: "DoorAotSet".to_string(),
            },
        ),
        // 60
        (
            Opcode::CutAuto,
            OpcodeInfo {
                instruction_size: 2,
                name: "CutAuto".to_string(),
            },
        ),
        (
            Opcode::MemberCopy,
            OpcodeInfo {
                instruction_size: 3,
                name: "MemberCopy".to_string(),
            },
        ),
        (
            Opcode::MemberCmp,
            OpcodeInfo {
                instruction_size: 6,
                name: "MemberCmp".to_string(),
            },
        ),
        (
            Opcode::PlcMotion,
            OpcodeInfo {
                instruction_size: 4,
                name: "PlcMotion".to_string(),
            },
        ),
        (
            Opcode::PlcDest,
            OpcodeInfo {
                instruction_size: 8,
                name: "PlcDest".to_string(),
            },
        ),
        (
            Opcode::PlcNeck,
            OpcodeInfo {
                instruction_size: 10,
                name: "PlcNeck".to_string(),
            },
        ),
        (
            Opcode::PlcRet,
            OpcodeInfo {
                instruction_size: 1,
                name: "PlcRet".to_string(),
            },
        ),
        (
            Opcode::SceEmSet,
            OpcodeInfo {
                instruction_size: 22,
                name: "SceEmSet".to_string(),
            },
        ),
        // 70
        (
            Opcode::AotReset,
            OpcodeInfo {
                instruction_size: 10,
                name: "AotReset".to_string(),
            },
        ),
        (
            Opcode::AotOn,
            OpcodeInfo {
                instruction_size: 2,
                name: "AotOn".to_string(),
            },
        ),
        (
            Opcode::CutReplace,
            OpcodeInfo {
                instruction_size: 3,
                name: "CutReplace".to_string(),
            },
        ),
        (
            Opcode::SceEsprKill,
            OpcodeInfo {
                instruction_size: 5,
                name: "SceEsprKill".to_string(),
            },
        ),
        (
            Opcode::ItemAotSet,
            OpcodeInfo {
                instruction_size: 22,
                name: "ItemAotSet".to_string(),
            },
        ),
        (
            Opcode::SceBgmControl,
            OpcodeInfo {
                instruction_size: 6,
                name: "SceBgmControl".to_string(),
            },
        ),
        (
            Opcode::SceEspr3dOn,
            OpcodeInfo {
                instruction_size: 22,
                name: "SceEspr3dOn".to_string(),
            },
        ),
        (
            Opcode::SceBgmtblSet,
            OpcodeInfo {
                instruction_size: 8,
                name: "SceBgmtblSet".to_string(),
            },
        ),
        (
            Opcode::PlcRot,
            OpcodeInfo {
                instruction_size: 4,
                name: "PlcRot".to_string(),
            },
        ),
        (
            Opcode::XaOn,
            OpcodeInfo {
                instruction_size: 4,
                name: "XaOn".to_string(),
            },
        ),
        (
            Opcode::PlcCnt,
            OpcodeInfo {
                instruction_size: 2,
                name: "PlcCnt".to_string(),
            },
        ),
        (
            Opcode::XaVol,
            OpcodeInfo {
                instruction_size: 2,
                name: "XaVol".to_string(),
            },
        ),
        (
            Opcode::SceItemLost,
            OpcodeInfo {
                instruction_size: 2,
                name: "SceItemLost".to_string(),
            },
        ),
        // 100
        (
            Opcode::SceEsprOn2,
            OpcodeInfo {
                instruction_size: 16,
                name: "SceEsprOn2".to_string(),
            },
        ),
        (
            Opcode::PlcStop,
            OpcodeInfo {
                instruction_size: 1,
                name: "PlcStop".to_string(),
            },
        ),
        (
            Opcode::AotSet4p,
            OpcodeInfo {
                instruction_size: 28,
                name: "AotSet4p".to_string(),
            },
        ),
        (
            Opcode::LightPosSet,
            OpcodeInfo {
                instruction_size: 6,
                name: "LightPosSet".to_string(),
            },
        ),
        (
            Opcode::LightKidoSet,
            OpcodeInfo {
                instruction_size: 4,
                name: "LightKidoSet".to_string(),
            },
        ),
        (
            Opcode::PartsSet,
            OpcodeInfo {
                instruction_size: 6,
                name: "PartsSet".to_string(),
            },
        ),
        (
            Opcode::ScePartsBomb,
            OpcodeInfo {
                instruction_size: 16,
                name: "ScePartsBomb".to_string(),
            },
        ),
        (
            Opcode::ScePartsDown,
            OpcodeInfo {
                instruction_size: 16,
                name: "ScePartsDown".to_string(),
            },
        ),
    ]);
    instruction_size
}

pub fn init_opcode_documentation() -> HashMap<String, String> {
    let opcode_documentation: HashMap<String, String> = HashMap::from([
        ("AotSet".to_string(), "AotSet(aot: u8, id: u8, type: u8, floor: u8, super: u8, x: i16, z: i16, width: i16, depth: i16, data: u8[6])\n".to_owned() +
            &"Initialize aot object".to_string()),
    ]);
    opcode_documentation
}

// SCD file is within RDT
pub fn parse_rdt_scd_stream(file_contents: &[u8], start_offset: u32) -> (Vec<String>, String) {
    let opcode_info_map = init_opcode_info_map();

    let mut function_offsets = Vec::new();
    let first_offset = u16::from_le_bytes([
        file_contents[start_offset as usize],
        file_contents[(start_offset + 1) as usize],
    ]);
    function_offsets.push(first_offset);

    for i in (2..first_offset).step_by(2) {
        let next_offset = u16::from_le_bytes([
            file_contents[start_offset as usize + i as usize],
            file_contents[start_offset as usize + (i + 1) as usize],
        ]);
        function_offsets.push(next_offset);
    }

    let function_offsets_count = function_offsets.len();

    let mut code_lines = Vec::new();
    let mut raw_code_lines = Vec::new();

    for i in 0..function_offsets_count {
        let function_length = match i {
            i if i == function_offsets_count - 1 => file_contents.len() - start_offset as usize,
            _ => (function_offsets[i + 1] - function_offsets[i]).into(),
        };

        let mut function_cur_offset = start_offset as usize + function_offsets[i] as usize;
        code_lines.push(format!("Start Function {}:", i));
        raw_code_lines.push(format!("Start Function {}:", i));
        for _line_num in 0..function_length {
            let opcode_byte = file_contents[function_cur_offset];
            let opcode = &num::FromPrimitive::from_u8(opcode_byte);
            function_cur_offset += 1;
            match opcode {
                Some(x) => {
                    match opcode_info_map.get(x) {
                        Some(info) => {
                            let num_bytes = info.instruction_size;
                            function_cur_offset += (num_bytes - 1) as usize;

                            let function_params =
                                &file_contents[(function_cur_offset - (num_bytes as usize) + 1)
                                    ..function_cur_offset];
                            let function_name = info.name.clone();
                            code_lines.push(format!("{}({:?})", function_name, function_params));
                            raw_code_lines.push(format!(
                                "{:02x?}",
                                &file_contents[(function_cur_offset - (num_bytes as usize))
                                    ..function_cur_offset]
                            ));

                            // Sleep contains sleep and sleeping commands
                            // The sleep command is [0x9 0xa u8 u8], where 0x9 is the sleep command and 0xa is the sleeping command
                            if *x == Opcode::Sleep {
                                function_cur_offset -= (num_bytes - 1) as usize
                            }
                        }
                        None => {
                            code_lines.push(format!("Unknown opcode {}", opcode_byte));
                            raw_code_lines.push(format!("{:02x}", opcode_byte));
                            continue;
                        }
                    }
                }
                None => {
                    code_lines.push(format!("Unknown opcode {}", opcode_byte));
                    raw_code_lines.push(format!("{:02x}", opcode_byte));
                    continue;
                }
            }

            match opcode {
                Some(Opcode::EvtEnd) => break,
                Some(_) => {}
                None => {}
            }
        }
        code_lines.push(format!("End Function {}\n", i));
        raw_code_lines.push(format!("End Function {}\n", i));
    }
    (code_lines, raw_code_lines.join("\n"))
}
