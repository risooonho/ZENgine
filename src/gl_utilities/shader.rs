use std::ffi::CString;
use std::collections::HashMap;
use crate::assets::text_loader;
use std::str::FromStr;

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub struct Shader {
    pub name: String,
    pub program: u32,
    attributes: HashMap<String, u32>,
    uniforms: HashMap<String, i32>
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
        println!("destroyed shader {}", self.name);
    }
}

impl Shader {
    pub fn new(name: &str) -> Shader {
        let mut shader = Shader {
            name: String::from(name),
            program: 0,
            attributes: HashMap::new(),
            uniforms: HashMap::new()
        };

        shader.load(
            &CString::new(text_loader::load(&format!("{}.vert", name)).data).expect("CString::new failed"),
            &CString::new(text_loader::load(&format!("{}.frag", name)).data).expect("CString::new failed"),
        );

        shader
    }

    pub fn create_basic_shader() -> Shader {
        let mut shader = Shader {
            name: String::from("basic"),
            program: 0,
            attributes: HashMap::new(),
            uniforms: HashMap::new()
        };

        shader.load(
            &CString::new(include_str!("../basic.vert")).expect("CString::new failed"),
            &CString::new(include_str!("../basic.frag")).expect("CString::new failed"),
        );

        shader
    }

    pub fn use_shader(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn get_attribute_location(&self, name: &str) -> u32 {
        match self.attributes.get(name) {
            Some(&attribute) => return attribute,
            _ => panic!("Unable to find attribute name {} in shader name {}", name, self.name)
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> i32 {
        match self.uniforms.get(name) {
            Some(&uniform) => return uniform,
            _ => panic!("Unable to find uniform name {} in shader name {}", name, self.name)
        }
    }

    fn load(&mut self, vertex_source: &CString, fragment_source: &CString) {
        let vertex_shader = Shader::load_shader(vertex_source, gl::VERTEX_SHADER);
        let fragment_shader = Shader::load_shader(fragment_source, gl::FRAGMENT_SHADER);

        self.program = Shader::create_program(vertex_shader, fragment_shader);

        self.detect_attributes();
        self.detect_uniforms();

        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }
    }

    fn load_shader(source: &CString, shader_type: gl::types::GLenum) -> u32 {
        let shader_id = unsafe { gl::CreateShader(shader_type) };

        unsafe {
            gl::ShaderSource(shader_id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(shader_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error_msg = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(
                    shader_id,
                    len,
                    std::ptr::null_mut(),
                    error_msg.as_ptr() as *mut gl::types::GLchar
                );
            }

            println!("{}", error_msg.into_string().expect("into_string() failed"));
        }

        shader_id
    }

    fn create_program(vertex_shader: u32, fragment_shader: u32) -> u32 {        
        let program_id = unsafe { gl::CreateProgram() };
        
        unsafe {            
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);

            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error_msg = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error_msg.as_ptr() as *mut gl::types::GLchar
                );
            }

            println!("{}", error_msg.into_string().expect("into_string() failed"));
        }

        unsafe {
            gl::DetachShader(program_id, vertex_shader);
            gl::DetachShader(program_id, fragment_shader);
        }

        program_id
    }

    fn detect_attributes(&mut self) {
        unsafe {
            let mut attributes_number: gl::types::GLint = 0;
            gl::GetProgramiv(self.program, gl::ACTIVE_ATTRIBUTES, &mut attributes_number);

            for i in 0..attributes_number {
                let mut size: gl::types::GLint = 0;         // variable size
                let mut var_type: gl::types::GLenum = 0;    // variable type (float, vec3, vec4, mat4, etc)
                const BUF_SIZE: usize = 16;                 // maximum name length
                let name = [0; BUF_SIZE];
                let mut length: gl::types::GLsizei = 0;     // name length

                gl::GetActiveAttrib(
                    self.program,
                    i as gl::types::GLuint,
                    BUF_SIZE as gl::types::GLint,
                    &mut length,
                    &mut size,
                    &mut var_type,
                    name.as_ptr() as *mut gl::types::GLchar
                );
                if length == 0 {
                    break;
                }

                let location = gl::GetAttribLocation(
                    self.program,
                    name.as_ptr() as *mut gl::types::GLchar
                ) as u32;

                self.attributes.insert(
                    String::from_str(
                        std::ffi::CStr::from_ptr(name.as_ptr()).to_str().unwrap()
                    ).unwrap(),
                    location
                );
            }
        }
    }

    fn detect_uniforms(&mut self) {
        unsafe {
            let mut uniforms_number: gl::types::GLint = 0;
            gl::GetProgramiv(self.program, gl::ACTIVE_UNIFORMS, &mut uniforms_number);

            for i in 0..uniforms_number {
                let mut size: gl::types::GLint = 0;         // variable size
                let mut var_type: gl::types::GLenum = 0;    // variable type (float, vec3, vec4, mat4, etc)
                const BUF_SIZE: usize = 16;                 // maximum name length
                let name = [0; BUF_SIZE];
                let mut length: gl::types::GLsizei = 0;     // name length

                gl::GetActiveUniform(
                    self.program,
                    i as gl::types::GLuint,
                    BUF_SIZE as gl::types::GLint,
                    &mut length,
                    &mut size,
                    &mut var_type,
                    name.as_ptr() as *mut gl::types::GLchar
                );
                if length == 0 {
                    break;
                }

                let location = gl::GetUniformLocation(
                    self.program,
                    name.as_ptr() as *mut gl::types::GLchar
                );

                self.uniforms.insert(
                    String::from_str(
                        std::ffi::CStr::from_ptr(name.as_ptr()).to_str().unwrap()
                    ).unwrap(),
                    location
                );
            }
        }
    }
}
