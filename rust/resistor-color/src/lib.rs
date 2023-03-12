use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntEnum, Sequence)]
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

pub fn color_to_value(color: ResistorColor) -> u32 {
    match color {
        ResistorColor::Black    => ResistorColor::Black.int_value(),
        ResistorColor::Blue     => ResistorColor::Blue.int_value(),
        ResistorColor::Brown    => ResistorColor::Brown.int_value(),
        ResistorColor::Green    => ResistorColor::Green.int_value(),
        ResistorColor::Grey     => ResistorColor::Grey.int_value(),
        ResistorColor::Orange   => ResistorColor::Orange.int_value(),
        ResistorColor::Red      => ResistorColor::Red.int_value(),
        ResistorColor::Violet   => ResistorColor::Violet.int_value(),
        ResistorColor::White    => ResistorColor::White.int_value(),
        ResistorColor::Yellow   => ResistorColor::Yellow.int_value(),
    }
}

pub fn value_to_color_string(value: u32) -> String {
    match value {
        0   => format!("{:?}", ResistorColor::Black),
        1   => format!("{:?}", ResistorColor::Brown),
        2   => format!("{:?}", ResistorColor::Red),
        3   => format!("{:?}", ResistorColor::Orange),
        4   => format!("{:?}", ResistorColor::Yellow),
        5   => format!("{:?}", ResistorColor::Green),
        6   => format!("{:?}", ResistorColor::Blue),
        7   => format!("{:?}", ResistorColor::Violet),
        8   => format!("{:?}", ResistorColor::Grey),
        9   => format!("{:?}", ResistorColor::White),
        _   => String::from("value out of range")
    }    
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect()
}
