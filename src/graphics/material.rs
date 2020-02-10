use std::rc::Rc;
use crate::graphics::color::Color;
use crate::graphics::texture::Texture;

use crate::assets::image_loader;
use std::collections::HashMap;
use core::sync::atomic::{AtomicBool, Ordering};

/// Only one MaterialManager can be alive at time
/// Set to false by default (not alive)
static IS_MATERIAL_MANAGER_ALIVE: AtomicBool = AtomicBool::new(false);

pub struct MaterialManager {
    materials: HashMap<String, Material>,
    textures: HashMap<String, Rc<Texture>>
}

impl MaterialManager {
    pub fn init() -> MaterialManager {
        let was_alive = IS_MATERIAL_MANAGER_ALIVE.swap(true, Ordering::Relaxed);

        if !was_alive {
            MaterialManager {
                textures: HashMap::new(),
                materials: HashMap::new()
            }
        } else {
            panic!("Cannot create two instance of MaterialManager");
        }        
    }

    pub fn register(&mut self, name: &str, image_name: &str, tint: Color) {   
        match self.textures.get(image_name) {
            Some(texture) => {      
                self.materials.insert(
                    String::from(name), 
                    Material::new(texture.clone(), tint)
                );
            },
            _ => {
                let target_texture = Rc::new(
                    Texture::new(image_name, image_loader::load(image_name))
                );

                self.materials.insert(
                    String::from(name), 
                    Material::new(target_texture.clone(), tint)
                );

                self.textures.insert(
                    String::from(image_name), 
                    target_texture
                ); 
            }
        }; 
    }

    pub fn get(&self, name: &str) -> &Material {
        match self.materials.get(name) {
            Some(material) => {                
                return material;
            },
            _ => panic!("Unable to find material {}", name)
        };
    }

    pub fn release(&mut self, name: &str) {
        self.materials.remove(name);
    }
}

pub struct Material {
    pub texture: Rc<Texture>,
    pub tint: Color
}

impl Material {
    pub fn new(texture: Rc<Texture>, tint: Color) -> Material {
        Material {
            texture: texture,
            tint: tint
        }
    }
}