pub mod parse;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum KnockDown {
    Hard,
    Soft,
}

#[derive(Debug, PartialEq)]
pub struct MoveFrameData {
    pub name: String,
    pub startup: i16,
    pub active: Option<i16>,
    pub recovery: Option<i16>,
    pub on_hit: Option<i16>,
    pub on_block: Option<i16>,
    pub counter_hit: Option<i16>,
    pub special_props: Option<String>,
    pub knock_down: Option<KnockDown>,
}

#[derive(Debug)]
pub struct CharacterInfo {
    pub name: String,
    pub moves: HashMap<String, MoveFrameData>,
}
