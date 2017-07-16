extern crate image;

#[derive(Copy, Clone)]
struct Complex{
    real:    f64,
    lateral: f64 // a.k.a imaginary
}
impl Complex{
    fn add(&self, num: &Complex) -> Complex{
       Complex {
           real:    self.real    + num.real,
           lateral: self.lateral + num.lateral 
       } 
    }
	fn sub(&self, num: &Complex) -> Complex{
       Complex {
           real:    self.real    - num.real,
           lateral: self.lateral - num.lateral
       } 
    }
    fn mul(&self, num: &Complex) -> Complex{
       Complex {                                  
           real:    self.real * num.real + (-1.0 * self.lateral * num.lateral),
           lateral: self.real * num.lateral + self.lateral * num.real  
       }                                          
    }
}

fn main() {
    let x = Complex {real: 1.0, lateral: 4.0};
    let y = Complex {real: 5.0, lateral: 1.0};
    let mut test = x.add(&y);
    println!("r: {}, i: {}", test.real, test.lateral);
    
    test = x.sub(&y);
    println!("r: {}, i: {}", test.real, test.lateral);
    
    test = x.mul(&y);
    println!("r: {}, i: {}", test.real, test.lateral);
}


fn mandelbrot(num: &Complex) -> f64 {
    let mut z = num;
    let mut mult = z;
    for n in 1..50 {
        mult = z.mul(&z);
        z = mult.add(&z);
        println!("r: {}, i: {}", z.real, z.lateral);
    }
    return 0.0
}
