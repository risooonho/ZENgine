extern crate sdl2;
extern crate gl;

use crate::input::Input;
use crate::input::Input::Keyboard;
use crate::input::{InputEvent, ActionType, InputFromEvent};
use crate::input::InputMapping;
use crate::behaviors::BehaviorDeclaration;
use crate::behaviors::Behavior;
use crate::components::ComponentDeclaration;
use sdl2::VideoSubsystem;
use sdl2::video::{GLProfile, DisplayMode, FullscreenType, SwapInterval};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{thread};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use serde::{Deserialize};

use crate::components::Component;
use crate::components::sprite_component::SpriteComponent;
use crate::world::manager::Manager;
use crate::math::matrix4x4::Matrix4x4;
use crate::world::scene::Scene;
use crate::assets::text_loader;

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

#[derive(Deserialize)]
pub struct EngineOption {
    pub title: String,
    pub fullscreen: bool,
    pub virtual_width: u32,
    pub virtual_height: u32,
    pub screen_width: u32,
    pub screen_height: u32,
    pub fps: u32
}

#[derive(Deserialize)]
pub struct DeclarationItem {
    pub name: String,
    pub file: String
}

#[derive(Deserialize, Default)]
pub struct ResourceDeclaration {
    #[serde(default)]
    pub shaders: Vec<DeclarationItem>,
    #[serde(default)]
    pub textures: Vec<DeclarationItem>
}

pub struct JsonBuilder {
    pub components: HashMap<String, fn(declaration: &ComponentDeclaration) -> Box<dyn Component>>,
    pub behaviors: HashMap<String, fn(declaration: &BehaviorDeclaration) -> Box<dyn Behavior>>
}

pub fn start(
    option: EngineOption, 
    resources_declaration: ResourceDeclaration,
    scenes_declaration: Vec<(String, Option<String>, Option<fn(&mut Scene)>)>,
    component_builder_declaration: Vec<(String, fn(declaration: &ComponentDeclaration) -> Box<dyn Component>)>,
    behavior_builder_declaration: Vec<(String, fn(declaration: &BehaviorDeclaration) -> Box<dyn Behavior>)>,
    input_mapping: InputMapping,
    first_scene: &str
) {
    println!("Hello, ZENgine!");

    // Init Window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let controller_subsystem = sdl_context.game_controller().unwrap();

    let available = controller_subsystem.num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e)).unwrap();

    println!("{} joysticks available", available);

    // Iterate over all available joysticks and look for game controllers.
    let mut _controller = (0..available).find_map(|id| {
        if !controller_subsystem.is_game_controller(id) {
            println!("{} is not a game controller", id);
            return None;
        }

        println!("Attempting to open controller {}", id);

        match controller_subsystem.open(id) {
            Ok(c) => {
                // We managed to find and open a game controller,
                // exit the loop
                println!("Success: opened \"{}\"", c.name());
                Some(c)
            },
            Err(e) => {
                println!("failed: {:?}", e);
                None
            }
        }
    }).expect("Couldn't open any controller");

    println!("Controller mapping: {}", _controller.mapping());

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

    video_subsystem.gl_set_swap_interval(SwapInterval::VSync).unwrap();

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(dbg_callback), std::ptr::null());

        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    println!("Pixel format of the window's GL context: {:?}", window.window_pixel_format());
    println!("OpenGL Profile: {:?} - OpenGL version: {:?}", gl_attr.context_profile(), gl_attr.context_version());

    let mut builder = JsonBuilder { components: HashMap::new(), behaviors: HashMap::new() };
    builder.components.insert(String::from("sprite"), SpriteComponent::json_builder);

    for c in &component_builder_declaration {
        builder.components.insert(String::from(&c.0), c.1);
    }

    for b in &behavior_builder_declaration {
        builder.behaviors.insert(String::from(&b.0), b.1);
    }    
    
    let mut manager = Manager::new();
    
    for t in resources_declaration.shaders.iter() {
        manager.shaders.register(&t.name, &t.file);
    }
    
    for t in resources_declaration.textures.iter() {
        manager.textures.register(&t.name, &t.file);
    }
    
    let mut scenes = HashMap::new();
    for s in scenes_declaration.iter() {
        scenes.insert(
            String::from(&s.0), 
            ( 
                match &s.1 { Some(file) => Some(String::from(file)), _ => None }, 
                s.2
            )
        );
    }    

    let mut action_map: HashMap<Input, String> = HashMap::new();
    for am in input_mapping.action_mapping {
        for e in am.events {
            action_map.insert(e, am.name.clone());
        }
    }

    let mut axis_map: HashMap<Input, String> = HashMap::new();
    for am in input_mapping.axis_mapping {
        for e in am.events {
            axis_map.insert(e, am.name.clone());
        }
    }

    let projection = Matrix4x4::orthographics(0.0, option.virtual_width as f32, 0.0, option.virtual_height as f32, -100.0, 100.0);
    let u_projection_location = manager.shaders.get("basic").get_uniform_location("u_projection");
    
    let scene_declaration = scenes.get(first_scene).unwrap();
    
    manager.shaders.get("basic").use_shader();

    resize(None, &option);
    
    let mut scene;
    if let Some(scene_file) = &scene_declaration.0 {
        scene = Scene::declare_from_json(first_scene, &scene_file, &builder);
    } else {
        scene = Scene::new(first_scene)
    }
    if let Some(declare_delegate) = scene_declaration.1 {
        declare_delegate(&mut scene);
    }

    scene.declare_resource(&mut manager);   

    scene.load(&manager);

    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let mut start_loop_time;
    let mut end_loop_time = None;
    let sec_per_frame = 1.0 / option.fps as f32;    


    'main_loop: loop {
        start_loop_time = Instant::now();

        let delta = match end_loop_time {
            Some(end_loop_time) => start_loop_time.duration_since(end_loop_time).as_secs_f32(),
            None => sec_per_frame
        };

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'main_loop;
                },
                _ => {
                    let input_from_event = Input::input_from_event(&event);
                    match input_from_event {
                        InputFromEvent::Single(data) => {
                            if let Some(action) = action_map.get(&data.input) {
                                let input_event = InputEvent::Action(action.clone());
                                scene.propagate_input_event(delta, &input_event);
                            }                            

                            if let Some(action) = axis_map.get(&data.input) {
                                let input_event = InputEvent::Axis(action.clone(), data.value);
                                scene.propagate_input_event(delta, &input_event);
                            }
                        },
                        _ => {}
                    }
                }
            }
        }

        scene.update(delta);

        unsafe {
            gl::Disable(gl::SCISSOR_TEST);

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::Enable(gl::SCISSOR_TEST);

            gl::ClearColor(
                scene.background.r as f32 / 255.0,
                scene.background.g as f32 / 255.0,
                scene.background.b as f32 / 255.0,
                scene.background.a as f32 / 255.0
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        
            gl::UniformMatrix4fv(
                u_projection_location,
                1,
                gl::FALSE,
                projection.data.as_ptr()
            );

            scene.render();
        }
        window.gl_swap_window();

        end_loop_time = Some(Instant::now());

        let loop_time = end_loop_time.unwrap().duration_since(start_loop_time).as_secs_f32();
        let sleep_for = sec_per_frame - loop_time;        
        if sleep_for > 0.0 {
            thread::sleep(Duration::from_secs_f32(sleep_for));
        }
            
        //println!("Limitless FPS {} - current FPS {} - loop_time {} - sleeped for {} - delta {}", 1.0 / loop_time, 1.0 / (loop_time + sleep_for), loop_time, sleep_for, delta);
    }

}

