use int_enum::IntEnum;
use enum_iterator::{all, Sequence};

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black   = 0,
    Brown   = 1,
    Red     = 2,
    Orange  = 3,
    Yellow  = 4,
    Green   = 5,
    Blue    = 6,
    Violet  = 7,
    Grey    = 8,
    White   = 9,
}

// The IntEnum trait allows method int_value to return the numerical equivalent
pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

// The IntEnum trait allows mehod from_int which converts an int to enum equivalent
pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value){
        Ok(resistor) => format!("{:?}", resistor),
        Err(_) => "value out of range".to_string()
    }
}

// The enum-iterator crate has function 'all' which returns an iterator over an enum
// The iterator can be converted to a collection using the collect() method
// and specifying to collect into a vector
pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
