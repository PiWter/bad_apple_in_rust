use std::thread;
use std::time::Duration;
use image::*;
use std::io;

fn main() {
    let frames = preload_frames();

    let mut start = String::new();

    println!("Press enter to start...");
    io::stdin().read_line(&mut start).unwrap();

    for af in frames{
        print!("{af}");
        thread::sleep(Duration::from_millis(26));
        print!("\x1B[2J\x1B[1;1H");
    }

}

fn preload_frames() -> Vec<String>{
    let mut ascii_frames: Vec<String> = Vec::new();

    for f in 1..=6562{
        println!("Loading frame {} / 6562", f);
        let mut frame_path: String = String::from("image_sequence/bad_apple_");
        
        let file_number = f.to_string();
        for n in file_number.chars() {
            frame_path.push(n);
        }
        frame_path += ".png";

        ascii_frames.push(image_to_ascii(&frame_path, 4));
    }
    ascii_frames
}

fn image_to_ascii(frame_path: &String, downscaling: u32) -> String{
    let img_frame = image::open(&frame_path).unwrap();
    let image_y_pixels = img_frame.height();
    let image_x_pixels = img_frame.width();
    let mut ascii_frame = "".to_string();
    for y in 1..image_y_pixels {
        for x in 1..image_x_pixels {
            if y % (downscaling * 2) == 0 && x % (downscaling * 2) == 0{
                if img_frame.get_pixel(x, y)[0] <= 200 && img_frame.get_pixel(x, y)[1] <= 200 && img_frame.get_pixel(x, y)[2] <= 200{
                    ascii_frame += "  ";
                } else {
                    ascii_frame += "+ ";
                }
            }
        }
        if y % (downscaling * 2) == 0{
            ascii_frame += "\n";
        }
    }
    ascii_frame += "\n";
    ascii_frame
}