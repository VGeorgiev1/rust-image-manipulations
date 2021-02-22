
/// Greyscaling an RGB photo
/// The coefficients for the conversion are taken from the scikit-image python package
/// https://scikit-image.org/docs/dev/auto_examples/color_exposure/plot_rgb_to_gray.html
pub fn greyscaling(width: u32, height: u32, data: &image::RgbImage) -> image::GrayImage {
    let mut img = image::ImageBuffer::new(width, height);

    for w in 0..width{
        for h in 0..height {
            let n_pixel = data.get_pixel(w, h);
            let image::Rgb(channels) = *n_pixel;

            let pixel = img.get_pixel_mut(w,h);
            *pixel = image::Luma([(0.2125  * channels[0] as f64 + 0.7154 * channels[1] as f64 + channels[2] as f64 * 0.0721).round() as u8]);
        }
    }
    
    return img;
}

fn x_gradient(i: u32, j: u32, data: &image::GrayImage) -> i32 {
    -(data.get_pixel(i, j)[0] as i32)
    -2*(data.get_pixel(i, j + 1)[0] as i32)
    -(data.get_pixel(i, j + 2)[0] as i32)
    +data.get_pixel(i + 2, j)[0] as i32
    +2*(data.get_pixel(i + 2, j + 1)[0] as i32)
    +(data.get_pixel(i + 2, j + 2)[0] as i32)
}
fn y_gradient(i: u32, j: u32, data: &image::GrayImage) -> i32 {
    -(data.get_pixel(i, j)[0] as i32)
    -2*(data.get_pixel(i + 1 , j)[0] as i32)
    -(data.get_pixel(i + 2, j)[0] as i32)
    +data.get_pixel(i, j + 2)[0] as i32
    +2*(data.get_pixel(i + 1, j + 2)[0] as i32)
    +(data.get_pixel(i + 2, j + 2)[0] as i32)
}

// Edge detection implemented with the Sobel operator
// https://en.wikipedia.org/wiki/Sobel_operator
// The original formula for the result gradient does not take the square root of the sum of the x and y gradients
// This makes the algorithm faster and the approximation is good enough for images
// https://homepages.inf.ed.ac.uk/rbf/HIPR2/sobel.htm

pub fn edge_detection(width: u32, height: u32, data: &image::RgbImage) -> image::GrayImage {
    let mut grey_img = greyscaling(width, height, &data);
    
    for i in 0..width-2{
        for j in 0..height-2{

            let gx = x_gradient(i,j, &grey_img);
            let gy = y_gradient(i,j, &grey_img);

            let mut sum = gx.abs() + gy.abs();
            
            if sum > 255 {
                sum = 255;
            }

            let pixel = grey_img.get_pixel_mut(i,j);
            (*pixel) = image::Luma([sum as u8]);
        }
    }
    
    grey_img

}
