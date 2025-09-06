use std::collections::HashMap;
use num_derive::FromPrimitive;

// Opcode name constants
pub const OPCODE_NO_OP: &str = "NoOp";
pub const OPCODE_EVT_END: &str = "EvtEnd";
pub const OPCODE_EVT_NEXT: &str = "EvtNext";
pub const OPCODE_EVT_CHAIN: &str = "EvtChain";
pub const OPCODE_EVT_EXEC: &str = "EvtExec";
pub const OPCODE_EVT_KILL: &str = "EvtKill";
pub const OPCODE_IF_START: &str = "IfStart";
pub const OPCODE_ELSE_START: &str = "ElseStart";
pub const OPCODE_END_IF: &str = "EndIf";
pub const OPCODE_SLEEP: &str = "Sleep";
pub const OPCODE_SLEEPING: &str = "Sleeping";
pub const OPCODE_WSLEEP: &str = "Wsleep";
pub const OPCODE_WSLEEPING: &str = "Wsleeping";
pub const OPCODE_FOR_START: &str = "ForStart";
pub const OPCODE_FOR_END: &str = "ForEnd";
pub const OPCODE_WHILE_START: &str = "WhileStart";
pub const OPCODE_WHILE_END: &str = "WhileEnd";
pub const OPCODE_DO_START: &str = "DoStart";
pub const OPCODE_DO_END: &str = "DoEnd";
pub const OPCODE_SWITCH: &str = "Switch";
pub const OPCODE_CASE: &str = "Case";
pub const OPCODE_END_SWITCH: &str = "EndSwitch";
pub const OPCODE_GOTO: &str = "Goto";
pub const OPCODE_GO_SUB: &str = "GoSub";
pub const OPCODE_BREAK: &str = "Break";
pub const OPCODE_WORK_COPY: &str = "WorkCopy";
pub const OPCODE_CHECK_BIT: &str = "CheckBit";
pub const OPCODE_SET_BIT: &str = "SetBit";
pub const OPCODE_COMPARE: &str = "Compare";
pub const OPCODE_SAVE: &str = "Save";
pub const OPCODE_COPY: &str = "Copy";
pub const OPCODE_CALC: &str = "Calc";
pub const OPCODE_SCE_RND: &str = "SceRnd";
pub const OPCODE_CUT_CHG: &str = "CutChg";
pub const OPCODE_CUT_OLD: &str = "CutOld";
pub const OPCODE_MESSAGE_ON: &str = "MessageOn";
pub const OPCODE_AOT_SET: &str = "AotSet";
pub const OPCODE_OBJ_MODEL_SET: &str = "ObjModelSet";
pub const OPCODE_WORK_SET: &str = "WorkSet";
pub const OPCODE_SPEED_SET: &str = "SpeedSet";
pub const OPCODE_ADD_SPEED: &str = "AddSpeed";
pub const OPCODE_ADD_ASPEED: &str = "AddAspeed";
pub const OPCODE_POS_SET: &str = "PosSet";
pub const OPCODE_DIR_SET: &str = "DirSet";
pub const OPCODE_MEMBER_SET: &str = "MemberSet";
pub const OPCODE_MEMBER_SET2: &str = "MemberSet2";
pub const OPCODE_SE_ON: &str = "SeOn";
pub const OPCODE_SCA_ID_SET: &str = "ScaIdSet";
pub const OPCODE_DIR_CK: &str = "DirCk";
pub const OPCODE_SCE_ESPR_ON: &str = "SceEsprOn";
pub const OPCODE_DOOR_AOT_SET: &str = "DoorAotSet";
pub const OPCODE_CUT_AUTO: &str = "CutAuto";
pub const OPCODE_MEMBER_COPY: &str = "MemberCopy";
pub const OPCODE_MEMBER_CMP: &str = "MemberCmp";
pub const OPCODE_PLC_MOTION: &str = "PlcMotion";
pub const OPCODE_PLC_DEST: &str = "PlcDest";
pub const OPCODE_PLC_NECK: &str = "PlcNeck";
pub const OPCODE_PLC_RET: &str = "PlcRet";
pub const OPCODE_SCE_EM_SET: &str = "SceEmSet";
pub const OPCODE_AOT_RESET: &str = "AotReset";
pub const OPCODE_AOT_ON: &str = "AotOn";
pub const OPCODE_CUT_REPLACE: &str = "CutReplace";
pub const OPCODE_SCE_ESPR_KILL: &str = "SceEsprKill";
pub const OPCODE_ITEM_AOT_SET: &str = "ItemAotSet";
pub const OPCODE_SCE_BGM_CONTROL: &str = "SceBgmControl";
pub const OPCODE_SCE_ESPR3D_ON: &str = "SceEspr3dOn";
pub const OPCODE_SCE_BGMTBL_SET: &str = "SceBgmtblSet";
pub const OPCODE_PLC_ROT: &str = "PlcRot";
pub const OPCODE_XA_ON: &str = "XaOn";
pub const OPCODE_PLC_CNT: &str = "PlcCnt";
pub const OPCODE_MIZU_DIV_SET: &str = "MizuDivSet";
pub const OPCODE_XA_VOL: &str = "XaVol";
pub const OPCODE_KAGE_SET: &str = "KageSet";
pub const OPCODE_CUT_BE_SET: &str = "CutBeSet";
pub const OPCODE_SCE_ITEM_LOST: &str = "SceItemLost";
pub const OPCODE_SCE_ESPR_ON2: &str = "SceEsprOn2";
pub const OPCODE_PLC_STOP: &str = "PlcStop";
pub const OPCODE_AOT_SET4P: &str = "AotSet4p";
pub const OPCODE_LIGHT_POS_SET: &str = "LightPosSet";
pub const OPCODE_LIGHT_KIDO_SET: &str = "LightKidoSet";
pub const OPCODE_PARTS_SET: &str = "PartsSet";
pub const OPCODE_SCE_PARTS_BOMB: &str = "ScePartsBomb";
pub const OPCODE_SCE_PARTS_DOWN: &str = "ScePartsDown";

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
    MizuDivSet = 93,
    XaVol = 95,
    KageSet = 96,
    CutBeSet = 97,
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
    pub function_params: String,
}

