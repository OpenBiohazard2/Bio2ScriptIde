# Biohazard 2 Script Opcodes Reference

This document contains all available opcodes for Biohazard 2 script parsing, organized in a searchable table format.

## Opcode Structure

Each opcode instruction follows this format:
- **Byte 0**: Opcode command (the opcode number)
- **Bytes 1+**: Parameters (if any)

Therefore: `Total Size = 1 + Sum of Parameter Sizes`

For example:
- `IfStart` opcode: 1 byte (opcode) + 1 byte (dummy) + 2 bytes (block_length) = 4 bytes total
- `AotSet` opcode: 1 byte (opcode) + 5 bytes (u8 params) + 8 bytes (i16 params) + 6 bytes (u8 array) = 20 bytes total

## Opcode Table

| Opcode | Name | Size (bytes) | Parameters | Description |
|--------|------|--------------|------------|-------------|
| 0 | NoOp | 1 | None | No operation |
| 1 | EvtEnd | 1 | None | End event execution |
| 2 | EvtNext | 1 | None | Continue to next event |
| 3 | EvtChain | 4 | param1 (u8), param2 (u8), param3 (u8) | Chain to another event |
| 4 | EvtExec | 4 | param1 (u8), param2 (u8), param3 (u8) | Execute another event |
| 5 | EvtKill | 2 | param1 (u8) | Kill an event |
| 6 | IfStart | 4 | dummy (u8), block_length (u16) | Start conditional block |
| 7 | ElseStart | 4 | dummy (u8), block_length (u16) | Start else block |
| 8 | EndIf | 1 | None | End conditional block |
| 9 | Sleep | 4 | dummy (u8), count (u16) | Sleep for specified time |
| 10 | Sleeping | 3 | param1 (u8), param2 (u8) | Check if sleeping |
| 11 | Wsleep | 1 | None | Wait sleep |
| 12 | Wsleeping | 1 | None | Check if wait sleeping |
| 13 | ForStart | 6 | dummy (u8), block_length (u16), count (u16) | Start for loop |
| 14 | ForEnd | 2 | param1 (u8) | End for loop |
| 15 | WhileStart | 4 | param1 (u8), param2 (u16) | Start while loop |
| 16 | WhileEnd | 2 | param1 (u8) | End while loop |
| 17 | DoStart | 4 | param1 (u8), param2 (u16) | Start do-while loop |
| 18 | DoEnd | 2 | param1 (u8) | End do-while loop |
| 19 | Switch | 4 | var_id (u8), block_length (u16) | Start switch statement |
| 20 | Case | 6 | dummy (u8), block_length (u16), value (u16) | Case in switch statement |
| 21 | Unknown21 | 1 | None | Unknown opcode |
| 22 | EndSwitch | 2 | param1 (u8) | End switch statement |
| 23 | Goto | 6 | if_else_counter (i8), loop_level (i8), unknown (u8), offset (i16) | Jump to label |
| 24 | GoSub | 2 | event (u8) | Call subroutine |
| 25 | Unknown25 | 1 | None | Unknown opcode |
| 26 | Break | 2 | param1 (u8) | Break from loop/switch |
| 27 | Unknown27 | 1 | None | Unknown opcode |
| 28 | Unknown28 | 1 | None | Unknown opcode |
| 29 | WorkCopy | 4 | param1 (u8), param2 (u8), param3 (u8) | Copy work values |
| 30 | Unknown30 | 1 | None | Unknown opcode |
| 31 | Unknown31 | 1 | None | Unknown opcode |
| 32 | Unknown32 | 1 | None | Unknown opcode |
| 33 | CheckBit | 4 | bit_array (u8), bit_number (u8), value (u8) | Check bit flag |
| 34 | SetBit | 4 | bit_array (u8), bit_number (u8), operation (u8) | Set bit flag |
| 35 | Compare | 6 | dummy (u8), var_id (u8), operation (u8), value (i16) | Compare values |
| 36 | Save | 4 | var_id (u8), value (i16) | Save value |
| 37 | Copy | 3 | dest_var_id (u8), source_var_id (u8) | Copy value |
| 38 | Calc | 6 | dummy (u8), operation (u8), var_id (u8), value (i16) | Calculate expression |
| 39 | Unknown39 | 1 | None | Unknown opcode |
| 40 | SceRnd | 1 | None | Generate random number |
| 41 | CutChg | 2 | camera_id (u8) | Change cutscene |
| 42 | CutOld | 1 | None | Restore old cutscene |
| 43 | MessageOn | 6 | param1 (u8), param2 (u8), param3 (u8), param4 (u8), param5 (u8) | Display message |
| 44 | AotSet | 20 | aot (u8), id (u8), type (u8), floor (u8), super (u8), x (i16), z (i16), width (i16), depth (i16), data[6] (u8) | Initialize aot object as a rectangle |
| 45 | ObjModelSet | 38 | object_index (u8), object_id (u8), counter (u8), wait (u8), num (u8), floor (u8), flag0 (u8), type (u16), flag1 (u16), attribute (i16), position[3] (i16), direction[3] (i16), offset[3] (i16), dimensions[3] (u16) | Set object model |
| 46 | WorkSet | 3 | component (u8), index (u8) | Set work value |
| 47 | SpeedSet | 4 | param1 (u8), param2 (u8), param3 (u8) | Set speed |
| 48 | AddSpeed | 1 | None | Add to speed |
| 49 | AddAspeed | 1 | None | Add to angular speed |
| 50 | PosSet | 8 | dummy (u8), x (i16), y (i16), z (i16) | Set position of work object |
| 51 | DirSet | 8 | param1 (u8), param2 (u8), param3 (u8), param4 (u8), param5 (u8), param6 (u8), param7 (u8) | Set direction |
| 52 | MemberSet | 4 | member_index (u8), value (u16) | Set member value |
| 53 | MemberSet2 | 3 | param1 (u8), param2 (u8) | Set member value (variant 2) |
| 54 | SeOn | 12 | param1 (u8), param2 (u8), param3 (u8), param4 (u8), param5 (u8), param6 (u8), param7 (u8), param8 (u8), param9 (u8), param10 (u8), param11 (u8) | Play sound effect |
| 55 | ScaIdSet | 4 | id (u8), flag (u16) | Set scale ID |
| 56 | Unknown56 | 1 | None | Unknown opcode |
| 57 | DirCk | 8 | param1 (u8), param2 (u8), param3 (u8), param4 (u8), param5 (u8), param6 (u8), param7 (u8) | Check direction |
| 58 | SceEsprOn | 16 | dummy (u8), id (u8), type (u8), work (u16), unknown1 (i16), x (i16), y (i16), z (i16), dir_y (u16) | Enable sprite effect |
| 59 | DoorAotSet | 32 | aot (u8), id (u8), type (u8), floor (u8), super (u8), x (i16), z (i16), width (i16), depth (i16), next_x (i16), next_y (i16), next_z (i16), next_dir (i16), stage (u8), room (u8), camera (u8), next_floor (u8), texture_type (u8), door_type (u8), knock_type (u8), key_id (u8), key_type (u8), free (u8) | Set door area of trigger |
| 60 | CutAuto | 2 | flag_on (u8) | Auto cutscene |
| 61 | MemberCopy | 3 | param1 (u8), param2 (u8) | Copy member value |
| 62 | MemberCmp | 6 | unknown0 (u8), member_index (u8), compare_operation (u8), value (i16) | Compare member values |
| 63 | PlcMotion | 4 | action (u8), move_number (u8), scene_flag (u8) | Set player motion |
| 64 | PlcDest | 8 | dummy (u8), action (u8), flag_number (u8), dest_x (i16), dest_z (i16) | Set player destination |
| 65 | PlcNeck | 10 | operation (u8), neck_x (i16), neck_y (i16), neck_z (i16), unknown[2] (i8) | Set player neck direction |
| 66 | PlcRet | 1 | None | Return player |
| 67 | Unknown67 | 1 | None | Unknown opcode |
| 68 | SceEmSet | 22 | dummy (u8), aot (u8), id (u8), type (u8), status (u8), floor (u8), sound_flag (u8), model_type (u8), em_set_flag (i8), x (i16), y (i16), z (i16), dir_y (u16), motion (u16), ctr_flag (u16) | Initialize animated entity |
| 69 | Unknown69 | 1 | None | Unknown opcode |
| 70 | AotReset | 10 | aot (u8), id (u8), type (u8), data[6] (u8) | Reset area of trigger |
| 71 | AotOn | 2 | param1 (u8) | Enable area of trigger |
| 72 | Unknown72 | 1 | None | Unknown opcode |
| 73 | Unknown73 | 1 | None | Unknown opcode |
| 74 | Unknown74 | 1 | None | Unknown opcode |
| 75 | CutReplace | 3 | param1 (u8), param2 (u8) | Replace cutscene |
| 76 | SceEsprKill | 5 | id (u8), type (u8), work_component (u8), work_index (u8) | Kill sprite effect |
| 77 | Unknown77 | 1 | None | Unknown opcode |
| 78 | ItemAotSet | 22 | aot (u8), id (u8), type (u8), floor (u8), super (u8), x (i16), z (i16), width (i16), depth (i16), item_id (u16), amount (u16), item_picked_index (u16), md1_model_id (u8), act (u8) | Set item area of trigger |
| 79 | Unknown79 | 1 | None | Unknown opcode |
| 80 | Unknown80 | 1 | None | Unknown opcode |
| 81 | SceBgmControl | 6 | id (u8), operation (u8), type (u8), left_volume (u8), right_volume (u8) | Control background music |
| 82 | Unknown82 | 1 | None | Unknown opcode |
| 83 | Unknown83 | 1 | None | Unknown opcode |
| 84 | SceEspr3dOn | 22 | dummy (u8), unknown0 (u16), work (u16), unknown1 (u16), vector1[3] (i16), vector2[3] (i16), dir_y (u16) | Enable 3D sprite effect |
| 85 | Unknown85 | 1 | None | Unknown opcode |
| 86 | Unknown86 | 1 | None | Unknown opcode |
| 87 | SceBgmtblSet | 8 | param1 (u8), param2 (u8), param3 (u8), param4 (u8), param5 (u8), param6 (u8), param7 (u8) | Set background music table |
| 88 | PlcRot | 4 | index (u8), value (i16) | Rotate player |
| 89 | XaOn | 4 | channel (u8), id (i16) | Enable XA audio |
| 90 | Unknown90 | 1 | None | Unknown opcode |
| 91 | PlcCnt | 2 | param1 (u8) | Set player count |
| 92 | Unknown92 | 1 | None | Unknown opcode |
| 93 | MizuDivSet | 2 | mizu_div_max (u8) | Set water division |
| 94 | Unknown94 | 1 | None | Unknown opcode |
| 95 | XaVol | 2 | param1 (u8) | Set XA volume |
| 96 | KageSet | 14 | work_set_component (u8), work_set_index (u8), color[3] (u8), half_x (i16), half_z (i16), offset_x (i16), offset_z (i16) | Set shadow |
| 97 | CutBeSet | 4 | param1 (u8), param2 (u8), param3 (u8) | Set cutscene behavior |
| 98 | SceItemLost | 2 | param1 (u8) | Item lost event |
| 99 | Unknown99 | 1 | None | Unknown opcode |
| 100 | SceEsprOn2 | 16 | param1 (u8), param2 (u8), param3 (u8), param4 (u8), param5 (u8), param6 (u8), param7 (u8), param8 (u8), param9 (u8), param10 (u8), param11 (u8), param12 (u8), param13 (u8), param14 (u8), param15 (u8) | Enable sprite effect (variant 2) |
| 101 | Unknown101 | 1 | None | Unknown opcode |
| 102 | PlcStop | 1 | None | Stop player |
| 103 | AotSet4p | 28 | aot (u8), id (u8), type (u8), floor (u8), super (u8), x1 (i16), z1 (i16), x2 (i16), z2 (i16), x3 (i16), z3 (i16), x4 (i16), z4 (i16), data[6] (u8) | Initialize aot object with 4 points |
| 104 | Unknown104 | 1 | None | Unknown opcode |
| 105 | Unknown105 | 1 | None | Unknown opcode |
| 106 | LightPosSet | 6 | param1 (u8), param2 (u8), param3 (u8), param4 (u8), param5 (u8) | Set light position |
| 107 | LightKidoSet | 4 | param1 (u8), param2 (u8), param3 (u8) | Set light intensity |
| 108 | Unknown108 | 1 | None | Unknown opcode |
| 109 | Unknown109 | 1 | None | Unknown opcode |
| 110 | PartsSet | 6 | param1 (u8), param2 (u8), param3 (u8), param4 (u8), param5 (u8) | Set parts |
| 111 | Unknown111 | 1 | None | Unknown opcode |
| 112 | Unknown112 | 1 | None | Unknown opcode |
| 113 | Unknown113 | 1 | None | Unknown opcode |
| 114 | Unknown114 | 1 | None | Unknown opcode |
| 115 | Unknown115 | 1 | None | Unknown opcode |
| 116 | Unknown116 | 1 | None | Unknown opcode |
| 117 | Unknown117 | 1 | None | Unknown opcode |
| 118 | Unknown118 | 1 | None | Unknown opcode |
| 119 | Unknown119 | 1 | None | Unknown opcode |
| 120 | Unknown120 | 1 | None | Unknown opcode |
| 121 | Unknown121 | 1 | None | Unknown opcode |
| 122 | ScePartsBomb | 16 | param1 (u8), param2 (u8), param3 (u8), param4 (u8), param5 (u8), param6 (u8), param7 (u8), param8 (u8), param9 (u8), param10 (u8), param11 (u8), param12 (u8), param13 (u8), param14 (u8), param15 (u8) | Parts bomb event |
| 123 | ScePartsDown | 16 | param1 (u8), param2 (u8), param3 (u8), param4 (u8), param5 (u8), param6 (u8), param7 (u8), param8 (u8), param9 (u8), param10 (u8), param11 (u8), param12 (u8), param13 (u8), param14 (u8), param15 (u8) | Parts down event |

## Parameter Types

- `u8` - Unsigned 8-bit integer (1 byte)
- `i8` - Signed 8-bit integer (1 byte)
- `u16` - Unsigned 16-bit integer (2 bytes)
- `i16` - Signed 16-bit integer (2 bytes)
- `u32` - Unsigned 32-bit integer (4 bytes)
- `i32` - Signed 32-bit integer (4 bytes)

## Special Notes

- **Sleep Opcode (9)**: Contains both sleep and sleeping commands. The sleep command is `[0x9 0xa u8 u8]`, where 0x9 is the sleep command and 0xa is the sleeping command.
- **EvtEnd (1)**: Always terminates function execution when encountered.
