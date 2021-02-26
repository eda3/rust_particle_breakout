extern crate js_sys;
use color_space::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

struct Particle {
  x: f64,
  y: f64,
  vx: f64,
  vy: f64,
  color: String,
}
impl Particle {
  pub fn new(&mut self, _x: f64, _y: f64) {
    self.x = _x;
    self.y = _y;
  }
}

#[wasm_bindgen]
struct Bar {
  x: f64,
  y: f64,
  width: f64,
  height: f64,
  col: JsValue,
}


#[wasm_bindgen]
impl Bar {
  pub fn draw(&self, context: &CanvasRenderingContext2d) {
    context.set_fill_style(&self.col);
    context.fill_rect(self.x, self.y, self.width, self.height);
  }
  pub fn remove(&self, context: &CanvasRenderingContext2d) {
    context.clear_rect(0.0, 0.0, 500.0, 500.0);
  }
  pub fn move_right(&mut self) {
    self.x += 10.0;
  }
  pub fn move_left(&mut self) {
    self.x -= 10.0;
  }
  pub fn tick(&self, context: &CanvasRenderingContext2d) {
    self.remove(context);
    self.draw(context);
  }
}

fn window() -> web_sys::Window {
  web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
  window().document().expect("should have a document on window")
}

fn canvas() -> web_sys::HtmlCanvasElement{
  document()
    .get_element_by_id("canvas_hybrid")
    .unwrap()
    .dyn_into::<HtmlCanvasElement>()
    .unwrap()
}

fn context() -> web_sys::CanvasRenderingContext2d {
  canvas()
    .get_context("2d")
    .unwrap()
    .unwrap()
    .dyn_into::<CanvasRenderingContext2d>()
    .unwrap()
}

#[wasm_bindgen]
pub struct Game {
  bar: Bar,
}

#[wasm_bindgen]
impl Game {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Game {
    let bar = Bar {
      x: (canvas().width() / 2) as f64,
      y: (canvas().height() - 50) as f64,
      width: 100.0,
      height: 1.0,
      col: JsValue::from_str("green"),
    };

    Game {
      bar,
    }
  }

  #[wasm_bindgen(method)]
  pub fn render_loop(&self) {
    console_log!("render_loop start");
    let game_width = 500.0;
    let game_height = 500.0;
    let context = context();

    context.clear_rect(0.0, 0.0, game_width, game_height);
    &self.bar.draw(&context);
  }

  #[wasm_bindgen(method)]
  pub fn on_key_right(&mut self) {
    &self.bar.move_right();
  }

  #[wasm_bindgen(method)]
  pub fn on_key_left(&mut self) {
    &self.bar.move_left();
  }

}

#[wasm_bindgen]
pub fn generate_mandelbrot_set(canvas_w: usize) -> Vec<u8> {
  // JSの8bit符号なし整数の配列であるUint8ClampledAllay型をつくりたいため、Vec<u8>で色情報を作る
  let mut data = vec![];
  for i in 0..500 {
    for _ in 0..canvas_w {
      // let v = (255 % j) as f64;
      let v = i as f64;

      let hsv = Hsv::new(v as f64, 1.0, 1.0);
      let rgb = Rgb::from_color(&hsv);

      data.push(rgb.r as u8); // R
      data.push(rgb.g as u8); // G
      data.push(rgb.b as u8); // B
      data.push(255); // A
    }
  }
  data
}

#[wasm_bindgen]
pub fn draw_mandelbrot_set() {
  // const CANVAS_ID :&str = "canvas_wasm";
  const CANVAS_ID: &str = "canvas_hybrid";
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
  // HtmlCanvasElement型のAPIを使うためにElement型からキャスト
  let canvas: web_sys::HtmlCanvasElement = canvas
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|_| ())
    .unwrap();
  // Object型からCanvasRenderingContext2d型にキャスト
  let context = canvas
    .get_context("2d") // Result<Option<Object>, JsValue>
    .unwrap() // Option<Object>
    .unwrap() // Object
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    .unwrap();
  let canvas_w = canvas.width() as usize;

  let mut result = generate_mandelbrot_set(canvas_w);
  let data = ImageData::new_with_u8_clamped_array_and_sh(
    Clamped(&mut result),
    canvas.width(),
    canvas.height(),
  );
  if let Ok(data) = data {
    let _ = context.put_image_data(&data, 0.0, 0.0);
  }
}
