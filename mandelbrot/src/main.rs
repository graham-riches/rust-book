use rayon::prelude::*;
use scarlet::colormap::{ColorMap, ListedColorMap};
use scarlet::color::RGBColor;
use num::Complex;
use pixel_canvas::{
    Canvas, 
    canvas::CanvasInfo, 
    Color};
use glium::glutin::event::{Event, WindowEvent, MouseButton, ElementState};

/// Module for the Mandelbrot set algorithm
mod mandelbrot;

/// Cursor object to map the mouse position into the complex plane
struct Cursor {
    x: i32,
    y: i32,
}

/// The complex image plane with scaling options for zooming the image
struct FractalCanvas {
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    re_range_init: f64,
    im_range_init: f64,
    scaling: f64,
    iterations: usize,
    width: i32,
    height: i32
}

/// Combined fractal image that manages a zooming cursor and the image canvas
struct FractalImage {
    cursor: Cursor,
    canvas: FractalCanvas
}

impl FractalImage {
    /// Creates a new fractal canvas object with a complex plane mapping and a cursor for zooming the image
    /// 
    /// # Arguments
    /// * `upper_left` - The upper left complex coordinate of the image
    /// * `lower_right` - The lower right complex coordinate of the image
    /// * `scaling` - The scaling factor to zoom the image by
    /// * `width` - Canvas width dimension
    /// * `height` - Canvas height dimension
    fn new(upper_left: Complex<f64>, lower_right: Complex<f64>, scaling: f64, width: i32, height: i32) -> Self {
        Self {
            cursor: Cursor {
                x: 0,
                y: 0,
            },
            canvas: FractalCanvas {
                upper_left,
                lower_right,
                re_range_init: lower_right.re - upper_left.re,
                im_range_init: upper_left.im - lower_right.im,
                scaling,
                iterations: 0,
                width,
                height
            }
        }
    }

    /// Calculates new corner coordinates for the image and returns them as a tuple of
    /// (upper_left, lower_right)
    fn calculate_new_corner_coordinates(&self) -> (Complex<f64>, Complex<f64>) {
        let re_range = self.canvas.re_range_init / self.canvas.scaling.powf(self.canvas.iterations as f64);
        let im_range = self.canvas.im_range_init / self.canvas.scaling.powf(self.canvas.iterations as f64);
        let x_new = (self.cursor.x as f64 / self.canvas.width as f64) * re_range + self.canvas.upper_left.re;
        let y_new = ((self.canvas.height as i32 - self.cursor.y) as f64 / self.canvas.height as f64) * im_range + self.canvas.lower_right.im;
        let center = Complex{re: x_new, im: y_new};

        (Complex{ re: center.re - (re_range / (2.0 * self.canvas.scaling)), im: center.im + (im_range / (2.0 * self.canvas.scaling))},
         Complex{ re: center.re + (re_range / (2.0 * self.canvas.scaling)), im: center.im - (im_range / (2.0 * self.canvas.scaling))})
    } 

    /// Event handler for input events to the fractal Image rendering
    /// 
    /// # Arguments
    /// * `info` - Information about the current canvas
    /// * `image` - Current mouse position data
    /// * `event` - Event type to be handled, which contains different data depending on the event
    pub fn handle_events(info: &CanvasInfo, image: &mut FractalImage, event: &Event<()>) -> bool {
        match event {
            Event::WindowEvent{event: WindowEvent::MouseInput{state, button, ..}, ..} => {
                match button {
                    MouseButton::Left => {
                        if *state == ElementState::Pressed {
                            // interpolate new coordinates
                            let corners = image.calculate_new_corner_coordinates();
                            image.canvas.upper_left = corners.0;
                            image.canvas.lower_right = corners.1;
                            image.canvas.iterations += 1;
                            true
                        } else {
                            false
                        }},
                    _ => false
                }
            },

            Event::WindowEvent{event: WindowEvent::CursorMoved{position, ..}, ..} => {
                let (x, y): (i32, i32) = (*position).into();
                image.cursor.x = (x as f64 * info.dpi) as i32;
                image.cursor.y = ((info.height as i32 - y) as f64 * info.dpi) as i32;
                false
            }
            _ => false,
        }
    }

    /// TODO pass FP and comment accordingly
    pub fn render(
        &self,
        pixels: &mut [Color],
        bounds: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
        cmap: &ColorMapBuffer,
        iterations: usize,
    ) {
        // Chunk should be rows * columns as RBG
        assert_eq!(pixels.len(), bounds.0 * bounds.1);
        for row in 0..bounds.1 {
            for column in 0..bounds.0 {
                let p = pixel_to_point(bounds, (column, row), upper_left, lower_right);
                let color = match mandelbrot::escape_time(p, iterations) {
                    None => (0, 0, 0),
                    Some(count) => cmap.colors[iterations - count - 1],
                };
                pixels[column] = Color {
                    r: color.0,
                    g: color.1,
                    b: color.2,
                };
            }
        }
    }

    
}

/// Converts a point in pixels to its corresponding complex mapping
/// 
/// # Arguments
/// * `bounds` - The total bounds of the current slice
/// * `pixel` - The current pixel coordinates
/// * `upper_left` - The complex upper left hand coordinate
/// * `lower_right` - The complex lower right hand coordinate
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}


struct ColorMapBuffer {
    colors: Vec<(u8, u8, u8)>,
}

impl ColorMapBuffer {
    fn from_cmap(size: usize, cmap: &ListedColorMap) -> Self {
        let colors = (0..size).into_iter()
        .map(|x| cmap.transform_single(x as f64 / size as f64))
        .map(|x: RGBColor| ((x.r * 255 as f64) as u8, (x.g * 255 as f64) as u8, (x.b * 255 as f64) as u8))
        .collect();
        Self { colors }
    }
}


fn main() {
    let iterations = 8000;
    let cmap = ListedColorMap::plasma();
    let bounds = (800, 800);
    let cmap_buffer = ColorMapBuffer::from_cmap(iterations, &cmap);
    let mut canvas = Canvas::new(bounds.0, bounds.1)
        .title("Mandelbrot")
        .show_ms(true)
        .state(FractalImage::new(Complex{re: -1.0, im: 1.0}, Complex{re: 1.0, im: -1.0}, 2.0, bounds.0 as i32, bounds.1 as i32))
        .input(FractalImage::handle_events);
    canvas = canvas.render_on_change(true);
    canvas.render(move |fractal, image| {
        {
            let bands: Vec<(usize, &mut [Color])> = image.pixels
                .chunks_mut(bounds.0)
                .enumerate()
                .collect();
    
            bands.into_par_iter()
                .for_each(|(i, band)| {
                    let top = i;
                    let band_bounds = (bounds.0, 1);
                    let band_upper_left = pixel_to_point(bounds, (0, top), fractal.canvas.upper_left, fractal.canvas.lower_right);
                    let band_lower_right = pixel_to_point(bounds, (bounds.0, top + 1), fractal.canvas.upper_left, fractal.canvas.lower_right);
                    fractal.render(band, band_bounds, band_upper_left, band_lower_right, &cmap_buffer, iterations);
                });        
        }
    });
}
