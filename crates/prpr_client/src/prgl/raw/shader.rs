use super::*;

#[derive(PartialEq)]
pub enum ShaderType {
  VertexShader,
  FragmentShader,
}

pub struct RawShader {
  shader: web_sys::WebGlShader,
  shader_type: ShaderType,
}
impl RawShader {
  pub fn new(gl: &GlContext, code: &str, shader_type: ShaderType) -> Option<Self> {
    let create_flag = match &shader_type {
      ShaderType::VertexShader => gl::VERTEX_SHADER,
      ShaderType::FragmentShader => gl::FRAGMENT_SHADER,
    };
    let shader = gl
      .create_shader(create_flag)
      .expect("failed to create shader");
    gl.shader_source(&shader, code);
    gl.compile_shader(&shader);
    if gl
      .get_shader_parameter(&shader, gl::COMPILE_STATUS)
      .is_falsy()
    {
      if let Some(info_log) = gl.get_shader_info_log(&shader) {
        log::error("failed to compile shader");
        log::error(code);
        log::error(info_log);
      }
      return None;
    }
    return Some(Self {
      shader,
      shader_type,
    });
  }
}
use std::sync::atomic::{AtomicUsize, Ordering};
static RAW_SHADER_PROGRAM_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub struct RawShaderProgram {
  program: web_sys::WebGlProgram,
  program_id: usize,
}
pub struct RawShaderProgramContents {
  pub vertex_shader: Option<RawShader>,
  pub fragment_shader: Option<RawShader>,
}
impl RawShaderProgram {
  pub fn new(gl: &GlContext, shaders: &RawShaderProgramContents) -> Option<Self> {
    let program = gl
      .create_program()
      .expect("failed to create shader program");
    if let Some(shader) = &shaders.vertex_shader {
      if shader.shader_type != ShaderType::VertexShader {
        log::error("Not Vertex Shader");
        return None;
      }
      gl.attach_shader(&program, &shader.shader);
    }
    if let Some(shader) = &shaders.fragment_shader {
      if shader.shader_type != ShaderType::FragmentShader {
        log::error("Not Fragment Shader");
        return None;
      }
      gl.attach_shader(&program, &shader.shader);
    }
    gl.link_program(&program);
    if gl
      .get_program_parameter(&program, gl::LINK_STATUS)
      .is_falsy()
    {
      if let Some(info_log) = gl.get_program_info_log(&program) {
        log::error("failed to link shader");
        log::error(info_log);
      }
      return None;
    }
    gl.validate_program(&program);
    if gl
      .get_program_parameter(&program, gl::VALIDATE_STATUS)
      .is_falsy()
    {
      if let Some(info_log) = gl.get_program_info_log(&program) {
        log::error("failed to validate shader");
        log::error(info_log);
      }
      return None;
    }
    let program_id = RAW_SHADER_PROGRAM_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    return Some(Self {
      program,
      program_id,
    });
  }
  pub fn raw_program(&self) -> &web_sys::WebGlProgram {
    &self.program
  }
  pub fn raw_program_id(&self) -> usize {
    self.program_id
  }
}