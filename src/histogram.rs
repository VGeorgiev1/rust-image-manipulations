use image::Rgb;
use image::ImageBuffer;

use crate::utils;
/// Creates a histogram equalized image
/// https://en.wikipedia.org/wiki/Histogram_equalization
/// 
/// In order to use the Histogram equalization algorithm first the RGB image needs to be converted to a color space with color intesity factor.
/// Here the YCrCb color space is used because its designed for digital photographs
/// The conversion is made according to this document: https://web.archive.org/web/20180421030430/http://www.equasys.de/colorconversion.html
pub fn equalization(width: u32, height: u32, data: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let mut hist = vec![0; 256];

    for w in 0..width{
        for h in 0..height {
            let n_pixel = data.get_pixel(w, h);
            let image::Rgb(channels) = *n_pixel;
            
            let r = channels[0] as f64;
            let g = channels[1] as f64;
            let b = channels[2] as f64;

            let y_val = (16.0 + 0.257 * r + 0.504 * g + 0.098 * b).round() as i32;
            let b_val = (128.0 - 0.148 * r - 0.291 * g + 0.439 * b).round() as i32;
            let r_val = (128.0 + 0.439 * r - 0.368 * g - 0.071 * b).round() as i32;

            let pixel = data.get_pixel_mut(w,h);
            *pixel = image::Rgb([y_val as u8, b_val as u8, r_val as u8]);
            
            hist[y_val as usize]+=1; 
        }
    }
    
    let mut cumulative = 0;
    let mut look_up = vec![0; 256];

    let total_pixes = width * height;

    let mut i = 0;
    while hist[i] == 0{
        i+=1;
    }

    let scale = 255.0 / (total_pixes - hist[i]) as f64;

    for (pos, val) in hist.iter().enumerate() {
       cumulative += val;

       let val = utils::clamp_0_255_f64((cumulative as f64 * scale).round());

       look_up[pos] = val as u8;
    }

    for w in 0..width{
        for h in 0..height {
            let n_pixel = data.get_pixel(w as u32, h as u32);
            let image::Rgb(channels) = *n_pixel;
            
            let y = channels[0];
            let b = channels[1];
            let r = channels[2];

            let modified_y = look_up[y as usize];

            let y_d = modified_y as f64 - 16.0;
            let b_d = b as f64 - 128.0;
            let r_d = r as f64 - 128.0;

            let mut new_r = 1.164 * y_d + 1.596 * r_d;
            let mut new_g = 1.164 * y_d - 0.392 * b_d - 0.813 * r_d;
            let mut new_b = 1.164 * y_d + 2.017 * b_d;

            new_r = utils::clamp_0_255_f64(new_r);
            new_g = utils::clamp_0_255_f64(new_g);
            new_b = utils::clamp_0_255_f64(new_b);

            let pixel = data.get_pixel_mut(w as u32,h as u32);
            (*pixel) = image::Rgb([new_r as u8, new_g as u8, new_b as u8]);
        }
    }
}