use std::fs;
use base64::{prelude::BASE64_STANDARD, Engine};
use raster::Image;
use opencv::{videoio::VideoWriterTrait};

fn generate_image_array(path: std::path::PathBuf) -> Vec<Vec<u8>>{
    let filecontent: Result<Vec<u8>, std::io::Error> = fs::read(path.clone());
    let value: Vec<u8> = match filecontent{
        Ok(val) => val,
        Err(_) => panic!("Invalid file")
    };
    let filename: Vec<char>  = path.file_name().unwrap().to_str().unwrap().chars().collect();
    let encoded_string: Vec<char> = BASE64_STANDARD.encode(value).chars().collect();
    let mut encodedarray: Vec<char> = Vec::new();
    let delimiter: Vec<char> = "|.|".chars().collect();
    for char in filename {
        encodedarray.push(char);
    }
    for char in delimiter {
        encodedarray.push(char);
    }
    for char in encoded_string{
        encodedarray.push(char);
    }

    let mut colorarray:Vec<Vec<u8>> = Vec::new();
    
    for index in (0..encodedarray.len()).step_by(3) {
        let mut tempvec: Vec<u8> = Vec::new();
        let r: u32 =  encodedarray[index].into();
        let g: u32;
        let b: u32;
        tempvec.push(r as u8);
        if !(index + 1 > encodedarray.len() - 1){
            g = encodedarray[index+1].into();
        }
        else {
            g = 255;
        }
        if !(index + 2 > encodedarray.len() - 1){
            b =  encodedarray[index+2].into();
        }
        else {
            b= 255;
        }
        tempvec.push(g as u8);
        tempvec.push(b as u8);
        colorarray.push(tempvec);
    }
    return colorarray;
}

pub fn encode(path:std::path::PathBuf, output:std::path::PathBuf) {
    let array: Vec<Vec<u8>>= generate_image_array(path);
    let pixels_per_image = 512 * 240;
    let ammount =  (array.len() + pixels_per_image - 1) / pixels_per_image;
    for i in 0..ammount{
        let mut image: Image = Image::blank(512, 240);
        for x in 0..512 {
            for y in 0..240 {
                let pixelindex = i * (512 * 240) + (y as usize * 512 + x as usize);
                if array.len() > pixelindex{
                    _ = image.set_pixel(x, y, raster::Color::rgb(array[pixelindex][0] as u8, array[pixelindex ][1] as u8, array[pixelindex][2] as u8));
                }
                else {
                    _ = image.set_pixel(x, y, raster::Color::white())
                }
            }
        }
        _ = fs::create_dir_all("./output/");
        let filename = String::from("./output/sample") + &i.to_string() + ".png";
        _ = raster::save(&image,  &filename);
    }
    let mut videowritter = opencv::videoio::VideoWriter::new(output.join("output.avi").to_str().unwrap(), opencv::videoio::VideoWriter::fourcc('F','F','V','1').unwrap(), 5.0, opencv::core::Size::new(512, 240), true).unwrap();
    let mut entries: Vec<_> = fs::read_dir("./output/")
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
        let frame: opencv::prelude::Mat = opencv::imgcodecs::imread(image.as_path().to_str().unwrap(), opencv::imgcodecs::IMREAD_COLOR).unwrap();
        _ = videowritter.write(&frame).unwrap();
    }
    _ = videowritter.release();
    _ = fs::remove_dir_all("./output/");
}