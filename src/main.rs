mod character;
mod error;
use crate::character::parse::get_char_info;
use structopt::StructOpt;
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

#[derive(Debug, StructOpt)]
#[structopt(name = "Killer Instinct Setup Finder")]
struct KiCalcOptions {
    #[structopt(short)]
    character: String,
}

fn main() {
    let opt = KiCalcOptions::from_args();
    let info = get_char_info(&opt.character).expect("Error getting character info");
    // for (_, data) in info.moves {
    //     println!("{:?}", data)
    // }
    println!("{:?}", info.moves.get("QCF+2K > K"))
}
