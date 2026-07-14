use colored::{self, Colorize};

pub fn first_version(image: &mut Vec<u8>, width: usize) {
    for _ in 0..3 {
        image.remove(0);
    }

    let mut counter = 1;
    let mut dot_rgb: [u8; 3] = [0, 0, 0];

    for i in image {
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