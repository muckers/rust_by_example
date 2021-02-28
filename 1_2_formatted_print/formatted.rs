use std::fmt;
use std::f32::consts;

struct Structure {
    value: i32
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result <(), fmt::Error> {
        write!(f, "{}", self.value)
    }
}

fn main() {
    println!("{} days", 31);
    
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    
    println!("{subject} {verb} {object}", subject="The lazy dog", verb="jumped over", object="the quick brown fox.");

    println!("{} of {:b} people know binary, the other half doesn't.", 1, 2);

    println!("{number:>width$}", number=1, width=6);
    
    println!("{number:>0width$}", number=1, width=6);
    
    println!("My name is {0}, {1} {0}.", "Bond", "James");
    
    #[allow(dead_code)]
    let s = Structure {value: 3};

    println!("This struct `{}` won't print...", s);

    println!("Pi is roughly {0:.6}", consts::PI);
}
