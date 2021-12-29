use super::*;

// WARN: 多分別スレッドから実行できない
use once_cell::sync::OnceCell;
static INSTANCE: OnceCell<WholeScreen> = OnceCell::new();
unsafe impl Send for WholeScreen {}
unsafe impl Sync for WholeScreen {}

pub struct WholeScreen {
  max_width: i32,
  max_height: i32,
  width: RwLock<i32>,
  height: RwLock<i32>,
  is_size_changed: RwLock<bool>,
}
impl WholeScreen {
  pub fn get() -> &'static Self {
    INSTANCE
      .get()
      .expect("system::WholeScreen is not initialized")
  }
  pub fn initialize() {
    // 一度生成したら固定
    let screen = js::html::screen();
    let instance = Self {
      max_width: screen.width().unwrap(),
      max_height: screen.height().unwrap(),
      width: RwLock::new(1),
      height: RwLock::new(1),
      is_size_changed: RwLock::new(true),
    };
    INSTANCE.set(instance).ok();
  }
  pub fn max_width() -> i32 {
    Self::get().max_width
  }
  pub fn max_height() -> i32 {
    Self::get().max_height
  }
  pub fn width() -> i32 {
    *Self::get().width.read().unwrap()
  }
  pub fn height() -> i32 {
    *Self::get().height.read().unwrap()
  }
  pub fn is_size_changed() -> bool {
    *Self::get().is_size_changed.read().unwrap()
  }
  pub fn viewport() -> math::Rect<i32> {
    let width = Self::width();
    let height = Self::height();
    math::Rect::new(
      (Self::max_width() - width) / 2,
      (Self::max_height() - height) / 2,
      width,
      height,
    )
  }
  pub fn max_viewport() -> math::Rect<i32> {
    math::Rect::new(0, 0, Self::max_width(), Self::max_height())
  }
  pub fn update_size(width: i32, height: i32) {
    let pre_width = Self::width();
    let pre_height = Self::height();
    if pre_width == width && pre_height == height {
      return;
    }
    *Self::get().width.write().unwrap() = width;
    *Self::get().height.write().unwrap() = height;
    *Self::get().is_size_changed.write().unwrap() = true;
  }
}