pub fn init_opcode_info_map() -> HashMap<Opcode, OpcodeInfo> {
    let instruction_size: HashMap<Opcode, OpcodeInfo> = HashMap::from([
        (
            Opcode::NoOp,
            OpcodeInfo {
                instruction_size: 1,
                name: OPCODE_NO_OP.to_string(),
                function_params: "".to_string(),
            },
        ),
        (
            Opcode::EvtEnd,
            OpcodeInfo {
                instruction_size: 1,
                name: OPCODE_EVT_END.to_string(),
                function_params: "".to_string(),
            },
        ),
        (
            Opcode::EvtNext,
            OpcodeInfo {
                instruction_size: 1,
                name: OPCODE_EVT_NEXT.to_string(),
                function_params: "".to_string(),
            },
        ),
        (
            Opcode::EvtChain,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_EVT_CHAIN.to_string(),
                function_params: "3u8".to_string(),
            },
        ),
        (
            Opcode::EvtExec,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_EVT_EXEC.to_string(),
                function_params: "3u8".to_string(),
            },
        ),
        (
            Opcode::EvtKill,
            OpcodeInfo {
                instruction_size: 2,
                name: OPCODE_EVT_KILL.to_string(),
                function_params: "1u8".to_string(),
            },
        ),
        (
            Opcode::IfStart,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_IF_START.to_string(),
                function_params: "1u8,1u16".to_string(),
            },
        ),
        (
            Opcode::ElseStart,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_ELSE_START.to_string(),
                function_params: "1u8,1u16".to_string(),
            },
        ),
        (
            Opcode::EndIf,
            OpcodeInfo {
                instruction_size: 1,
                name: OPCODE_END_IF.to_string(),
                function_params: "".to_string(),
            },
        ),
        (
            Opcode::Sleep,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_SLEEP.to_string(),
                function_params: "1u8,1u16".to_string(),
            },
        ),
        // 10
        (
            Opcode::Sleeping,
            OpcodeInfo {
                instruction_size: 3,
                name: OPCODE_SLEEPING.to_string(),
                function_params: "2u8".to_string(),
            },
        ),
        (
            Opcode::Wsleep,
            OpcodeInfo {
                instruction_size: 1,
                name: OPCODE_WSLEEP.to_string(),
                function_params: "".to_string(),
            },
        ),
        (
            Opcode::Wsleeping,
            OpcodeInfo {
                instruction_size: 1,
                name: OPCODE_WSLEEPING.to_string(),
                function_params: "".to_string(),
            },
        ),
        (
            Opcode::ForStart,
            OpcodeInfo {
                instruction_size: 6,
                name: "ForStart".to_string(),
                function_params: "1u8,2u16".to_string(),
            },
        ),
        (
            Opcode::ForEnd,
            OpcodeInfo {
                instruction_size: 2,
                name: "ForEnd".to_string(),
                function_params: "1u8".to_string(),
            },
        ),
        (
            Opcode::WhileStart,
            OpcodeInfo {
                instruction_size: 4,
                name: "WhileStart".to_string(),
                function_params: "1u8,1u16".to_string(),
            },
        ),
        (
            Opcode::WhileEnd,
            OpcodeInfo {
                instruction_size: 2,
                name: "WhileEnd".to_string(),
                function_params: "1u8".to_string(),
            },
        ),
        (
            Opcode::DoStart,
            OpcodeInfo {
                instruction_size: 4,
                name: "DoStart".to_string(),
                function_params: "1u8,1u16".to_string(),
            },
        ),
        (
            Opcode::DoEnd,
            OpcodeInfo {
                instruction_size: 2,
                name: "DoEnd".to_string(),
                function_params: "1u8".to_string(),
            },
        ),
        (
            Opcode::Switch,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_SWITCH.to_string(),
                function_params: "1u8,1u16".to_string(),
            },
        ),
        // 20
        (
            Opcode::Case,
            OpcodeInfo {
                instruction_size: 6,
                name: OPCODE_CASE.to_string(),
                function_params: "1u8,2u16".to_string(),
            },
        ),
        (
            Opcode::EndSwitch,
            OpcodeInfo {
                instruction_size: 2,
                name: OPCODE_END_SWITCH.to_string(),
                function_params: "1u8".to_string(),
            },
        ),
        (
            Opcode::Goto,
            OpcodeInfo {
                instruction_size: 6,
                name: OPCODE_GOTO.to_string(),
                function_params: "2i8,1u8,1i16".to_string(),
            },
        ),
        (
            Opcode::GoSub,
            OpcodeInfo {
                instruction_size: 2,
                name: OPCODE_GO_SUB.to_string(),
                function_params: "1u8".to_string(),
            },
        ),
        (
            Opcode::Break,
            OpcodeInfo {
                instruction_size: 2,
                name: OPCODE_BREAK.to_string(),
                function_params: "1u8".to_string(),
            },
        ),
        (
            Opcode::WorkCopy,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_WORK_COPY.to_string(),
                function_params: "3u8".to_string(),
            },
        ),
        (
            Opcode::CheckBit,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_CHECK_BIT.to_string(),
                function_params: "3u8".to_string(),
            },
        ),
        (
            Opcode::SetBit,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_SET_BIT.to_string(),
                function_params: "3u8".to_string(),
            },
        ),
        (
            Opcode::Compare,
            OpcodeInfo {
                instruction_size: 6,
                name: OPCODE_COMPARE.to_string(),
                function_params: "3u8,1i16".to_string(),
            },
        ),
        (
            Opcode::Save,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_SAVE.to_string(),
                function_params: "1u8,1i16".to_string(),
            },
        ),
        (
            Opcode::Copy,
            OpcodeInfo {
                instruction_size: 3,
                name: OPCODE_COPY.to_string(),
                function_params: "2u8".to_string(),
            },
        ),
        (
            Opcode::Calc,
            OpcodeInfo {
                instruction_size: 6,
                name: OPCODE_CALC.to_string(),
                function_params: "3u8,1i16".to_string(),
            },
        ),
        // 40
        (
            Opcode::SceRnd,
            OpcodeInfo {
                instruction_size: 1,
                name: OPCODE_SCE_RND.to_string(),
                function_params: "".to_string(),
            },
        ),
        (
            Opcode::CutChg,
            OpcodeInfo {
                instruction_size: 2,
                name: OPCODE_CUT_CHG.to_string(),
                function_params: "1u8".to_string(),
            },
        ),
        (
            Opcode::CutOld,
            OpcodeInfo {
                instruction_size: 1,
                name: OPCODE_CUT_OLD.to_string(),
                function_params: "".to_string(),
            },
        ),
        (
            Opcode::MessageOn,
            OpcodeInfo {
                instruction_size: 6,
                name: OPCODE_MESSAGE_ON.to_string(),
                function_params: "5u8".to_string(),
            },
        ),
        (
            Opcode::AotSet,
            OpcodeInfo {
                instruction_size: 20,
                name: OPCODE_AOT_SET.to_string(),
                function_params: "5u8,4i16,6u8".to_string(),
            },
        ),
        (
            Opcode::ObjModelSet,
            OpcodeInfo {
                instruction_size: 38,
                name: OPCODE_OBJ_MODEL_SET.to_string(),
                function_params: "7u8,2u16,10i16,3u16".to_string(),
            },
        ),
        (
            Opcode::WorkSet,
            OpcodeInfo {
                instruction_size: 3,
                name: OPCODE_WORK_SET.to_string(),
                function_params: "2u8".to_string(),
            },
        ),
        (
            Opcode::SpeedSet,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_SPEED_SET.to_string(),
                function_params: "3u8".to_string(),
            },
        ),
        (
            Opcode::AddSpeed,
            OpcodeInfo {
                instruction_size: 1,
                name: OPCODE_ADD_SPEED.to_string(),
                function_params: "".to_string(),
            },
        ),
        (
            Opcode::AddAspeed,
            OpcodeInfo {
                instruction_size: 1,
                name: OPCODE_ADD_ASPEED.to_string(),
                function_params: "".to_string(),
            },
        ),
        // 50
        (
            Opcode::PosSet,
            OpcodeInfo {
                instruction_size: 8,
                name: OPCODE_POS_SET.to_string(),
                function_params: "1u8,3i16".to_string(),
            },
        ),
        (
            Opcode::DirSet,
            OpcodeInfo {
                instruction_size: 8,
                name: OPCODE_DIR_SET.to_string(),
                function_params: "7u8".to_string(),
            },
        ),
        (
            Opcode::MemberSet,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_MEMBER_SET.to_string(),
                function_params: "1u8,1u16".to_string(),
            },
        ),
        (
            Opcode::MemberSet2,
            OpcodeInfo {
                instruction_size: 3,
                name: OPCODE_MEMBER_SET2.to_string(),
                function_params: "2u8".to_string(),
            },
        ),
        (
            Opcode::SeOn,
            OpcodeInfo {
                instruction_size: 12,
                name: OPCODE_SE_ON.to_string(),
                function_params: "11u8".to_string(),
            },
        ),
        (
            Opcode::ScaIdSet,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_SCA_ID_SET.to_string(),
                function_params: "1u8,1u16".to_string(),
            },
        ),
        (
            Opcode::DirCk,
            OpcodeInfo {
                instruction_size: 8,
                name: OPCODE_DIR_CK.to_string(),
                function_params: "7u8".to_string(),
            },
        ),
        (
            Opcode::SceEsprOn,
            OpcodeInfo {
                instruction_size: 16,
                name: OPCODE_SCE_ESPR_ON.to_string(),
                function_params: "3u8,1u16,4i16,1u16".to_string(),
            },
        ),
        (
            Opcode::DoorAotSet,
            OpcodeInfo {
                instruction_size: 32,
                name: OPCODE_DOOR_AOT_SET.to_string(),
                function_params: "5u8,8i16,10u8".to_string(),
            },
        ),
        // 60
        (
            Opcode::CutAuto,
            OpcodeInfo {
                instruction_size: 2,
                name: OPCODE_CUT_AUTO.to_string(),
                function_params: "1u8".to_string(),
            },
        ),
        (
            Opcode::MemberCopy,
            OpcodeInfo {
                instruction_size: 3,
                name: OPCODE_MEMBER_COPY.to_string(),
                function_params: "2u8".to_string(),
            },
        ),
        (
            Opcode::MemberCmp,
            OpcodeInfo {
                instruction_size: 6,
                name: OPCODE_MEMBER_CMP.to_string(),
                function_params: "3u8,1i16".to_string(),
            },
        ),
        (
            Opcode::PlcMotion,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_PLC_MOTION.to_string(),
                function_params: "3u8".to_string(),
            },
        ),
        (
            Opcode::PlcDest,
            OpcodeInfo {
                instruction_size: 8,
                name: OPCODE_PLC_DEST.to_string(),
                function_params: "3u8,2i16".to_string(),
            },
        ),
        (
            Opcode::PlcNeck,
            OpcodeInfo {
                instruction_size: 10,
                name: OPCODE_PLC_NECK.to_string(),
                function_params: "1u8,3i16,2i8".to_string(),
            },
        ),
        (
            Opcode::PlcRet,
            OpcodeInfo {
                instruction_size: 1,
                name: OPCODE_PLC_RET.to_string(),
                function_params: "".to_string(),
            },
        ),
        (
            Opcode::SceEmSet,
            OpcodeInfo {
                instruction_size: 22,
                name: OPCODE_SCE_EM_SET.to_string(),
                function_params: "8u8,1i8,3i16,3u16".to_string(),
            },
        ),
        // 70
        (
            Opcode::AotReset,
            OpcodeInfo {
                instruction_size: 10,
                name: OPCODE_AOT_RESET.to_string(),
                function_params: "9u8".to_string(),
            },
        ),
        (
            Opcode::AotOn,
            OpcodeInfo {
                instruction_size: 2,
                name: OPCODE_AOT_ON.to_string(),
                function_params: "1u8".to_string(),
            },
        ),
        (
            Opcode::CutReplace,
            OpcodeInfo {
                instruction_size: 3,
                name: OPCODE_CUT_REPLACE.to_string(),
                function_params: "2u8".to_string(),
            },
        ),
        (
            Opcode::SceEsprKill,
            OpcodeInfo {
                instruction_size: 5,
                name: OPCODE_SCE_ESPR_KILL.to_string(),
                function_params: "4u8".to_string(),
            },
        ),
        (
            Opcode::ItemAotSet,
            OpcodeInfo {
                instruction_size: 22,
                name: OPCODE_ITEM_AOT_SET.to_string(),
                function_params: "5u8,4i16,3u16,2u8".to_string(),
            },
        ),
        (
            Opcode::SceBgmControl,
            OpcodeInfo {
                instruction_size: 6,
                name: OPCODE_SCE_BGM_CONTROL.to_string(),
                function_params: "5u8".to_string(),
            },
        ),
        (
            Opcode::SceEspr3dOn,
            OpcodeInfo {
                instruction_size: 22,
                name: OPCODE_SCE_ESPR3D_ON.to_string(),
                function_params: "1u8,3u16,6i16,1u16".to_string(),
            },
        ),
        (
            Opcode::SceBgmtblSet,
            OpcodeInfo {
                instruction_size: 8,
                name: OPCODE_SCE_BGMTBL_SET.to_string(),
                function_params: "7u8".to_string(),
            },
        ),
        (
            Opcode::PlcRot,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_PLC_ROT.to_string(),
                function_params: "1u8,1i16".to_string(),
            },
        ),
        (
            Opcode::XaOn,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_XA_ON.to_string(),
                function_params: "1u8,1i16".to_string(),
            },
        ),
        (
            Opcode::PlcCnt,
            OpcodeInfo {
                instruction_size: 2,
                name: OPCODE_PLC_CNT.to_string(),
                function_params: "1u8".to_string(),
            },
        ),
        (
            Opcode::MizuDivSet,
            OpcodeInfo {
                instruction_size: 2,
                name: OPCODE_MIZU_DIV_SET.to_string(),
                function_params: "1u8".to_string(),
            },
        ),
        (
            Opcode::XaVol,
            OpcodeInfo {
                instruction_size: 2,
                name: OPCODE_XA_VOL.to_string(),
                function_params: "1u8".to_string(),
            },
        ),
        (
            Opcode::KageSet,
            OpcodeInfo {
                instruction_size: 14,
                name: OPCODE_KAGE_SET.to_string(),
                function_params: "5u8,4i16".to_string(),
            },
        ),
        (
            Opcode::CutBeSet,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_CUT_BE_SET.to_string(),
                function_params: "3u8".to_string(),
            },
        ),
        (
            Opcode::SceItemLost,
            OpcodeInfo {
                instruction_size: 2,
                name: OPCODE_SCE_ITEM_LOST.to_string(),
                function_params: "1u8".to_string(),
            },
        ),
        // 100
        (
            Opcode::SceEsprOn2,
            OpcodeInfo {
                instruction_size: 16,
                name: OPCODE_SCE_ESPR_ON2.to_string(),
                function_params: "15u8".to_string(),
            },
        ),
        (
            Opcode::PlcStop,
            OpcodeInfo {
                instruction_size: 1,
                name: OPCODE_PLC_STOP.to_string(),
                function_params: "".to_string(),
            },
        ),
        (
            Opcode::AotSet4p,
            OpcodeInfo {
                instruction_size: 28,
                name: OPCODE_AOT_SET4P.to_string(),
                function_params: "5u8,8i16,6u8".to_string(),
            },
        ),
        (
            Opcode::LightPosSet,
            OpcodeInfo {
                instruction_size: 6,
                name: OPCODE_LIGHT_POS_SET.to_string(),
                function_params: "5u8".to_string(),
            },
        ),
        (
            Opcode::LightKidoSet,
            OpcodeInfo {
                instruction_size: 4,
                name: OPCODE_LIGHT_KIDO_SET.to_string(),
                function_params: "3u8".to_string(),
            },
        ),
        (
            Opcode::PartsSet,
            OpcodeInfo {
                instruction_size: 6,
                name: OPCODE_PARTS_SET.to_string(),
                function_params: "5u8".to_string(),
            },
        ),
        (
            Opcode::ScePartsBomb,
            OpcodeInfo {
                instruction_size: 16,
                name: OPCODE_SCE_PARTS_BOMB.to_string(),
                function_params: "15u8".to_string(),
            },
        ),
        (
            Opcode::ScePartsDown,
            OpcodeInfo {
                instruction_size: 16,
                name: OPCODE_SCE_PARTS_DOWN.to_string(),
                function_params: "15u8".to_string(),
            },
        ),
    ]);
    instruction_size
}

