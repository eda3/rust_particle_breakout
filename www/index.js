import { Game }  from "../pkg/rust_mandelbrot";

// let game = Game.new();
let game = new Game();
// console.log(game)
// game.main()
// main(game)
// console.log(game)

let word = "WORD"
const loop = word => {
  game.render_loop()
};
const timer = setInterval(loop.bind(undefined, game), 1000 / 60);

function keyboardControls(event) {
  console.log("keyboardControls")
  switch (event.key) {
    case "ArrowLeft":
      game.on_key_left();
      break;
    case "ArrowRight":
      game.on_key_right();
      break;
    default:
  }
  // last = Date.now();
}
document.addEventListener("keydown", keyboardControls);



// game.main();

// const game = import("../pkg/").catch("console error");
// Promise.all([game]).then(async function([{Game}]) {
//   console.log("finished loading wasm");
//   let game = new Game();
//   game.main();
// });


// wasmの読み込みは非同期で行われる
// Promiseで読み込み完了を待ってbutton要素のonClickに登録
// Promise.all([mandelbrot]).then(async function([{Game}]) {
  // console.log("finished loading wasm");
  // console.log("setInterval");
  // let game = new Game();
  // game.main();
  //
  // const renderBtn = document.getElementById("render");
  //
  // renderBtn.addEventListener("click", () => {
  //   draw_mandelbrot_set();

    // let wasmResult = null;
    // {
    //   const CANVAS_ID = "canvas_hybrid";
    //   let canvas = document.getElementById(CANVAS_ID);
    //   let context = canvas.getContext("2d");
    //   const canvasWidth = canvas.width;
    //   const canvasHeight = canvas.height;
    //
    //   wasmResult = generate_mandelbrot_set(canvasWidth)
    //   draw(context, canvasHeight, canvasHeight, wasmResult);
    // }
  // })
// });
