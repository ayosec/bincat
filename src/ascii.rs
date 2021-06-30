use std::fmt;

pub struct EscapedChar(pub u8);

impl fmt::Display for EscapedChar {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            0x07 => fmt.write_str("\\a"),
            0x08 => fmt.write_str("\\b"),
            0x1B => fmt.write_str("\\e"),
            0x0C => fmt.write_str("\\f"),
            0x0A => fmt.write_str("\\n"),
            0x0D => fmt.write_str("\\r"),
            0x09 => fmt.write_str("\\t"),
            0x0B => fmt.write_str("\\v"),
            b => write!(fmt, "{:02X}", b),
        }
    }
}
