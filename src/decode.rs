use opencv::{self, core::{MatTraitConst, Vector}, videoio::{VideoCaptureTrait, VideoCaptureTraitConst}};
use std::fs;
use raster;
use base64::{prelude::BASE64_STANDARD, Engine};

fn save_frames(path:std::path::PathBuf){
    let mut videocapture = opencv::videoio::VideoCapture::from_file(path.as_path().to_str().unwrap(), opencv::videoio::CAP_ANY).unwrap();
    _ = fs::create_dir_all("./file_output_frames/");
    if !videocapture.is_opened().unwrap() {
        panic!("Is the file you provided a video?")
    }
    let mut frame = opencv::core::Mat::default();
    let mut index = 0;
    loop {
        let success = videocapture.read(&mut frame).unwrap();
        if !success || frame.empty() {
            break;
        }
        let filename = format!("./file_output_frames/sample{}.png", index);
        let mut params: Vector<i32> = Vector::new();
        params.push(opencv::imgcodecs::IMWRITE_PNG_COMPRESSION);
        params.push(0);
        _ = opencv::imgcodecs::imwrite(&filename, &frame, &params);
        index += 1;
    }
}

fn generate_file_from_images(output:std::path::PathBuf) {
    let mut buffer: Vec<char> = Vec::new();
    let mut entries: Vec<_> = fs::read_dir("./file_output_frames/")
    .unwrap()
    .map(|res| res.unwrap().path())
    .collect();
    entries.sort_by_key(|path| {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .and_then(|s| s.strip_prefix("sample"))
        .and_then(|n| n.parse::<u32>().ok())
        .unwrap_or(0)
    });
    for image in entries {
        let img = raster::open(image.to_str().unwrap()).unwrap();
        for y in 0..240 {
            for x in 0..512 {
                let pixel: raster::Color = img.get_pixel(x, y).unwrap();
                if pixel.r != 255 {
                    buffer.push(char::from_u32(pixel.r as u32).unwrap());
                }
                if pixel.g != 255 {
                    buffer.push(char::from_u32(pixel.g as u32).unwrap());
                }
                if pixel.b != 255 {
                    buffer.push(char::from_u32(pixel.b as u32).unwrap());
                    }
                }
            }
        }
        let final_string: String = String::from_iter(buffer);
        if let Some((filename, encodedfile)) = final_string.split_once("|.|") {
            _ = fs::write(output.join(filename), BASE64_STANDARD.decode(encodedfile).unwrap());
        }
        _ = fs::remove_dir_all("./file_output_frames/").unwrap()
    }

pub fn decode(path:std::path::PathBuf, output:std::path::PathBuf) {
    save_frames(path);
    generate_file_from_images(output);
}