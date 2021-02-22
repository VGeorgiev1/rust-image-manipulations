use image::ImageBuffer;
use image::Rgb;
use crate::utils;

/// Apllying 2D Kernel to image
/// https://en.wikipedia.org/wiki/Kernel_(image_processing)

pub fn apply2_dconvulution(width: u32, height: u32, data: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, kernel: Vec<Vec<f64>>){
    for h in 1..height-1{
        for w in 1..width-1{

            let mut new_r : f64 = 0.0;
            let mut new_g : f64 = 0.0;
            let mut new_b : f64 = 0.0;

            for yk in -1..2{
                for xk in -1..2{
                    let n_pixel = data.get_pixel((w as i32+xk) as u32, (h as i32+yk) as u32);
                    let image::Rgb(channels) = *n_pixel;

                    new_r += (channels[0] as f64) * kernel[(yk+1) as usize][(xk+1) as usize];
                    new_g += (channels[1] as f64) * kernel[(yk+1) as usize][(xk+1) as usize];
                    new_b += (channels[2] as f64) * kernel[(yk+1) as usize][(xk+1) as usize];
                }
            }

            new_r = utils::clamp_0_255_f64(new_r); 
            new_g = utils::clamp_0_255_f64(new_g);
            new_b = utils::clamp_0_255_f64(new_b);

            let pixel = data.get_pixel_mut(w as u32,h as u32);
            (*pixel) = image::Rgb([new_r as u8, new_g as u8, new_b as u8]);

        }
    }
}

/// Apllying 1D Kernel to image
/// https://en.wikipedia.org/wiki/Kernel_(image_processing)
pub fn apply1_dconvulution(width: i32, height: i32, data: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, kernel: Vec<f64>)
{
    let kernel_size = kernel.len() as i32;
    let ks2 = (kernel_size / 2) as i32;

    for w in 0..width{
        for h in ks2..(height-ks2) {
            let mut new_r : f64 = 0.0;
            let mut new_g : f64 = 0.0;
            let mut new_b : f64 = 0.0;
            for i in -ks2..ks2{
                let n_pixel = data.get_pixel(w as u32, (i + h) as u32);
                let image::Rgb(channels) = *n_pixel;
                new_r += (channels[0] as f64) * kernel[(i + ks2) as usize];
                new_g += (channels[1] as f64) * kernel[(i + ks2) as usize];
                new_b += (channels[2] as f64) * kernel[(i + ks2) as usize];
            }
            let pixel = data.get_pixel_mut(w as u32,h as u32);
            (*pixel) = image::Rgb([new_r as u8, new_g as u8, new_b as u8]);
        }
    }

    for h in 0..height{
        for w in ks2..(width-ks2) {
            let mut new_r : f64 = 0.0;
            let mut new_g : f64 = 0.0;
            let mut new_b : f64 = 0.0;
            for i in -ks2..ks2{
                let n_pixel = data.get_pixel((i + w) as u32, h as u32);
                let image::Rgb(channels) = *n_pixel;
                new_r += (channels[0] as f64) * kernel[(i + ks2) as usize];
                new_g += (channels[1] as f64) * kernel[(i + ks2) as usize];
                new_b += (channels[2] as f64) * kernel[(i + ks2) as usize];
            }
            let pixel = data.get_pixel_mut(w as u32,h as u32);
            (*pixel) = image::Rgb([new_r as u8, new_g as u8, new_b as u8]);
        }
    }
}
