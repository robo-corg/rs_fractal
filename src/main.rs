extern crate image;
extern crate num;

use num::Complex;

fn main() {
    let sizex = 2048;
    let sizey = 2048;
    let sizexf = sizex as f32;
    let sizeyf = sizey as f32;

    let mut img = image::RgbImage::new(sizex, sizex);

    for y in 0..sizex {
        for x in 0..sizey {
            let px = (x as f32) / sizexf;
            let py = (y as f32) / sizeyf;

            let c = Complex::<f32>::new((px - 0.75)*2.0, (py - 0.5)*2.0);
            let mut z : Complex<f32> = Complex::<f32>::new(0.0, 0.0);
            let mut n = 0;

            while n < 255 && z.re*z.re + z.im*z.im < 4.0 {
                z = z*z + c;
                n += 1;
            }

            img.put_pixel(x, y, image::Rgb{data: [n, n, n]});
        }
    }

    img.save("output.png").unwrap();
}
