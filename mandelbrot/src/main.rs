use std::str::FromStr;
use num::Complex;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::env;
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 5 {
        eprintln!("Usage: {} <filename> <pixels> <upperleft> <lowerright>", args[0]);
        eprintln!("Sample: {} mandel.png 1000x750 -1.20,0.35 -1.0,0.20", args[0]);
    }

    let bounds = parse_pair(&args[2], 'x').expect("Could not parse image size");
    let upper_left = parse_complex(&args[3]).expect("Could not parse upper left complex type");
    let lower_right = parse_complex(&args[4]).expect("Could not parse lower right complex type");    

    let mut pixels = vec![0; bounds.0 * bounds.1];
    {
        let bands: Vec<(usize, &mut [u8])> = pixels
            .chunks_mut(bounds.0)
            .enumerate()
            .collect();

        bands.into_par_iter()
            .for_each(|(i, band)| {
                let top = i;
                let band_bounds = (bounds.0, 1);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + 1), upper_left, lower_right);
                render(band, band_bounds, band_upper_left, band_lower_right);
            });        
    }

    let mut buff = image::ImageBuffer::new(bounds.0 as u32, bounds.1 as u32);
    for (x, y, pixel) in buff.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }
    buff.save("test.png").unwrap();
    write_image(&args[1], &pixels, bounds).expect("Error writing PNG file");
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels,
                   bounds.0 as u32,
                   bounds.1 as u32,
                   ColorType::Gray(8))?;
    Ok(())
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex {re, im}),
        None => None
    }
}

fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>
                ) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re,
                           upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

fn render(pixels: &mut[u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let p = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            let rgb_intensity = match escape_time(p, 255) {
                None => 0,
                Some(count) => 255 - count as u8
            };
            pixels[row * bounds.0 + column] = rgb_intensity;           
        }
    }
}


#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10,20)));
    assert_eq!(parse_pair::<i32>("", ','), None);
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.625"), Some(Complex{ re: 1.25, im: -0.625}));
    assert_eq!(parse_complex(",-0.625"), None);
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175),
                              Complex { re: -1.0, im: 1.0 },
                              Complex { re: 1.0, im: -1.0 }), Complex{ re: -0.5, im: -0.75 } );
}
