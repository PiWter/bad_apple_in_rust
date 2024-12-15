use std::thread;
use std::time::Duration;
use image::*;

fn main() {
    bad_apple(4);
}


fn bad_apple(downscaling: u32){
    for f in 100..=6562 {
        let mut frame_path: String = String::from("image_sequence/bad_apple_");
        let file_number = f.to_string();
        for n in file_number.chars() {
            frame_path.push(n);
        }
        frame_path += ".png";
        //println!("{frame_path}");

        let img = image::open(&frame_path).unwrap();
        let image_x_pixels = img.width();
        let image_y_pixels = img.height();

        for y in 1..image_y_pixels {
            for x in 1..image_x_pixels {
                if y % (downscaling * 2) == 0 && x % (downscaling * 2) == 0{
                    if img.get_pixel(x, y) == Rgba([0, 0, 0, 255]){
                        print!("  ");
                    } else {
                        print!("+ ");
                    }
                }
            }
            if y % (downscaling * 2) == 0{
                println!("");
            }
        }
        println!();
        thread::sleep(Duration::from_millis(50));
    }
}
