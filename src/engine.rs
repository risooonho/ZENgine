extern crate sdl2;
extern crate gl;

use crate::assets::image_loader::ImageAsset;
use sdl2::VideoSubsystem;
use serde::{Deserialize, Serialize};

use sdl2::video::{GLProfile, DisplayMode, FullscreenType};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::gl_utilities::shader::{ShaderManager};
use crate::graphics::material::MaterialManager;
//use crate::graphics::texture::TextureManager;
//use crate::assets::asset_manager::AssetManager;

use crate::math::matrix4x4::Matrix4x4;
use crate::graphics::sprite::Sprite;
use crate::math::transform::Transform;
use crate::graphics::color::Color;

extern "system" fn dbg_callback(
    source: gl::types::GLenum,
    etype: gl::types::GLenum,
    _id: gl::types::GLuint,
    severity: gl::types::GLenum,
    _msg_length: gl::types::GLsizei,
    msg: *const gl::types::GLchar,
    _user_data: *mut std::ffi::c_void,
) {
    unsafe {
        println!(
            "dbg_callback {:#X} {:#X} {:#X} {:?}",
            source,
            etype,
            severity,
            std::ffi::CStr::from_ptr(msg),
        );
    }
}

#[derive(Serialize, Deserialize)]
pub struct EngineOption {
    pub title: String,
    pub fullscreen: bool,
    pub virtual_width: u32,
    pub virtual_height: u32,
    pub screen_width: u32,
    pub screen_height: u32
}

pub fn start(option: EngineOption) {
    // Init Window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 6);
    gl_attr.set_double_buffer(true);

    let mut window = video_subsystem
        .window(
            option.title.as_ref(), 
            option.screen_width, 
            option.screen_height
        )
        .opengl()
        .build()
        .unwrap();

    if option.fullscreen {
        let display_mode = get_display_mode(&video_subsystem, &option);
        window.set_display_mode(display_mode).unwrap();
        window.set_fullscreen(FullscreenType::True).unwrap();
    }

    let _ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(dbg_callback), std::ptr::null());

        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    println!("Pixel format of the window's GL context: {:?}", window.window_pixel_format());
    println!("OpenGL Profile: {:?} - OpenGL Version {:?}", gl_attr.context_profile(), gl_attr.context_version());
    
    let mut shader_manager = ShaderManager::init();
    let mut material_manager = MaterialManager::init();
    //let mut texture_manager = TextureManager::init();
    //let mut asset_manager = AssetManager::init();

    let projection = Matrix4x4::orthographics(0.0, option.virtual_width as f32, 0.0, option.virtual_height as f32, -100.0, 100.0);

    shader_manager.register(
        "basic", 
        include_str!("basic.vert"), 
        include_str!("basic.frag")
    );

    let basic_shader = shader_manager.get("basic");    
    
    //let logo = asset_manager.load("logo", "assets/images/test.png");
    //let logo2 = asset_manager.load("logo2", "assets/images/test2.png");

    //let test = Any::downcast_ref::<ImageAsset>(a.as_ref()).unwrap();

    //let image = a.as_ref() as &ImageAsset;

    //texture_manager.register("logo_texture", ImageAsset::convert(logo).as_ref());

    //texture_manager.register("logo_texture2", ImageAsset::convert(logo2).as_ref());

    /*let texture = texture_manager.get("logo_texture");
    let texture2 = texture_manager.get("logo_texture2");*/

    material_manager.register("test", "assets/images/test.png", Color::white());
    //material_manager.register("test2", "assets/images/test2.png", Color::white());

    //material_manager.release("test");
    //material_manager.release("test2");
    
    basic_shader.use_shader();
    
    
    //texture_manager.release("logo_texture");

    let u_projection_location = basic_shader.get_uniform_location("u_projection");

    let mut sprite = Sprite::new("test", basic_shader, material_manager.get("test"), None, None);
    sprite.load();

    let mut transform = Transform::new();
    transform.position.x = 150.0;    
    transform.position.y = 150.0;  
    
    transform.rotation.z = 30.0;  

    transform.scale.x = 3.0;
    transform.scale.y = 3.0;

    resize(None, &option);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'main_loop;
                },
                Event::KeyUp { keycode: Some(keycode), keymod, .. } => match(keycode, keymod) {
                    (Keycode::R, _) => {
                        println!("red");
                        unsafe {
                            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
                        }
                    },
                    (Keycode::G, _) => {
                        println!("green");
                        unsafe {
                            gl::ClearColor(0.0, 1.0, 0.0, 1.0);
                        }
                    }
                    (Keycode::B, _) => {
                        println!("blue");
                        unsafe {
                            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
                        }
                    }
                    _ => ()
                }
                _ => ()
            }
        }

        unsafe {
            gl::Disable(gl::SCISSOR_TEST);

            gl::ClearColor( 0.0, 0.0, 0.0, 1.0 );
            gl::Clear(gl::COLOR_BUFFER_BIT);    

            gl::Enable(gl::SCISSOR_TEST);  

            gl::ClearColor( 1.0, 0.5, 0.3, 1.0 );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        
            gl::UniformMatrix4fv(
                u_projection_location,
                1,
                gl::FALSE,
                projection.data.as_ptr()
            );

            sprite.draw(&transform.get_transformation_matrix());
        }
        window.gl_swap_window();
    }
}

fn resize(new_size: Option<(i32, i32)>, option: &EngineOption) {
    let target_aspect_ratio = option.virtual_width as f32 / option.virtual_height as f32;

    let width: u32;
    let height: u32;
    match new_size {
        Some(new_size) => {
            width = new_size.0 as u32;
            height = new_size.1 as u32;
        },
        None => {
            width = option.screen_width;
            height = option.screen_height;
        }
    } 
    let mut calculated_height = (width as f32 / target_aspect_ratio) as u32;
    let mut calculated_width = width;
    
    if calculated_height > height {
        //It doesn't fit our height, we must switch to pillarbox then
        calculated_height = height;
        calculated_width = (calculated_height as f32 * target_aspect_ratio) as u32;
    }

    // set up the new viewport centered in the backbuffer
    let vp_x = (width / 2) - (calculated_width / 2);
    let vp_y = (height / 2) - (calculated_height/ 2);
    
    unsafe {
        gl::Viewport( vp_x as i32, vp_y as i32, calculated_width as i32, calculated_height as i32 );
        gl::Scissor( vp_x as i32, vp_y as i32, calculated_width as i32, calculated_height as i32 );
    }
}

fn get_display_mode(window: &VideoSubsystem, option: &EngineOption) -> DisplayMode {
    for i in 0..window.num_display_modes(0).unwrap() {
        let display_mode = window.display_mode(0,i).unwrap();
        if display_mode.w == option.screen_width as i32 && display_mode.h == option.screen_height as i32 {
            return display_mode;
        }
    }

    panic!("No DisplayMode available for width: {} and height {}", option.screen_width, option.screen_height);
}


