pub mod parse;
use std::collections::HashMap;

#[derive(Debug)]
pub struct MoveFrameData {
    name: String,
    startup: i16,
    active: Option<i16>,
    recovery: Option<i16>,
    on_hit: Option<i16>,
    on_block: Option<i16>,
    counter_hit: Option<i16>,
    special_props: Option<String>
}

#[derive(Debug)]
pub struct CharacterInfo {
    name: String,
    moves: HashMap<String, MoveFrameData>
}