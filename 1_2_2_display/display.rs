use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Display: {} + {}i", self.real, self.imag)
    }
}

fn main() {

    let comp = Complex {real:3.3, imag:7.2};
    println!("{}", comp);
    println!("{:?}", comp);
    
}
