extern crate direct2d;
use direct2d::brush::SolidColorBrush;

mod complex;
use std::env::args;
use complex::*;


type Compl = Complex<f32>;

struct ImageOpts {
    width: u32,
    height: u32
}

fn main() {
    let io = ImageOpts{width: 3, height: 3};

    for c in complex_plane(&io) {
        println!("{:?}", c);
    }
}

fn hello(name: &str) {
    println!("FUCK YOU {}", name);
}

fn mb_does_diverge(c: Compl, iters: u32) -> u32 {
    let mut z = c;
    for i in 0..iters {
        z = z.pow(2) + c;
        if (z.abs_sq() >= 4.0) {
            return i
        }
    }

    return 0;
}

fn complex_plane<'a>(io: &'a ImageOpts) -> impl Iterator<Item = Compl> +'a {
    let real = move || (0..io.width).map(move |x| (x as f32/io.width as f32 * 3.0) - 2.0);
    let imag = (0..io.height).map(move |x| (x as f32/io.height as f32 * 3.0) - 1.5);

    imag.map(move |i| real().map(move |r| Complex(r,i))).flatten()
}
