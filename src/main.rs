mod character;
use crate::character::parse::get_char_info;
#[macro_use]
extern crate lazy_static;

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
    let info = get_char_info();
    println!("{:?}", info.name);
    // for (_, data) in info.moves {
    //     println!("{:?}", data)
    // }
    println!("{:?}", info.moves.get("QCF+2K > K"))
}
