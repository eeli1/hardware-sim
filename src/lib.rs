mod circuit;
mod connection;
mod entry;
mod in_out;
mod lookup_tabel;

#[derive(PartialEq, Debug, Clone)]
pub struct Error {
    msg: String,
}

impl Error {
    fn msg(msg: &str) -> Self {
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
    fn box_clone(&self) -> Box<Self>;
}

trait IODevice {
    fn set(&mut self, in_name: &str, value: bool, max_vised: u8) -> Result<(), Error>;
    fn get(&mut self, out_name: &str, max_vised: u8) -> Result<bool, Error>;
    fn in_names(&self) -> Vec<String>;
    fn out_names(&self) -> Vec<String>;
}
