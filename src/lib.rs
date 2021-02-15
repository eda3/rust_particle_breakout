mod utils;
extern crate js_sys;
use color_space::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, KeyboardEvent};

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

struct Bar {
  x: f64,
  y: f64,
  width: f64,
  height: f64,
  col: JsValue,
}

impl Bar {
  pub fn draw(&self, context: CanvasRenderingContext2d) {
    context.set_fill_style(&self.col);
    context.fill_rect(self.x, self.y, self.width, self.height);
  }
}

#[wasm_bindgen]
pub fn main() {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let canvas = document
    .get_element_by_id("canvas_hybrid")
    .unwrap()
    .dyn_into::<HtmlCanvasElement>()
    .unwrap();
  let context = canvas
    .get_context("2d")
    .unwrap()
    .unwrap()
    .dyn_into::<CanvasRenderingContext2d>()
    .unwrap();

  let bar = Bar {
    x: (canvas.width() / 2) as f64,
    y: (canvas.height() - 50) as f64,
    width: 100.0,
    height: 10.0,
    col: JsValue::from_str("green"),
  };
  bar.draw(context);
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
