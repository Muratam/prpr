use super::*;

crate::shader_attr! {
  struct CameraAttribute {
    view_mat: mat4,
    proj_mat: mat4,
    view_proj_mat: mat4,
    camera_pos: vec3,
    camera_dummy: float,
    camera_target_pos: vec3,
    camera_dummy2: float,
    fovy: float,
    aspect_ratio: float,
    near: float,
    far: float,
  }
}
pub struct CameraData {
  pub camera_pos: Vec3,
  pub camera_target_pos: Vec3,
  pub fovy: f32,
  pub aspect_ratio: f32,
  pub near: f32,
  pub far: f32,
}
impl CameraData {
  fn to_view_mat(camera_pos: Vec3, target_pos: Vec3) -> Mat4 {
    Mat4::look_at_rh(camera_pos, target_pos, Vec3::Y)
  }
  fn to_proj_mat(fovy: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
    Mat4::perspective_rh(fovy, aspect_ratio, near, far)
  }
}
impl Default for CameraData {
  fn default() -> Self {
    Self {
      camera_pos: Vec3::ONE,
      camera_target_pos: Vec3::ZERO,
      fovy: 3.141592 * 0.25,
      aspect_ratio: 1.0,
      near: 0.01,
      far: 1000.0,
    }
  }
}

impl RefInto<CameraAttribute> for CameraData {
  fn ref_into(&self) -> CameraAttribute {
    let view_mat = Self::to_view_mat(self.camera_pos, self.camera_target_pos);
    let proj_mat = Self::to_proj_mat(self.fovy, self.aspect_ratio, self.near, self.far);
    let view_proj_mat = proj_mat * view_mat;
    CameraAttribute {
      view_mat: view_mat,
      proj_mat: proj_mat,
      view_proj_mat: view_proj_mat,
      camera_pos: self.camera_pos,
      camera_dummy: 0.0,
      camera_target_pos: self.camera_target_pos,
      camera_dummy2: 0.0,
      fovy: self.fovy,
      aspect_ratio: self.aspect_ratio,
      near: self.near,
      far: self.far,
    }
  }
}
pub type Camera = IntoUniformBufferTemplate<CameraAttribute, CameraData>;
