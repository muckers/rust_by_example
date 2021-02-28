use std::fmt::{self, Formatter, Display};

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        /* This works, but it seems awkward af...
        let mut rhex = self.red as u32;
        let mut ghex = self.green as u32;
        let bhex = self.blue as u32;
        ghex = ghex << 08;
        rhex = rhex << 16;
        let colhex = rhex + ghex + bhex;*/
        let sum: u64 = (self.red + self.green + self.blue).into(); //#panic
        write!(f, "RGB ({}, {}, {}) {:#08X}", self.red, self.green, self.blue, sum)
    }
}

fn main() {
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color);
    }
}


