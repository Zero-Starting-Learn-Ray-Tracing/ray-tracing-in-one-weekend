use std::io::{stderr, Write};

fn main() {
    const IMAGE_WIDTH: u32 = 960;
    const IMAGE_HEIGHT: u32 = 540;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let r = (i as f32) / (IMAGE_WIDTH as f32);
            let g = (j as f32) / (IMAGE_HEIGHT as f32);
            let b = 0.2 as f32;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprint!("\ndone!");
}