fn resize(new_size: Option<(i32, i32)>, option: &EngineOption) {
    let target_aspect_ratio = option.virtual_width as f32 / option.virtual_height as f32;

    let width: i32;
    let height: i32;
    match new_size {
        Some(new_size) => {
            width = new_size.0;
            height = new_size.1;
        },
        None => {
            width = option.screen_width as i32;
            height = option.screen_height as i32;
        }
    }

    let mut calculated_height = (width as f32 / target_aspect_ratio) as i32;
    let mut calculated_width = width;

    if calculated_height > height {
        calculated_height = height;
        calculated_width = (calculated_height as f32 * target_aspect_ratio) as i32;
    }

    let vp_x = (width / 2) - (calculated_width /2);
    let vp_y = (height / 2) - (calculated_height /2);

    unsafe {
        gl::Viewport(vp_x, vp_y, calculated_width, calculated_height);
        gl::Scissor(vp_x, vp_y, calculated_width, calculated_height);        
    }
}

fn get_display_mode(video_subsystem: &VideoSubsystem, option: &EngineOption) -> DisplayMode {
    for i in 0..video_subsystem.num_display_modes(0).unwrap() {
        let display_mode = video_subsystem.display_mode(0,i).unwrap();
        if display_mode.w == option.screen_width as i32 && display_mode.h == option.screen_height as i32 {
            return display_mode;
        }
    }

    panic!("No DisplayMode available for width {} and height {}", option.screen_width, option.screen_height);
}

pub fn option_from_json(json_file: &str) -> EngineOption {
    let option_json = text_loader::load(json_file);
    serde_json::from_str(&option_json.data).unwrap()
}

pub fn resources_declaration_from_json(json_file: &str) -> ResourceDeclaration {
    let resources_json = text_loader::load(json_file);
    serde_json::from_str(&resources_json.data).unwrap()
}