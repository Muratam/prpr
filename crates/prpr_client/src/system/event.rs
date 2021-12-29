use super::*;
use collections::BitSet64;
use std::sync::mpsc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub enum MouseState {
  IsDown,
  IsLeftClicked,
  IsRightClicked,
  IsDoubleClicked,
}
#[derive(Clone, Copy)]
enum MouseEvent {
  Move,
  Down,
  Up,
  Click,
  DoubleClick,
  ContextMenu,
}
struct MouseEventInfo {
  x: i32,
  y: i32,
  event: MouseEvent,
}
use once_cell::sync::OnceCell;
static INSTANCE: OnceCell<RwLock<EventHolderImpl>> = OnceCell::new();
unsafe impl Send for EventHolderImpl {}
unsafe impl Sync for EventHolderImpl {}
pub struct EventHolderImpl {
  mouse_x: i32,
  mouse_y: i32,
  mouse_pre_x: i32,
  mouse_pre_y: i32,
  mouse_state: BitSet64,
  mouse_rx: mpsc::Receiver<MouseEventInfo>,
}
impl EventHolderImpl {
  pub fn read_global() -> RwLockReadGuard<'static, Self> {
    INSTANCE
      .get()
      .expect("event holder is not initialized")
      .read()
      .unwrap()
  }
  pub fn write_global() -> RwLockWriteGuard<'static, Self> {
    INSTANCE
      .get()
      .expect("event holder is not initialized")
      .write()
      .unwrap()
  }
  pub fn initialize_global() {
    INSTANCE.set(RwLock::new(Self::new(&html::body()))).ok();
  }
  pub fn new(elem: &web_sys::HtmlElement) -> Self {
    let (mouse_tx, mouse_rx) = mpsc::channel::<MouseEventInfo>();
    let mut result = Self {
      mouse_x: 0,
      mouse_y: 0,
      mouse_pre_x: 0,
      mouse_pre_y: 0,
      mouse_state: BitSet64::new(),
      mouse_rx,
    };
    result.setup_mouse_events(elem, mouse_tx);
    result
  }
  fn setup_mouse_events(&mut self, elem: &web_sys::HtmlElement, tx: mpsc::Sender<MouseEventInfo>) {
    let tx = Arc::new(tx);
    let setup_callback = |event_type: MouseEvent, event_name: &str, prevent_default: bool| {
      let tx = tx.clone();
      let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        tx.send(MouseEventInfo {
          x: event.offset_x(),
          y: event.offset_y(),
          event: event_type,
        })
        .ok();
        if prevent_default {
          event.prevent_default();
        }
      }) as Box<dyn FnMut(_)>);
      elem
        .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
        .ok();
      closure.forget();
    };
    setup_callback(MouseEvent::Move, "mousemove", false);
    setup_callback(MouseEvent::Up, "mouseup", false);
    setup_callback(MouseEvent::Down, "mousedown", false);
    setup_callback(MouseEvent::Click, "click", false);
    setup_callback(MouseEvent::DoubleClick, "dblclick", false);
    setup_callback(MouseEvent::ContextMenu, "contextmenu", true);
  }
  pub fn mouse_x(&self) -> i32 {
    self.mouse_x
  }
  pub fn mouse_y(&self) -> i32 {
    self.mouse_y
  }
  pub fn mouse_dx(&self) -> i32 {
    self.mouse_x - self.mouse_pre_x
  }
  pub fn mouse_dy(&self) -> i32 {
    self.mouse_y - self.mouse_pre_y
  }
  pub fn mouse_state(&self, state: MouseState) -> bool {
    self.mouse_state.get(state as usize)
  }
  fn set_mouse_state(&mut self, state: MouseState, value: bool) {
    self.mouse_state.set(state as usize, value);
  }
}
impl Updatable for EventHolderImpl {
  fn update(&mut self) {
    // mouse state
    let is_mouse_down = self.mouse_state(MouseState::IsDown);
    self.mouse_state.set_all_false();
    self.set_mouse_state(MouseState::IsDown, is_mouse_down);
    self.mouse_pre_x = self.mouse_x;
    self.mouse_pre_y = self.mouse_y;
    while let Ok(info) = self.mouse_rx.try_recv() {
      self.mouse_x = info.x;
      self.mouse_y = info.y;
      match info.event {
        MouseEvent::Move => {}
        // めっちゃはやいとだめかも？反応しないことが多ければwhile を if に
        MouseEvent::Down => self.set_mouse_state(MouseState::IsDown, true),
        MouseEvent::Up => self.set_mouse_state(MouseState::IsDown, false),
        MouseEvent::Click => self.set_mouse_state(MouseState::IsLeftClicked, true),
        MouseEvent::ContextMenu => self.set_mouse_state(MouseState::IsRightClicked, true),
        MouseEvent::DoubleClick => self.set_mouse_state(MouseState::IsDoubleClicked, true),
      }
    }
  }
}
