mod ppm;
mod types;

use ppm::{Ppm, PpmColor};

fn main() {
    let mut img = Ppm::new(256, 256, 256);

    for i in 0 .. 256 {
        for j in 0 .. 256 {
            img.set(i, j, i as u8, j as u8, 0)
        }
    }
    println!("{}", img);
}
