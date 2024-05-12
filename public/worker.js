// import init, {
//   init_bevy_app,
//   finish_init,
//   enter_frame,
//   create_window_by_canvas,
//   create_window_by_offscreen_canvas,
// } from "./bevy_in_web_worker.js";

// Worker 有自己的作用域，无法直接访问全局作用域的函数/对象, 无法使用 ES6 module.
importScripts("./bevy_in_web_worker.js");

const {
  init_bevy_app,
  finish_init,
  enter_frame,
  create_window_by_canvas,
  create_window_by_offscreen_canvas,
} = wasm_bindgen;

let appHandle = 0;
let initFinished = 0;

async function init_wasm_in_worker() {
  // 装载 wasm 文件
  await wasm_bindgen("./bevy_in_web_worker_bg.wasm");

  // 创建 app
  appHandle = init_bevy_app();

  // 监听主线程发来的消息.
  self.onmessage = async (event) => {
    let data = event.data;
    switch (data.ty) {
      case "init":
        let canvas = data.canvas;
        initApp(canvas, data.devicePixelRatio);
        break;
      case "enterFrame":
        enterFrame();
        break;
      default:
        break;
    }
  };

  // 通知主线程 worker 已准备好
  self.postMessage({ ty: "workerIsReady" });
}
init_wasm_in_worker();

async function initApp(offscreenCanvas, devicePixelRatio) {
  // 创建渲染窗口
  //   let canvas = document.getElementById("app-canvas");
  //   let offscreenCanvas = canvas.transferControlToOffscreen();
  create_window_by_offscreen_canvas(
    appHandle,
    offscreenCanvas,
    devicePixelRatio
  );

  //   requestAnimationFrame(enterFrame);
}

// 开始渲染帧
// TODO: 前 3 帧之间先等待 1 秒
let isLastFrameCompleted = 1;
let frameIndex = 0;
let frameCount = 0;
function enterFrame() {
  // 当 app 准备好时，执行 app 的帧循环
  if (initFinished > 0 && isLastFrameCompleted > 0) {
    if (frameIndex >= 3 || (frameIndex < 3 && frameCount % 60 == 0)) {
      isLastFrameCompleted = 0;
      isLastFrameCompleted = enter_frame(appHandle);
      console.log(frameIndex, frameCount);
      isFirstFrame = false;
      frameIndex++;
    }
    frameCount++;
  } else {
    initFinished = finish_init(appHandle);
  }
}
