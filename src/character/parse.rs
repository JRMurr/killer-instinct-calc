use calamine::{Reader, Xlsx, open_workbook,DataType};
use crate::character::{CharacterInfo, MoveFrameData};
use std::collections::HashMap;


fn parse_cell_num(cell: &DataType) -> Option<i16> {
    use DataType::*;
    match cell.to_owned() {
        String(_) => None, 
        Float(f) => Some(f as i16),
        Int(i) => Some(i as i16),
        _ => None
    }
}

fn parse_row(row: &[DataType]) -> Option<MoveFrameData> {
    let name: String = row[0].get_string().unwrap().into();
    let startup = parse_cell_num(&row[2]);
    let active = parse_cell_num(&row[3]);
    let mut recovery= parse_cell_num(&row[4]);
    let on_hit= parse_cell_num(&row[6]);
    let on_block= parse_cell_num(&row[7]);
    let counter_hit= parse_cell_num(&row[8]);
    let special_props = row[13].get_string().map(|x| x.into());
    let startup = match startup {
        None => {
            if recovery == None {
                // row[4] is recovery, if its not filled, this is a combo breaker row
                return None;
            }
            let tmp = recovery.unwrap();
            recovery = None;
            tmp
        },
        Some(x) => x
    };
    Some(MoveFrameData {name, startup, active, recovery, on_block, on_hit, counter_hit, special_props})
}


pub fn get_char_info() -> CharacterInfo {
    let mut workbook: Xlsx<_> = open_workbook("/home/jr/code/rustCode/kiCalc/src/kiFrameData.xlsx").expect("Cannot open file");
    let char_info = workbook.worksheet_range("Tusk").unwrap().unwrap();
    let moves = char_info.rows()
    .take_while(|x| x[0] != DataType::String("Enders".into()))
    .filter(|x| !x[0].is_empty() && !x[2].is_empty() && x[2] != DataType::String("Startup".into()))
    .filter_map(|x| parse_row(x))
    .map(|x| (x.name.clone(), x))
    .collect();
    CharacterInfo {name: "Tusk".into(), moves}
}