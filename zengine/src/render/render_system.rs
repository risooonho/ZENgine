use crate::core::system::Read;
use crate::core::system::System;
use crate::core::Store;
use crate::gl_utilities::gl_buffer::AttributeInfo;
use crate::gl_utilities::gl_buffer::GLBuffer;
use crate::render::Background;
use crate::render::WindowSpecs;
use sdl2::video::{DisplayMode, FullscreenType, GLContext, GLProfile, Window};
use sdl2::VideoSubsystem;

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

pub struct RenderSystem {
    window_specs: WindowSpecs,
    window: Option<Window>,
    ctx: Option<GLContext>,
    buffer: Option<GLBuffer>,
}

impl RenderSystem {
    pub fn new(specs: WindowSpecs) -> Self {
        RenderSystem {
            window_specs: specs,
            buffer: None,
            window: None,
            ctx: None,
        }
    }

    fn create_window_and_opengl_context(&mut self, video_subsystem: &VideoSubsystem) {
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        if cfg!(target_os = "macos") {
            gl_attr.set_context_version(4, 1);
        } else {
            gl_attr.set_context_version(4, 6);
        }
        gl_attr.set_double_buffer(true);

        let mut window = video_subsystem
            .window(
                self.window_specs.title.as_ref(),
                self.window_specs.width,
                self.window_specs.height,
            )
            .opengl()
            .allow_highdpi()
            .build()
            .unwrap();

        if self.window_specs.fullscreen {
            let display_mode = self.get_display_mode(&video_subsystem);
            window.set_display_mode(display_mode).unwrap();
            window.set_fullscreen(FullscreenType::True).unwrap();
        }

        self.ctx = Some(window.gl_create_context().unwrap());
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        println!(
            "Pixel format of the window's GL context: {:?}",
            window.window_pixel_format()
        );
        println!(
            "OpenGL Profile: {:?} - OpenGL version: {:?}",
            gl_attr.context_profile(),
            gl_attr.context_version()
        );

        self.window = Some(window);
    }

    fn get_display_mode(&self, video_subsystem: &VideoSubsystem) -> DisplayMode {
        for i in 0..video_subsystem.num_display_modes(0).unwrap() {
            let display_mode = video_subsystem.display_mode(0, i).unwrap();
            if display_mode.w == self.window_specs.width as i32
                && display_mode.h == self.window_specs.height as i32
            {
                return display_mode;
            }
        }

        panic!(
            "No DisplayMode available for width {} and height {}",
            self.window_specs.width, self.window_specs.height
        );
    }
}

impl<'a> System<'a> for RenderSystem {
    type Data = (Read<'a, Background>);

    fn init(&mut self, store: &mut Store) {
        let video_subsystem = store
            .get_resource::<VideoSubsystem>()
            .expect("No VideoSubsystem resource found");
        self.create_window_and_opengl_context(&video_subsystem);

        unsafe {
            if !cfg!(target_os = "macos") {
                gl::Enable(gl::DEBUG_OUTPUT);
                gl::DebugMessageCallback(Some(dbg_callback), std::ptr::null());
            }
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let mut buffer = GLBuffer::new();

        buffer.configure(
            vec![
                AttributeInfo {
                    location: 0, //TODO FIXME
                    component_size: 3,
                },
                AttributeInfo {
                    location: 1, //TODO FIXME
                    component_size: 2,
                },
            ],
            false,
        );

        self.buffer = Some(buffer);
    }

    fn run(&mut self, (background): Self::Data) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(
                background.color.r,
                background.color.g,
                background.color.b,
                background.color.a,
            );
        }

        if let Some(window) = &self.window {
            window.gl_swap_window();
        }
    }
}
