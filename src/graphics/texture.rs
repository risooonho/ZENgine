use std::collections::HashMap;
use crate::assets::image_loader;

const LEVEL: i32 = 0;
const BORDER: i32 = 0;

pub struct TextureManager {
    pub textures: HashMap<String, Texture>
}

impl TextureManager {
    pub fn new() -> TextureManager {
        TextureManager {
            textures: HashMap::new()
        }
    }

    pub fn register(&mut self, texture_name: &str) {
        self.textures.insert(String::from(texture_name), Texture::new(texture_name));
    }

    pub fn get(&self, texture_name: &str) -> &Texture {
        match self.textures.get(texture_name) {
            Some(texture) => texture,
            None => {
                panic!("Texture with name {} not found", texture_name)
            }
        }
    }

    pub fn release(&mut self, texture_name: &str) {
        let t = self.get(texture_name);
        unsafe {
            gl::DeleteTextures(1, &t.texture_id);
        }

        self.textures.remove(texture_name);
    }
}

pub struct Texture {
    name: String,

    width: u32,
    height: u32,

    texture_id: u32
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
        println!("destroyed texture {}", self.name);
    }
}

impl Texture {
    pub fn new(image_name: &str) -> Texture {
        let img = image_loader::load(image_name);

        let mut t = Texture {
            name: String::from(image_name),

            width: img.width,
            height: img.height,

            texture_id: 0
        };

        unsafe {
            gl::GenTextures(1, &mut t.texture_id);
            gl::BindTexture(gl::TEXTURE_2D, t.texture_id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                LEVEL,
                gl::RGBA as i32,
                t.width as i32,
                t.height as i32,
                BORDER,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.data.as_ptr() as *const gl::types::GLvoid
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        t
    }

    pub fn activate(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }
}