pub fn init_opcode_documentation() -> HashMap<String, String> {
    let opcode_documentation: HashMap<String, String> = HashMap::from([
        (OPCODE_AOT_SET.to_string(), "AotSet(aot: u8, id: u8, type: u8, floor: u8, super: u8, x: i16, z: i16, width: i16, depth: i16, data: u8[6])\n".to_owned() +
            &"Initialize aot object as a rectangle with a point and dimensions".to_string()),
        (OPCODE_POS_SET.to_string(), "PosSet(dummy: u8, x: i16, y: i16, z: i16)\n".to_owned() +
            &"Set position of work object".to_string()),
        (OPCODE_SCE_EM_SET.to_string(), "SceEmSet(dummy: u8, aot: u8, id: u8, type: u8, status: u8, floor: u8, soundFlag: u8, modelType: u8, emSetFlag: i8, x: i16, y: i16, z: i16, dirY: u16, motion: u16, ctrFlag: u16)\n".to_owned() +
            &"Initialize animated entity".to_string()),
        (OPCODE_AOT_SET4P.to_string(), "AotSet4p(aot: u8, id: u8, type: u8, floor: u8, super: u8, x1: i16, z1: i16, x2: i16, z2: i16, x3: i16, z3: i16, x4: i16, z4: i16, data: u8[6])\n".to_owned() +
            &"Initialize aot object with 4 points".to_string()),
        (OPCODE_EVT_END.to_string(), "EvtEnd()\n".to_owned() +
            &"End event execution".to_string()),
        (OPCODE_EVT_NEXT.to_string(), "EvtNext()\n".to_owned() +
            &"Continue to next event".to_string()),
        (OPCODE_EVT_CHAIN.to_string(), "EvtChain(param1: u8, param2: u8, param3: u8)\n".to_owned() +
            &"Chain to another event".to_string()),
        (OPCODE_EVT_EXEC.to_string(), "EvtExec(param1: u8, param2: u8, param3: u8)\n".to_owned() +
            &"Execute another event".to_string()),
        (OPCODE_EVT_KILL.to_string(), "EvtKill(param1: u8)\n".to_owned() +
            &"Kill an event".to_string()),
        (OPCODE_IF_START.to_string(), "IfStart(param1: u8, param2: u16)\n".to_owned() +
            &"Start conditional block".to_string()),
        (OPCODE_ELSE_START.to_string(), "ElseStart(param1: u8, param2: u16)\n".to_owned() +
            &"Start else block".to_string()),
        (OPCODE_END_IF.to_string(), "EndIf()\n".to_owned() +
            &"End conditional block".to_string()),
        (OPCODE_SLEEP.to_string(), "Sleep(param1: u8, param2: u16)\n".to_owned() +
            &"Sleep for specified time".to_string()),
        (OPCODE_SLEEPING.to_string(), "Sleeping(param1: u8, param2: u8)\n".to_owned() +
            &"Check if sleeping".to_string()),
        (OPCODE_WSLEEP.to_string(), "Wsleep()\n".to_owned() +
            &"Wait sleep".to_string()),
        (OPCODE_WSLEEPING.to_string(), "Wsleeping()\n".to_owned() +
            &"Check if wait sleeping".to_string()),
        (OPCODE_FOR_START.to_string(), "ForStart(param1: u8, param2: u16, param3: u16)\n".to_owned() +
            &"Start for loop".to_string()),
        (OPCODE_FOR_END.to_string(), "ForEnd(param1: u8)\n".to_owned() +
            &"End for loop".to_string()),
        (OPCODE_WHILE_START.to_string(), "WhileStart(param1: u8, param2: u16)\n".to_owned() +
            &"Start while loop".to_string()),
        (OPCODE_WHILE_END.to_string(), "WhileEnd(param1: u8)\n".to_owned() +
            &"End while loop".to_string()),
        (OPCODE_DO_START.to_string(), "DoStart(param1: u8, param2: u16)\n".to_owned() +
            &"Start do-while loop".to_string()),
        (OPCODE_DO_END.to_string(), "DoEnd(param1: u8)\n".to_owned() +
            &"End do-while loop".to_string()),
        (OPCODE_SWITCH.to_string(), "Switch(param1: u8, param2: u16)\n".to_owned() +
            &"Start switch statement".to_string()),
        (OPCODE_CASE.to_string(), "Case(param1: u8, param2: u16, param3: u16)\n".to_owned() +
            &"Case in switch statement".to_string()),
        (OPCODE_END_SWITCH.to_string(), "EndSwitch(param1: u8)\n".to_owned() +
            &"End switch statement".to_string()),
        (OPCODE_GOTO.to_string(), "Goto(param1: i8, param2: i8, param3: u8, param4: i16)\n".to_owned() +
            &"Jump to label".to_string()),
        (OPCODE_GO_SUB.to_string(), "GoSub(param1: u8)\n".to_owned() +
            &"Call subroutine".to_string()),
        (OPCODE_BREAK.to_string(), "Break(param1: u8)\n".to_owned() +
            &"Break from loop/switch".to_string()),
        (OPCODE_WORK_COPY.to_string(), "WorkCopy(param1: u8, param2: u8, param3: u8)\n".to_owned() +
            &"Copy work values".to_string()),
        (OPCODE_CHECK_BIT.to_string(), "CheckBit(param1: u8, param2: u8, param3: u8)\n".to_owned() +
            &"Check bit flag".to_string()),
        (OPCODE_SET_BIT.to_string(), "SetBit(param1: u8, param2: u8, param3: u8)\n".to_owned() +
            &"Set bit flag".to_string()),
        (OPCODE_COMPARE.to_string(), "Compare(param1: u8, param2: u8, param3: u8, param4: i16)\n".to_owned() +
            &"Compare values".to_string()),
        (OPCODE_SAVE.to_string(), "Save(param1: u8, param2: i16)\n".to_owned() +
            &"Save value".to_string()),
        (OPCODE_COPY.to_string(), "Copy(param1: u8, param2: u8)\n".to_owned() +
            &"Copy value".to_string()),
        (OPCODE_CALC.to_string(), "Calc(param1: u8, param2: u8, param3: u8, param4: i16)\n".to_owned() +
            &"Calculate expression".to_string()),
        (OPCODE_SCE_RND.to_string(), "SceRnd()\n".to_owned() +
            &"Generate random number".to_string()),
        (OPCODE_CUT_CHG.to_string(), "CutChg(param1: u8)\n".to_owned() +
            &"Change cutscene".to_string()),
        (OPCODE_CUT_OLD.to_string(), "CutOld()\n".to_owned() +
            &"Restore old cutscene".to_string()),
        (OPCODE_MESSAGE_ON.to_string(), "MessageOn(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8)\n".to_owned() +
            &"Display message".to_string()),
        (OPCODE_OBJ_MODEL_SET.to_string(), "ObjModelSet(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8, param6: u8, param7: u8, param8: u16, param9: u16, param10: i16, param11: i16, param12: i16, param13: i16, param14: i16, param15: i16, param16: i16, param17: i16, param18: i16, param19: i16, param20: i16, param21: u16, param22: u16, param23: u16)\n".to_owned() +
            &"Set object model".to_string()),
        (OPCODE_WORK_SET.to_string(), "WorkSet(param1: u8, param2: u8)\n".to_owned() +
            &"Set work value".to_string()),
        (OPCODE_SPEED_SET.to_string(), "SpeedSet(param1: u8, param2: u8, param3: u8)\n".to_owned() +
            &"Set speed".to_string()),
        (OPCODE_ADD_SPEED.to_string(), "AddSpeed()\n".to_owned() +
            &"Add to speed".to_string()),
        (OPCODE_ADD_ASPEED.to_string(), "AddAspeed()\n".to_owned() +
            &"Add to angular speed".to_string()),
        (OPCODE_DIR_SET.to_string(), "DirSet(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8, param6: u8, param7: u8)\n".to_owned() +
            &"Set direction".to_string()),
        (OPCODE_MEMBER_SET.to_string(), "MemberSet(param1: u8, param2: u16)\n".to_owned() +
            &"Set member value".to_string()),
        (OPCODE_MEMBER_SET2.to_string(), "MemberSet2(param1: u8, param2: u8)\n".to_owned() +
            &"Set member value (variant 2)".to_string()),
        (OPCODE_SE_ON.to_string(), "SeOn(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8, param6: u8, param7: u8, param8: u8, param9: u8, param10: u8, param11: u8)\n".to_owned() +
            &"Play sound effect".to_string()),
        (OPCODE_SCA_ID_SET.to_string(), "ScaIdSet(param1: u8, param2: u16)\n".to_owned() +
            &"Set scale ID".to_string()),
        (OPCODE_DIR_CK.to_string(), "DirCk(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8, param6: u8, param7: u8)\n".to_owned() +
            &"Check direction".to_string()),
        (OPCODE_SCE_ESPR_ON.to_string(), "SceEsprOn(param1: u8, param2: u8, param3: u8, param4: u16, param5: i16, param6: i16, param7: i16, param8: i16, param9: u16)\n".to_owned() +
            &"Enable sprite effect".to_string()),
        (OPCODE_DOOR_AOT_SET.to_string(), "DoorAotSet(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8, param6: i16, param7: i16, param8: i16, param9: i16, param10: i16, param11: i16, param12: i16, param13: i16, param14: u8, param15: u8, param16: u8, param17: u8, param18: u8, param19: u8, param20: u8, param21: u8, param22: u8, param23: u8)\n".to_owned() +
            &"Set door area of trigger".to_string()),
        (OPCODE_CUT_AUTO.to_string(), "CutAuto(param1: u8)\n".to_owned() +
            &"Auto cutscene".to_string()),
        (OPCODE_MEMBER_COPY.to_string(), "MemberCopy(param1: u8, param2: u8)\n".to_owned() +
            &"Copy member value".to_string()),
        (OPCODE_MEMBER_CMP.to_string(), "MemberCmp(param1: u8, param2: u8, param3: u8, param4: i16)\n".to_owned() +
            &"Compare member values".to_string()),
        (OPCODE_PLC_MOTION.to_string(), "PlcMotion(param1: u8, param2: u8, param3: u8)\n".to_owned() +
            &"Set player motion".to_string()),
        (OPCODE_PLC_DEST.to_string(), "PlcDest(param1: u8, param2: u8, param3: u8, param4: i16, param5: i16)\n".to_owned() +
            &"Set player destination".to_string()),
        (OPCODE_PLC_NECK.to_string(), "PlcNeck(param1: u8, param2: i16, param3: i16, param4: i16, param5: i8, param6: i8)\n".to_owned() +
            &"Set player neck direction".to_string()),
        (OPCODE_PLC_RET.to_string(), "PlcRet()\n".to_owned() +
            &"Return player".to_string()),
        (OPCODE_AOT_RESET.to_string(), "AotReset(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8, param6: u8, param7: u8, param8: u8, param9: u8)\n".to_owned() +
            &"Reset area of trigger".to_string()),
        (OPCODE_AOT_ON.to_string(), "AotOn(param1: u8)\n".to_owned() +
            &"Enable area of trigger".to_string()),
        (OPCODE_CUT_REPLACE.to_string(), "CutReplace(param1: u8, param2: u8)\n".to_owned() +
            &"Replace cutscene".to_string()),
        (OPCODE_SCE_ESPR_KILL.to_string(), "SceEsprKill(param1: u8, param2: u8, param3: u8, param4: u8)\n".to_owned() +
            &"Kill sprite effect".to_string()),
        (OPCODE_ITEM_AOT_SET.to_string(), "ItemAotSet(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8, param6: i16, param7: i16, param8: i16, param9: i16, param10: u16, param11: u16, param12: u16, param13: u8, param14: u8)\n".to_owned() +
            &"Set item area of trigger".to_string()),
        (OPCODE_SCE_BGM_CONTROL.to_string(), "SceBgmControl(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8)\n".to_owned() +
            &"Control background music".to_string()),
        (OPCODE_SCE_ESPR3D_ON.to_string(), "SceEspr3dOn(param1: u8, param2: u16, param3: u16, param4: u16, param5: i16, param6: i16, param7: i16, param8: i16, param9: i16, param10: i16, param11: u16)\n".to_owned() +
            &"Enable 3D sprite effect".to_string()),
        (OPCODE_SCE_BGMTBL_SET.to_string(), "SceBgmtblSet(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8, param6: u8, param7: u8)\n".to_owned() +
            &"Set background music table".to_string()),
        (OPCODE_PLC_ROT.to_string(), "PlcRot(param1: u8, param2: i16)\n".to_owned() +
            &"Rotate player".to_string()),
        (OPCODE_XA_ON.to_string(), "XaOn(param1: u8, param2: i16)\n".to_owned() +
            &"Enable XA audio".to_string()),
        (OPCODE_PLC_CNT.to_string(), "PlcCnt(param1: u8)\n".to_owned() +
            &"Set player count".to_string()),
        (OPCODE_MIZU_DIV_SET.to_string(), "MizuDivSet(param1: u8)\n".to_owned() +
            &"Set water division".to_string()),
        (OPCODE_XA_VOL.to_string(), "XaVol(param1: u8)\n".to_owned() +
            &"Set XA volume".to_string()),
        (OPCODE_KAGE_SET.to_string(), "KageSet(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8, param6: i16, param7: i16, param8: i16, param9: i16)\n".to_owned() +
            &"Set shadow".to_string()),
        (OPCODE_CUT_BE_SET.to_string(), "CutBeSet(param1: u8, param2: u8, param3: u8)\n".to_owned() +
            &"Set cutscene behavior".to_string()),
        (OPCODE_SCE_ITEM_LOST.to_string(), "SceItemLost(param1: u8)\n".to_owned() +
            &"Item lost event".to_string()),
        (OPCODE_SCE_ESPR_ON2.to_string(), "SceEsprOn2(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8, param6: u8, param7: u8, param8: u8, param9: u8, param10: u8, param11: u8, param12: u8, param13: u8, param14: u8, param15: u8)\n".to_owned() +
            &"Enable sprite effect (variant 2)".to_string()),
        (OPCODE_PLC_STOP.to_string(), "PlcStop()\n".to_owned() +
            &"Stop player".to_string()),
        (OPCODE_LIGHT_POS_SET.to_string(), "LightPosSet(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8)\n".to_owned() +
            &"Set light position".to_string()),
        (OPCODE_LIGHT_KIDO_SET.to_string(), "LightKidoSet(param1: u8, param2: u8, param3: u8)\n".to_owned() +
            &"Set light intensity".to_string()),
        (OPCODE_PARTS_SET.to_string(), "PartsSet(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8)\n".to_owned() +
            &"Set parts".to_string()),
        (OPCODE_SCE_PARTS_BOMB.to_string(), "ScePartsBomb(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8, param6: u8, param7: u8, param8: u8, param9: u8, param10: u8, param11: u8, param12: u8, param13: u8, param14: u8, param15: u8)\n".to_owned() +
            &"Parts bomb event".to_string()),
        (OPCODE_SCE_PARTS_DOWN.to_string(), "ScePartsDown(param1: u8, param2: u8, param3: u8, param4: u8, param5: u8, param6: u8, param7: u8, param8: u8, param9: u8, param10: u8, param11: u8, param12: u8, param13: u8, param14: u8, param15: u8)\n".to_owned() +
            &"Parts down event".to_string()),
    ]);
    opcode_documentation
}