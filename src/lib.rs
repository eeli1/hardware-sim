mod chip_def;
mod circuit;
mod components;
mod connection;
mod entry;
mod in_out;
mod lookup_tabel;

pub use chip_def::{ChipDef, ComponentDef, ComponentIO, ComponentMap};
pub use circuit::Circuit;
pub use connection::Connection;
pub use lookup_tabel::LookupTable;

#[derive(PartialEq, Debug, Clone)]
pub struct Error {
    msg: String,
}

impl Error {
    fn msg(msg: String) -> Self {
        Self { msg }
    }
    fn msg_str(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

pub trait Component: Clone {
    fn set(&mut self, in_names: &str, value: bool) -> Result<(), Error>;
    fn get(&mut self, out_names: &str) -> Result<bool, Error>;
    fn in_names(&self) -> Vec<String>;
    fn out_names(&self) -> Vec<String>;
    fn name(&self) -> String;
    fn to_lut(&self) -> Option<LookupTable>;
    fn to_circuit(&self) -> Option<Circuit>;
    fn set_max_vised(&mut self, max_vised: u8);
}

trait IODevice: Clone {
    fn set(&mut self, in_name: &str, value: bool, max_vised: u8) -> Result<(), Error>;
    fn get(&mut self, out_name: &str, max_vised: u8) -> Result<bool, Error>;
    fn in_names(&self) -> Vec<String>;
    fn out_names(&self) -> Vec<String>;
}
