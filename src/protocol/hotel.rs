use std::sync::Mutex;
use once_cell::sync::Lazy;

pub(crate) static CUR_HOTEL: Lazy<Mutex<Hotel>> = Lazy::new(| | Mutex::new(Hotel::Undefined));

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Hotel {
    Unity,
    Flash,
    Nitro,
    Undefined
}
