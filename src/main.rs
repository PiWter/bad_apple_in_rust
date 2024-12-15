use std::thread;
use std::time::Duration;
use image::*;

fn main() {
    preload_images();
}

fn preload_images(){
    let mut frames: Vec<DynamicImage> = Vec::new();

    for f in 100..=6562{
        println!("Loading frame {} / 6562", f);
        let mut frame_path: String = String::from("image_sequence/bad_apple_");
        let file_number = f.to_string();
        for n in file_number.chars() {
            frame_path.push(n);
        }
        frame_path += ".png";

        frames.push(image::open(&frame_path).unwrap());
    }
    let image_x_pixels = frames[0].width();
    let image_y_pixels = frames[0].height();
    bad_apple(9, image_x_pixels, image_y_pixels, frames);
}

fn bad_apple(downscaling: u32, image_x_pixels: u32, image_y_pixels: u32, frames: Vec<DynamicImage>){
    let mut frame: String;
    for f in frames {
        frame = "".to_string();
        for y in 1..image_y_pixels {
            for x in 1..image_x_pixels {
                if y % (downscaling * 2) == 0 && x % (downscaling * 2) == 0{
                    if f.get_pixel(x, y) == Rgba([0, 0, 0, 255]){
                        //print!("  ");
                        frame += "  ";
                    } else {
                        //print!("+ ");
                        frame += "+ ";
                    }
                }
            }
            if y % (downscaling * 2) == 0{
                //println!("");
                frame += "\n";
            }
        }
        println!("{frame}");
        thread::sleep(Duration::from_millis(5));
    }
}
