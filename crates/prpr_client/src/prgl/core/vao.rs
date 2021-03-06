use super::*;
use std::collections::HashMap;
pub struct Vao<T: BufferAttribute> {
  v_buffer: VertexBuffer<T>,
  i_buffer: Option<IndexBuffer>,
  shader_id_to_raw_vao: SRwLock<HashMap<u64, RawVao>>,
}
pub trait VaoTrait {
  fn bind(&self, cmd: &mut Command);
}
impl<T: BufferAttribute> Vao<T> {
  pub fn new(v_buffer: VertexBuffer<T>, i_buffer: IndexBuffer) -> Self {
    Self {
      v_buffer,
      i_buffer: Some(i_buffer),
      shader_id_to_raw_vao: SRwLock::new(HashMap::new()),
    }
  }
  pub fn new_without_index_buffer(v_buffer: VertexBuffer<T>) -> Self {
    Self {
      v_buffer,
      i_buffer: None,
      shader_id_to_raw_vao: SRwLock::new(HashMap::new()),
    }
  }
  pub fn draw_command(&self) -> DrawCommand {
    if let Some(i_buffer) = &self.i_buffer {
      DrawCommand::DrawIndexed {
        first: 0,
        count: i_buffer.len() as i32,
      }
    } else {
      DrawCommand::Draw {
        first: 0,
        count: self.v_buffer.len() as i32,
      }
    }
  }
  // pub fn draw_instanced_command() -> DrawCommand {}
}
impl<T: BufferAttribute> VaoTrait for Vao<T> {
  fn bind(&self, cmd: &mut Command) {
    if let Some(shader) = cmd.current_shader() {
      let id = shader.id();
      let mut lock = self.shader_id_to_raw_vao.write();
      if let Some(raw_vao) = lock.get(&id) {
        cmd.set_vao(&raw_vao);
        return;
      }
      let i_buffer = self.i_buffer.as_ref().map(|x| x.raw_buffer());
      let raw_vao = RawVao::new(
        shader.raw_program().raw_program(),
        Some((self.v_buffer.template(), self.v_buffer.raw_buffer())),
        i_buffer,
      );
      cmd.set_vao(&raw_vao);
      lock.insert(id, raw_vao);
    }
  }
}
impl<T: BufferAttribute> VaoTrait for SOwner<Vao<T>> {
  fn bind(&self, cmd: &mut Command) {
    self.read().bind(cmd);
  }
}
impl<T: BufferAttribute> VaoTrait for SReader<Vao<T>> {
  fn bind(&self, cmd: &mut Command) {
    self.read().bind(cmd);
  }
}
