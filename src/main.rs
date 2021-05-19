use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use rayon::prelude::*;

mod complex;
use complex::*;

type Compl = Complex<f32>;

struct ImageOpts {
    width: u32,
    height: u32
}

fn main() {
    let iters = 255;
    let size = 30000;
    
    let path = Path::new(r"best_madelbrot_ever_rendered_by_human_beings.bmp.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    
    let mut encoder = png::Encoder::new(w, size, size); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let data: Vec<_> = coordinates(size).map(|(x,y)| {
        let val = mb_does_diverge(Complex(
            (x as f32 / size as f32 * 3.0) - 2.0,
            (y as f32 / size as f32 * 3.0) - 1.5,
        ), iters);
        val
    }).collect();

    writer.write_image_data(&data).unwrap();
}

fn mb_does_diverge(c: Compl, iters: u8) -> u8 {
    let mut z = c;
    for i in 0..iters {
        z = z.pow(2) + c;
        if z.abs_sq() >= 4.0 {
            return i
        }
    }

    return 0;
}

fn coordinates(size: u32) -> impl ParallelIterator<Item = (u32, u32)> {
    (0..size).into_par_iter().map(move |i| (0..size).into_par_iter().map(move |r| (r,i))).flatten()
}