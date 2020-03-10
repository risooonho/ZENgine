use crate::world::scene::Scene;
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
        let mut sm = ShaderManager {
            shaders: HashMap::new()
        };

        sm.shaders.insert(
            String::from("basic"), 
            Shader::create_basic_shader()
        );

        sm
    }

    pub fn register(&mut self, shader_name: &str, shader_file: &str) -> &Shader {
        self.shaders.insert(String::from(shader_name), Shader::new(shader_file));

        self.get(shader_name)
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

    pub fn register(&mut self, texture_name: &str, file_name: &str) {
        self.textures.insert(String::from(texture_name), Texture::new(file_name));
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
        self.textures.remove(texture_name);
    }
}