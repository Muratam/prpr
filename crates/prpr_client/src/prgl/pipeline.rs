use super::*;

/*
- Instance
  - RenderPass
    - &Texture
  - Pipeline
    - ShaderProgram
  - DescriptorSet
    - &Texture
    - &Buffer (bind by name)
*/

pub struct Pipeline {
  gl: Rc<GlContext>,
  // states
  draw_command: Option<DrawCommand>,
  primitive_topology: PrimitiveToporogy,
  shader: Option<Rc<Shader>>,
  descriptor: Descriptor,
}

impl Pipeline {
  pub fn new(gl: &Rc<GlContext>) -> Self {
    Self {
      gl: Rc::clone(gl),
      draw_command: None,
      primitive_topology: PrimitiveToporogy::Triangles,
      shader: None,
      descriptor: Descriptor::new(),
    }
  }

  pub fn draw(&mut self) {
    let gl = &self.gl;
    let mut outer_desc_ctx = DescriptorContext::Nil;
    if let Some(shader) = &self.shader {
      shader.use_program();
      outer_desc_ctx
        .cons(&mut self.descriptor)
        .bind(shader.raw_program());
    } else {
      log::error("No Shader Program");
      return;
    }
    let topology = self.primitive_topology as u32;
    if let Some(command) = &self.draw_command {
      match &command {
        DrawCommand::Draw { first, count } => {
          gl.draw_arrays(topology, *first, *count);
        }
        DrawCommand::DrawIndexed { first, count } => {
          assert_type_eq!(u32, IndexBufferType);
          gl.draw_elements_with_i32(topology, *count, gl::UNSIGNED_INT, *first);
        }
      }
    } else {
      log::error("No Draw Command");
      return;
    }
  }
  // set resource
  pub fn set_shader(&mut self, shader: &Rc<Shader>) {
    self.shader = Some(Rc::clone(shader));
  }
  pub fn set_vao(&mut self, vao: &VaoDynPtr) {
    self.descriptor.set_vao(vao);
  }
  pub fn add_uniform_buffer(&mut self, buffer: &UniformBufferDynPtr) {
    self.descriptor.add_uniform_buffer(buffer);
  }
  // draw
  pub fn set_draw(&mut self, first: i32, count: i32) {
    self.draw_command = Some(DrawCommand::Draw { first, count });
  }
  pub fn set_draw_indexed(&mut self, first: i32, count: i32) {
    self.draw_command = Some(DrawCommand::DrawIndexed { first, count });
  }
  pub fn set_draw_mode(&mut self, primitive_topology: PrimitiveToporogy) {
    self.primitive_topology = primitive_topology;
  }
}
