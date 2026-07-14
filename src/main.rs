use std::fs;

mod versions;
use versions::first::first_version;

fn main() {
    let mut image = fs::read("test.sif").expect("Read Error");

    let mut counter = 1;
    let mut version: u8 = 0;
    let mut width: usize = 0;

    for i in &image {
        match counter {
            1 => {
                version = *i;
            }
            2 => {
                width = *i as usize;
            }
            _ => {}
        }
        counter += 1
    }

    match version {
        1 => {
            first_version(&mut image, width);
        }
        _ => {
            println!("\nAN ERROR IN VERSION VALUE\n");
        }
    }
}
