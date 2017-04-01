#[macro_use]
extern crate glium;

use glium::Surface;

#[derive(Copy, Clone)]
struct RGB_ {
    r: f64,
    g: f64,
    b: f64,
}

#[derive(Copy, Clone)]
struct Color_ {
    r: f64,
    g: f64,
    b: f64,
    special: f64
}
impl Color_ {
    pub fn new() -> Color_ {
        Color_ {
            r: 0.5,
            g: 0.5,
            b: 0.5,
            special: 0.0
        }
    }
}

#[derive(Copy, Clone)]
struct Light_{
    position: Vect_,
    color: Color_,
}
impl Light_{
    pub fn new() -> Light_ {
        Light_ {
            position: Vect_{x:0.0, y:0.0, z:0.0},
            color:    Color_{r:1.0,g:1.0,b:1.0,special:0.0}
        }
    }
}

#[derive(Copy, Clone)]
struct Vect_ {
    x: f64,
    y: f64,
    z: f64,
}
impl Vect_ {
    fn normalize(&self) -> Vect_ { 
        let mag = (&self.x.powf(2.0) + &self.y.powf(2.0) + &self.z.powf(2.0)).powf(0.5);
        return Vect_{x: &self.x/mag, y: &self.y/mag, z: &self.z/mag}
    }
    fn magnitude(&self) -> f64 { 
        return (&self.x.powf(2.0) + &self.y.powf(2.0) + &self.z.powf(2.0)).powf(0.5)
    }
    fn negative(&self) -> Vect_ { 
        return Vect_{x: -&self.x, y: -&self.y, z: -&self.z};
    }
    
    fn dot(&self, v: Vect_) -> f64 { 
        return &self.x*v.x + &self.y*v.y + &self.z*v.z;
    }
    fn cross(&self, v: Vect_) -> Vect_ { 
        return Vect_ {x: &self.y*v.z - &self.z*v.y, 
                      y: &self.z*v.x - &self.x*v.z,
                      z: &self.x*v.y - &self.y*v.x};
    }
    fn vectAdd(&self, v: Vect_) -> Vect_ { 
        return Vect_ {x: &self.x + v.x, 
                      y: &self.y + v.y,
                      z: &self.z + v.z};
    }
    fn vectMult(&self, val: f64) -> Vect_ { 
        return Vect_ {x: &self.x*val, 
                      y: &self.y*val,
                      z: &self.z*val};
    }
    
    pub fn new() -> Vect_ {
        Vect_ {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

#[derive(Copy, Clone)]
struct Ray_ {
    origin: Vect_,
    direction: Vect_,
}
impl Ray_ {
    //fn value(&self) -> &f64 { &self.x }
    pub fn new() -> Ray_ {
        Ray_ {
            origin:    Vect_ {x: 0.0, y: 0.0, z: 0.0},
            direction: Vect_ {x: 1.0, y: 0.0, z: 0.0},
        }
    }
}

#[derive(Copy, Clone)]
struct Camera_ {
    pos:   Vect_,
    dir:   Vect_,
    right: Vect_,
    down:  Vect_,
}
impl Camera_ {
    //fn value(&self) -> &f64 { &self.x }
    pub fn new() -> Camera_ {
        Camera_ {
            pos:   Vect_ {x: 0.0, y: 0.0, z: 0.0},
            dir:   Vect_ {x: 0.0, y: 0.0, z: 1.0},
            right: Vect_ {x: 0.0, y: 0.0, z: 0.0},
            down:  Vect_ {x: 0.0, y: 0.0, z: 0.0},
        }
    }
}


fn main() {
    //?dpi
    let width:  u32 = 640;
    let height: u32 = 480;
    let mut image = Vec::new();
    
    for x in 0..width*height {
        image.push(Vect_ {x: 0.0, y: 0.0, z: 0.0});
    }
    
    let mut X = Vect_ {x: 1.0, y: 0.0, z: 0.0};
    let mut Y = Vect_ {x: 0.0, y: 1.0, z: 0.0};
    let mut Z = Vect_ {x: 0.0, y: 0.0, z: 1.0};
    
    let mut campos = Vect_{x: 3.0, y: 1.0, z:-5.2};
    let mut look_point = Vect_{x: 0.0, y: 0.0, z: 0.0};
    // Difference between camera position and the point we are aiming for
    let mut diff_between = Vect_ {x: campos.x - look_point.x, y: campos.y - look_point.y, z: campos.z - look_point.z};
    // Normalized vector opposite direction of diff_between
    let mut camdir   = diff_between.negative().normalize();
    let mut camright = Y.cross(camdir).normalize();
    let mut camdown  = camright.cross(camdir);
    let mut cam      = Camera_{pos:campos, dir:camdir, right:camright, down:camdown};
    
    let white = Color_{r:1.0,g:1.0,b:1.0,special:0.0};
    let green = Color_{r:0.5,g:1.0,b:0.5,special:0.3};
    let gray  = Color_{r:0.5,g:0.5,b:0.5,special:0.0};
    let black = Color_{r:0.0,g:0.0,b:0.0,special:0.0};
    
    let light_position = Vect_{x:-7.0, y:10.0, z: -10.0};
    let light_source   = Light_{position: light_position, color: white};
    
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 1.0, 1.0, 1.0);
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}