use std::fs;
use std::fs::File;
use std::io::Read;
use rand::Rng;
use rand::distributions::Alphanumeric;

//https://www.reddit.com/r/rust/comments/dekpl5/how_to_read_binary_data_from_a_file_into_a_vecu8/
pub fn get_file_as_byte_vec(filename: String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

pub fn clamp_0_255_f64(x: f64) -> f64 {
    (0.0_f64).max((255.0_f64).min(x))
}

pub fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}