use super::*;
// - DescriptorContext にどんどんconsしていって、適用する。
//   - Stackになるので、動作がわかりやすいはず
//   - RenderPass・Pipelineも持つ
// TODO: ShaderBind時にデフォルトテクスチャを貼る

pub struct Descriptor {
  vao: Option<VaoDynPtr>,
  u_buffers: Vec<UniformBufferDynPtr>,
  // u_textures: Vec<Texture>
}
impl Descriptor {
  pub fn new() -> Descriptor {
    Self {
      vao: None,
      u_buffers: Vec::new(),
    }
  }
  pub fn set_vao(&mut self, vao: &VaoDynPtr) {
    self.vao = Some(Arc::clone(vao));
  }
  pub fn add_uniform_buffer(&mut self, buffer: &UniformBufferDynPtr) {
    self.u_buffers.push(Arc::clone(buffer));
  }
}
pub enum DescriptorContext<'a, 'b> {
  Cons {
    prior: &'a mut Descriptor,
    others: &'b mut DescriptorContext<'a, 'b>,
  },
  Nil,
}

impl<'a, 'b> DescriptorContext<'a, 'b> {
  pub fn cons(&'a mut self, prior: &'b mut Descriptor) -> DescriptorContext<'a, 'b> {
    Self::Cons {
      prior,
      others: self,
    }
  }
  pub fn bind(&mut self, program: &RawShaderProgram) {
    if let Self::Cons { prior, others } = self {
      others.bind(program);
      for u_buffer in &mut prior.u_buffers {
        u_buffer.write().unwrap().bind(program);
      }
      if let Some(vao) = &mut prior.vao {
        vao.write().unwrap().bind(program);
      } else {
        log::error("No Vertex Array Object");
        return;
      }
    }
  }
}
