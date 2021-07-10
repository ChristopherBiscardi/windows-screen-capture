use arraydeque::{ArrayDeque, Wrapping};
use captrs::{Bgr8, Capturer};
use core::time;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use winput::message_loop;
use winput::{Action, Vk};

fn main() {
    let receiver = message_loop::start().unwrap();

    loop {
        match receiver.next_event() {
            message_loop::Event::Keyboard {
                vk,
                action: Action::Press,
                ..
            } => {
                if vk == Vk::Escape {
                    break;
                } else {
                    println!("{:?} was pressed!", vk);
                }
            }
            _ => (),
        }
    }

    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    
    let mut stored_frames: ArrayDeque<[Vec<Bgr8>; 30], Wrapping> = ArrayDeque::new();
    let cap = Capturer::new(0);
    match cap {
        Ok(mut cap) => {
            for i in 0..30 {
                dbg!("sleep");
                std::thread::sleep(time::Duration::from_millis(33));

                let frame = cap.capture_frame().unwrap();
                stored_frames.push_back(frame);
            }
        }
        Err(str) => {
            dbg!(str);
        }
    }
    println!("writing");
    let mut i = 0;
    for frame in stored_frames {
        dbg!("writing {}", i);
        i+=1;
        write_png_frame(&format!("./image-{}.png", i), &frame);
    }
}

fn write_png_frame(filename: &str, frame: &[Bgr8]) {
    // let filename = format!("./image-{}.png", i);
    let path = Path::new(&filename);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 1920, 1080); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let data = frame
        .iter()
        .map(|pixel| [pixel.r, pixel.g, pixel.b, 255])
        .flatten()
        .collect::<Vec<u8>>();
    writer.write_image_data(&data).unwrap(); // Save
}
