use num::Complex;

/// The Mandelbrot set escape time algorithm for some complex number c
/// 
/// # Arguments
/// * `c` - The complex number to run the iterative calculation for
/// * `limit` - Maximum number of set iterations
/// 
pub fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
   let mut z = Complex { re: 0.0, im: 0.0 };
   for i in 0..limit {
       if z.norm_sqr() > 4.0 {
           return Some(i);
       }
       z = z * z + c;
   }
   None
}
