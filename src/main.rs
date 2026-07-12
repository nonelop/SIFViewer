use colored::{self, Colorize};
use std::fs;

fn main() {
    let mut image = fs::read("test.sif").expect("Read Error");

    let mut counter = 1;
    let mut dot_rgb: [u8; 3] = [0, 0, 0];

    let mut width: usize = 0;

    for i in &image {
        match counter {
            1 => {
                width = *i as usize;
            }
            _ => {}
        }
        counter += 1
    }

    for _ in 0..3 {
        image.remove(0);
    }

    counter = 1;

    for i in &image {
        let mut index: i32 = counter % 3 - 1;
        if index < 0 {
            index = 2
        }
        dot_rgb[index as usize] = *i;

        if counter % 3 == 0 {
            let [r, g, b] = dot_rgb;
            print!("{}", "  ".on_custom_color((r, g, b)));
            dot_rgb.fill(0);
        }
        if counter % (width as i32 * 3) == 0 {
            print!("\n");
        }
        counter += 1
    }
}
