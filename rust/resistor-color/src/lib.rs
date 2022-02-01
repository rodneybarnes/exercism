use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;

#[repr(usize)]
#[derive(Debug, PartialEq, Copy, Clone, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}


pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::into_enum_iter().find(|e| e.int_value() == value) {
        Some(rc) => format!("{:?}", rc),
        None => "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut rcArray = ResistorColor::into_enum_iter().collect::<Vec<ResistorColor>>();
    rcArray.sort_by(|a, b| a.int_value().cmp(&b.int_value()));

    return rcArray
}
