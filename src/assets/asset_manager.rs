use std::any::Any;
//use crate::assets::image_loader::ImageLoader;
use std::path::Path;
use std::ffi::OsStr;
use std::rc::Rc;
use std::collections::HashMap;
use core::sync::atomic::{AtomicBool, Ordering};
/*
/// Only one AssetManager can be alive at time
/// Set to false by default (not alive)
static IS_ASSET_MANAGER_ALIVE: AtomicBool = AtomicBool::new(false);
/*
pub trait Asset: Any {
   
}*/

pub trait AssetLoader {
    fn is_extension_supported(&self, extension: &str) -> bool;

    fn load(&self, file_path: &str) -> Box<dyn Any>;
}

pub struct AssetManager {
    assets: HashMap<String, Rc<dyn Any>>,

    loaders: Vec<Box<dyn AssetLoader>>
}

impl AssetManager {
    pub fn init() -> AssetManager {
        let was_alive = IS_ASSET_MANAGER_ALIVE.swap(true, Ordering::Relaxed);

        if !was_alive {
            let mut a = AssetManager {
                assets: HashMap::new(),
                loaders: Vec::new()
            };

            a.register_loader(Box::new(ImageLoader {}));

            return a;
        } else {
            panic!("Cannot create two instance of AssetManager");
        }        
    }

    pub fn register_loader(&mut self, loader: Box<dyn AssetLoader>) {
        self.loaders.push(loader);
    }

    pub fn load(name: &str, file_path: &str) -> Box<dyn Any> {          
        let extension = Path::new(file_path)
            .extension()
            .and_then(OsStr::to_str)
            .expect(&format!("Unable to extract extension from file_path {}", file_path));

        for loader in self.loaders.iter() {
            if loader.is_extension_supported(extension) {
                return loader.load(file_path);
            }
        }

        panic!("no loader for target extension");
    }

    pub fn get(&self, name: &str) -> Rc<dyn Any> {
        match self.assets.get(name) {
            Some(material) => {                
                return material.clone();
            },
            _ => panic!("Unable to find asset {}", name)
        };
    }

    pub fn release(&mut self, name: &str) {
        self.assets.remove(name);
    }
}*/