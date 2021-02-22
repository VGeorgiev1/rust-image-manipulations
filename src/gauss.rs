use std::f32::consts::PI;

// Calculating 1d Gaussian fuction the output of this function is what is known as standard deviation
// https://en.wikipedia.org/wiki/Gaussian_function
pub fn gauss(x: f64, sigma: f64) -> f64 {
    return (-((x * x)/(2.0*sigma*sigma))).exp() / (2.0 * PI as f64 * sigma * sigma).sqrt();
}

pub fn gaussian_kernel1_d(samples: i8, sigma: f64) -> Vec<f64> {

    let steps = (if samples % 2 == 0 {samples - 2} else {samples - 1}) / 2;

    let step_size = 1.0;

    let mut kernel : Vec<f64> = Vec::<f64>::new();
    let mut sum : f64 = 0.0;
    for i in (0..steps).rev() {
        let x = -(i as f64 * step_size);

        let gauss = gauss(x, sigma);
        sum += gauss;
        kernel.push(gauss);
    }

    kernel.push(gauss(0.0, sigma));

    sum += kernel.last().unwrap();

    if samples % 2 == 0 {
        kernel.push(gauss(0.0, sigma));
        sum += kernel.last().unwrap();
    }

    for i in 1..steps+1 {
        let x = i as f64 * step_size;
        let gauss = gauss(x, sigma);

        sum += gauss;

        kernel.push(gauss);
    }
    
    for j in 0..kernel.len() {
        kernel[j] /= sum;
    }

    return kernel;
}