use crate::graphics::texture::Texture;
use crate::gl_utilities::shader::Shader;
use std::collections::HashMap;

pub struct Manager {
    pub shaders: ShaderManager,
    pub textures: TextureManager
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            shaders: ShaderManager::new(),
            textures: TextureManager::new()
        }
    }
}

pub struct ShaderManager {
    shaders: HashMap<String, Shader>
}

impl ShaderManager {
    pub fn new() -> ShaderManager {        
        let mut s = ShaderManager {
            shaders: HashMap::new()
        };

        s.shaders.insert(
            String::from("basic"), 
            Shader::create_basic_shader()
        );

        s
    }

    pub fn register(&mut self, name: &str) -> &Shader {
        self.shaders.insert(String::from(name), Shader::new(name));

        self.get(name)
    }

    pub fn get(&self, name: &str) -> &Shader {
        match self.shaders.get(name) {
            Some(shader) => return shader,
            _ => panic!("Unable to find shader {}", name)
        };
    }
}

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

        self.textures.remove(texture_name);
    }
}