"use strict"

function draw(ctx, canvas_w, canvas_h, data) {
  let img = new ImageData(new Uint8ClampedArray(data),canvas_w, 500);
  ctx.putImageData(img, 0, 0);
}

const mandelbrot = import("../pkg").catch(console.error);

// wasmの読み込みは非同期で行われる
// Promiseで読み込み完了を待ってbutton要素のonClickに登録
Promise.all([mandelbrot]).then(async function([{main, generate_mandelbrot_set, draw_mandelbrot_set}]) {
  console.log("finished loading wasm");
  main();
  const renderBtn = document.getElementById("render");
  renderBtn.addEventListener("click", () => {
    draw_mandelbrot_set();

    let wasmResult = null;
    {
      const CANVAS_ID = "canvas_hybrid";
      let canvas = document.getElementById(CANVAS_ID);
      let context = canvas.getContext("2d");
      const canvasWidth = canvas.width;
      const canvasHeight = canvas.height;

      wasmResult = generate_mandelbrot_set(canvasWidth)
      draw(context, canvasHeight, canvasHeight, wasmResult);
    }
  })
});