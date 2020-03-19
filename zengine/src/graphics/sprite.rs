use std::sync::Arc;
use crate::graphics::texture::Texture;
use crate::graphics::material::Material;
use crate::math::matrix4x4::Matrix4x4;
use crate::math::vector3::Vector3;
use crate::gl_utilities::gl_buffer::AttributeInfo;
use crate::gl_utilities::gl_buffer::GLBuffer;
use crate::gl_utilities::shader::Shader;
use crate::graphics::vertex::Vertex;

pub struct Sprite {

    pub width: f32,
    pub height: f32,

    origin: Vector3,

    u_color_location: i32,  
    u_model_location: i32,
    u_diffuse_location: i32,

    buffer: GLBuffer,
    vertices: [Vertex; 6],

    pub material: Material,

    pub shader_name: String
}

impl Sprite {
    pub fn new(shader_name: &str, material: Material, width: Option<f32>, height: Option<f32>) -> Sprite {
        Sprite {
            shader_name: String::from(shader_name),
            width: match width { Some(w) => w, _ => 10.0 },
            height: match height { Some(h) => h, _ => 10.0 },

            origin: Vector3::zero(),

            u_color_location: -1,
            u_model_location: -1,
            u_diffuse_location: -1,

            buffer: GLBuffer::new(),

            vertices: [Vertex::new(0.0, 0.0, 0.0, 0.0, 0.0); 6],

            material: material
        }
    }

    pub fn get_origin(&self) -> &Vector3 {
        &self.origin
    }

    pub fn set_origin(&mut self, value: Vector3) {
        self.origin = value;
        self.calculate_vertices();
    }

    pub fn load(&mut self, shader: &Shader, texture: Arc<Texture>) {
        let a_position_location = shader.get_attribute_location("a_position");
        let a_tex_coord_location = shader.get_attribute_location("a_tex_coord");

        self.u_color_location = shader.get_uniform_location("u_tint");
        self.u_model_location = shader.get_uniform_location("u_model");
        self.u_diffuse_location = shader.get_uniform_location("u_diffuse");

        self.buffer.configure(
            vec![
                AttributeInfo {
                    location: a_position_location,
                    component_size: 3
                },
                AttributeInfo {
                    location: a_tex_coord_location,
                    component_size: 2
                }
            ],
            false
        );

        self.calculate_vertices();

        self.material.set_texture(texture);        
    }

    pub fn calculate_vertices(&mut self) {
        let min_x = -(self.width * self.origin.x);
        let max_x = self.width * (1.0 - self.origin.x);
        
        let min_y = -(self.height * self.origin.y);
        let max_y = self.height * (1.0 - self.origin.y);

        self.vertices[0] = Vertex::new(min_x, min_y, 0.0, 0.0, 0.0);
        self.vertices[1] = Vertex::new(min_x, max_y, 0.0, 0.0, 1.0);
        self.vertices[2] = Vertex::new(max_x, max_y, 0.0, 1.0, 1.0);

        self.vertices[3] = Vertex::new(max_x, max_y, 0.0, 1.0, 1.0);
        self.vertices[4] = Vertex::new(max_x, min_y, 0.0, 1.0, 0.0);
        self.vertices[5] = Vertex::new(min_x, min_y, 0.0, 0.0, 0.0);

        self.buffer.upload(
            &self.vertices.iter().flat_map(|v| vec![v.position.x, v.position.y, v.position.z, v.tex_coord.x, v.tex_coord.y]).collect::<Vec<f32>>()
        );
    }

    pub fn draw(&self, model: &Matrix4x4) {
        unsafe {
            gl::UniformMatrix4fv(
                self.u_model_location,
                1,
                gl::FALSE,
                model.data.as_ptr()
            );
        }

        self.material.use_material(self.u_color_location, self.u_diffuse_location);

        self.buffer.draw();
    }
}
