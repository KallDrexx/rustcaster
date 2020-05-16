use std::collections::HashMap;
use std::io;
use std::fs::File;
use png::{OutputInfo, ColorType};

pub struct Atlas {
    image_data: Vec<u8>,
    image_info: OutputInfo,
    sections: HashMap<String, ImageSection>,
}

struct ImageSection {
    start_x: u32,
    start_y: u32,
    height: u32,
    width: u32,
}

impl Atlas {
    pub fn new(texture_file: String) -> io::Result<Self> {
        let decoder = png::Decoder::new(File::open(texture_file)?);
        let (info, mut reader) = decoder.read_info()?;
        let mut img_data = vec![0; info.buffer_size()];
        reader.next_frame(&mut img_data)?;

        Ok(Atlas {
            image_data: img_data,
            image_info: info,
            sections: HashMap::new(),
        })
    }

    pub fn create_section(&mut self, name: String, start_x: u32, start_y: u32, height: u32, width: u32) {
        if start_y + height > self.image_info.height {
            panic!("Section goes beyond image height");
        }

        if start_x + width > self.image_info.width {
            panic!("Section goes beyond image width");
        }

        self.sections.insert(name, ImageSection { start_x, start_y, height, width });
    }

    pub fn get_rgb_at(&self, section_name: &str, x: u32, y: u32) -> Option<(u8, u8, u8)> {
        let section = match self.sections.get(section_name) {
            None => return None,
            Some(x) => x,
        };

        if x > section.width || y > section.height {
            dbg!(x);
            dbg!(y);
            panic!("Attempted to get pixel outside of section boundary");
        }

        let x = x + section.start_x;
        let y = y + section.start_y;

        if x > self.image_info.width || y > self.image_info.height {
            panic!("Attempted to get pixel outside image boundary");
        }

        match self.image_info.color_type {
            ColorType::RGB => {
                let first_byte_index = ((self.image_info.width * 3 * y) + (x * 3)) as usize;
                Some((
                    *self.image_data.get(first_byte_index).unwrap(),
                    *self.image_data.get(first_byte_index + 1).unwrap(),
                    *self.image_data.get(first_byte_index+ 2).unwrap(),
                ))
            }

            x => panic!("Unknown color type of {:?}", x),
        }
    }

    pub fn get_section_width_and_height(&self, section_name: &str) -> Option<(u32, u32)> {
        match self.sections.get(section_name) {
            None => None,
            Some(section) => Some((section.width, section.height)),
        }
    }
}