// use calamine::{Reader, Xlsx, open_workbook};
// use crate::character::parse::get_char_info;
mod character;
use crate::character::parse::get_char_info;
// "Jago",
// "Sabrewulf",
// "Glacius",
// "Thunder",
// "Shadow Jago",
// "Sadira",
// "Orchid",
// "Spinal",
// "Fulgore",
// "TJ Combo",
// "Maya",
// "Kan-Ra",
// "Riptor",
// "Omen",
// "Aganos",
// "Hisako",
// "Cinder",
// "ARIA",
// "Kim Wu",
// "Tusk",
// "Arbiter",
// "Rash",
// "Gargos",
// "Mira",
// "General RAAM",
// "Eyedol",
// "Kilgore",
// "Shin Hisako",
// "Eagle",

fn main() {
    // let mut workbook: Xlsx<_> = open_workbook("/home/jr/code/rustCode/kiCalc/src/kiFrameData.xlsx").expect("Cannot open file");

    // let char_info = workbook.worksheet_range("Tusk").unwrap().unwrap();
    // for row in char_info.rows() {
    //     println!("row={:?}, row[0]={:?}", row, row[0]);
    // }
    let info = get_char_info();
    println!("{:?}", info);
}
