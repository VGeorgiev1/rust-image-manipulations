#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket::Data;
use rocket::response::status::BadRequest;
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField};
use rocket::response::NamedFile;
use rocket::http::{ContentType, Status};
use rocket::local::Client;
mod utils;
mod gauss;
mod luma_operations;
mod convulutions;
mod histogram;

#[macro_use] extern crate rocket;
extern crate image;

#[get("/")]
fn index() -> Template {
    let context = {};
    Template::render("index", &context)
}

#[post("/", data = "<data>")]
fn upload(content_type: &ContentType, data: Data) -> Result<NamedFile, BadRequest<String>>
{
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec! [
            MultipartFormDataField::file("image").content_type_by_string(Some(mime::IMAGE_STAR)).unwrap(),
            MultipartFormDataField::text("filter"),
            MultipartFormDataField::text("0"),
            MultipartFormDataField::text("1"),
            MultipartFormDataField::text("2"),
            MultipartFormDataField::text("3"),
            MultipartFormDataField::text("4"),
            MultipartFormDataField::text("5"),
            MultipartFormDataField::text("6"),
            MultipartFormDataField::text("7"),
            MultipartFormDataField::text("8"),
        ]
    );

    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).unwrap();

    let photo = multipart_form_data.files.get("image"); 
    let filter = multipart_form_data.texts.remove("filter");
   
    if photo.is_none() {
        return Err(BadRequest(Some("Picture not found!".to_string())));
    }

    let file_fields = photo.unwrap();

    let file_field = &file_fields[0]; // Because we only put one "photo" field to the allowed_fields, the max length of this file_fields is 1.

    let _content_type = &file_field.content_type;
    let _path = &file_field.path;

    let photo_bbj = photo.unwrap().clone();

    let string_name = photo_bbj[0].path.clone().into_os_string().into_string().unwrap();

    let res = utils::get_file_as_byte_vec(string_name);

    let mut img = image::load_from_memory(&res).unwrap().to_rgb8();

    let (width, height) = img.dimensions();

    let mut text_field = filter.unwrap();

    let _text = text_field.remove(0).text;

    let name = utils::random_string() + ".png";

    match _text.as_str() {
        "Gaussian Blur" => {
            let kernel = gauss::gaussian_kernel1_d(10, 5.0);
            
            convulutions::apply1_dconvulution(width as i32, height as i32, &mut img, kernel);
        },
        "Greyscale" => {
            let g_image = luma_operations::greyscaling(width, height, &img);
            match g_image.save(name.clone()) {
                Err(_e) => return Err(BadRequest(Some("Error whileapplying filter!".to_string()))),
                Ok(_m) => return Ok(NamedFile::open(name).unwrap())
            }
        },
        "Histogram Equalization" => {
            histogram::equalization(width, height, &mut img);
        },
        "Sharpening" => {
            let m3 = vec![
                vec![0.0,-0.5,0.0],
                vec![-0.5,3.0,-0.5],
                vec![0.0,-0.5,0.0]
            ];
            convulutions::apply2_dconvulution(width, height, &mut img, m3);
        },
        "Edge Detection" => {
            let e_image = luma_operations::edge_detection(width, height, &img);

            match e_image.save(name.clone()) {
                Err(_e) => return Err(BadRequest(Some("Error whileapplying filter!".to_string()))),
                Ok(_m) => return Ok(NamedFile::open(name).unwrap())
            }
        }
        "Custom kernel..." => {

            let mut kernel : Vec<Vec<f64>> = Vec::new();
            let mut row : Vec<f64> = Vec::new();
            for i in 0..9 {
                let k: &str = &(i.to_string());

                if i % 3 == 0 && i != 0 {
                    kernel.push(row);
                    row = Vec::new();
                }
                
                let mut matrix_n = multipart_form_data.texts.remove(k).unwrap();
                let number_as_text = matrix_n.remove(0).text;
                
                let fl_val = number_as_text.parse::<f64>();
                match fl_val {
                    Err(_e) => { return Err(BadRequest(Some("Error parsing custom kernel!".to_string()))) },
                    Ok(number) => { row.push(number) }
                }
            }
            kernel.push(row);
            print!("{:?}", kernel);
            convulutions::apply2_dconvulution(width, height, &mut img, kernel);
        }
        _ => {
            return Err(BadRequest(Some("Filter not found!".to_string())));
        }
    }

    match img.save(name.clone()) {
        Err(_e) => return Err(BadRequest(Some("Error whileapplying filter!".to_string()))),
        Ok(_m) => return Ok(NamedFile::open(name).unwrap())
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/upload", routes![upload])
                    .mount("/", routes![index])
                    .mount("/public", StaticFiles::from("static"))
                    .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn index_page_works() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }


    #[test]
    fn static_files_work() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response_js = client.get("/public/matrix.js").dispatch();
        let response_css = client.get("/public/matrix.js").dispatch();
        assert_eq!(response_js.status(), Status::Ok);
        assert_eq!(response_css.status(), Status::Ok);
    }

    #[test]
    fn tests_for_file_leak() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response_html = client.get("/templates/intex.hbs").dispatch();
        let response_cargo = client.get("/Cargo.toml").dispatch();

        assert_eq!(response_html.status(), Status::NotFound);
        assert_eq!(response_cargo.status(), Status::NotFound);
    }

    #[test]
    fn clamp_f64_test() {
        assert_eq!(utils::clamp_0_255_f64(300.123), 255.0);
        assert_eq!(utils::clamp_0_255_f64(-100.0), 0.0);
        assert_eq!(utils::clamp_0_255_f64(-1.0), 0.0);
        assert_eq!(utils::clamp_0_255_f64(256.0),255.0);
        assert_eq!(utils::clamp_0_255_f64(256.0),255.0);


        assert_eq!(utils::clamp_0_255_f64(254.0),254.0);

        assert_eq!(utils::clamp_0_255_f64(123.123),123.123);
        assert_eq!(utils::clamp_0_255_f64(1.567),1.567);
    }
    #[test]
    fn gauss_kernel_test() {
        let kernel1 = gauss::gaussian_kernel1_d(10, 3.);
        let kernel2 = gauss::gaussian_kernel1_d(7, 3.);

        assert_eq!(kernel1.len(), 10);
        assert_eq!(kernel2.len(), 7);
    }
}