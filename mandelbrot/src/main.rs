use num::Complex;
use rayon::prelude::*;
use pixel_canvas::{
    Canvas, 
    canvas::CanvasInfo, 
    Color};
use glium::glutin::event::{Event, WindowEvent, MouseButton, ElementState};
use scarlet::colormap::{ColorMap, ListedColorMap};
use scarlet::color::RGBColor;
use rug::Float;

/// Input structure to map mouse input into graphics canvas
struct MandelbrotInteractiveMouse {
    x: i32,
    y: i32,
    virtual_x: i32,
    virtual_y: i32,
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    re_range: f64,
    im_range: f64,
    re_range_init: f64,
    im_range_init: f64,
    scaling: f64,
    iterations: usize
}

impl MandelbrotInteractiveMouse {
    /// Creates a new mouse object
    fn new(upper_left: Complex<f64>, lower_right: Complex<f64>, scaling: f64) -> Self {
        Self {
            x: 0,
            y: 0,
            virtual_x: 0,
            virtual_y: 0,
            upper_left,
            lower_right,
            re_range: lower_right.re - upper_left.re,
            im_range: upper_left.im - lower_right.im,
            re_range_init: lower_right.re - upper_left.re,
            im_range_init: upper_left.im - lower_right.im,
            scaling,
            iterations: 0
        }
    }

    /// Event handler for mouse input events
    /// 
    /// # Arguments
    /// * `info` - Information about the current canvas
    /// * `mouse` - Current mouse position data
    /// * `event` - Event type to be handled, which contains different data depending on the event
    pub fn handle_events(info: &CanvasInfo, mouse: &mut MandelbrotInteractiveMouse, event: &Event<()>) -> bool {
        match event {
            Event::WindowEvent{event: WindowEvent::MouseInput{state, button, ..}, ..} => {
                match button {
                    MouseButton::Left => {
                        if *state == ElementState::Pressed {
                            // interpolate new coordinates
                            let x_new = (mouse.x as f64 / info.width as f64) * mouse.re_range + mouse.upper_left.re;
                            let y_new = ((info.height as i32 - mouse.y) as f64 / info.height as f64) * mouse.im_range + mouse.lower_right.im;
                            let center = Complex{re: x_new, im: y_new};
                            mouse.upper_left = Complex{ re: center.re - (mouse.re_range / (2.0 * mouse.scaling)), im: center.im + (mouse.im_range / (2.0 * mouse.scaling))};
                            mouse.lower_right = Complex{ re: center.re + (mouse.re_range / (2.0 * mouse.scaling)), im: center.im - (mouse.im_range / (2.0 * mouse.scaling))};
                            mouse.iterations += 1;
                            mouse.re_range = mouse.re_range_init / mouse.scaling.powf(mouse.iterations as f64);
                            mouse.im_range = mouse.im_range_init / mouse.scaling.powf(mouse.iterations as f64);
                            true
                        } else {
                            false
                        }},
                    _ => false
                }
            },
            Event::WindowEvent{event: WindowEvent::CursorMoved{position, ..}, ..} => {
                let (x, y): (i32, i32) = (*position).into();
                mouse.virtual_x = x;
                mouse.virtual_y = y;
                mouse.x = (x as f64 * info.dpi) as i32;
                mouse.y = ((info.height as i32 - y) as f64 * info.dpi) as i32;
                false
            }
            _ => false,
        }
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
    let iterations = 500;
    let cmap = ListedColorMap::viridis();
    let bounds = (1024, 1024);
    let cmap_buffer = ColorMapBuffer::from_cmap(iterations, &cmap);
    let mut canvas = Canvas::new(bounds.0, bounds.1)
        .title("Mandelbrot")
        .show_ms(true)
        .state(MandelbrotInteractiveMouse::new(Complex{re: -1.0, im: 1.0}, Complex{re: 1.0, im: -1.0}, 2.0))
        .input(MandelbrotInteractiveMouse::handle_events);
    canvas = canvas.render_on_change(true);
    canvas.render(move |mandelbrot, image| {
        let test: Float = Float::with_val(64, 1.05);
        println!("{:?}", test);   
        {
            let bands: Vec<(usize, &mut [Color])> = image.pixels
                .chunks_mut(bounds.0)
                .enumerate()
                .collect();
    
            bands.into_par_iter()
                .for_each(|(i, band)| {
                    let top = i;
                    let band_bounds = (bounds.0, 1);
                    let band_upper_left = pixel_to_point(bounds, (0, top), mandelbrot.upper_left, mandelbrot.lower_right);
                    let band_lower_right = pixel_to_point(bounds, (bounds.0, top + 1), mandelbrot.upper_left, mandelbrot.lower_right);
                    render(band, band_bounds, band_upper_left, band_lower_right, &cmap_buffer, iterations);
                });        
        }
    });
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

fn render(pixels: &mut[Color],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>,
          cmap: &ColorMapBuffer,
          iterations: usize) {
    // Chunk should be rows * columns as RBG
    assert_eq!(pixels.len(), bounds.0 * bounds.1);        
    for row in 0..bounds.1 { 
        for column in 0..bounds.0 {
            let p = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            let color = match escape_time(p, iterations) {
                None => (0, 0, 0),
                Some(count) => cmap.colors[count]
            };
            pixels[column] = Color{r: color.0, g: color.1, b: color.2};
        }
    }
}
