extern crate strum;



use strum_macros::{EnumString,EnumVariantNames};
use strum::VariantNames;
#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "PascalCase")]
pub enum Actions {
    Add,
    Remove,
    Select,
    Read,
    List,
    Put,
    Append,
    Wrong
}

pub const NAMES:&'static [&'static str] = Actions::VARIANTS;

pub trait ActionsTr: Sized {
    /// The associated error which can be returned from parsing.
    fn try_again(line: String) -> Self;
    fn get_action() -> Self; 
}


