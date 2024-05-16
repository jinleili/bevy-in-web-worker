// Worker 有自己的作用域，无法直接访问全局作用域的函数/对象, 无法使用 ES6 module.
importScripts("./bevy_in_web_worker.js");

const {
  init_bevy_app,
  is_preparation_completed,
  create_window_by_offscreen_canvas,
  enter_frame,
  mouse_move,
  set_hover,
  set_selection,
  left_bt_down,
  left_bt_up,
  set_auto_animation,
} = wasm_bindgen;

let appHandle = 0;
let initFinished = 0;
let isStoppedRunning = false;
let renderBlockTime = 1;

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
        createWorkerAppWindow(canvas, data.devicePixelRatio);
        break;

      case "startRunning":
        if (isStoppedRunning) {
          isStoppedRunning = false;
          // 开始帧循环
          requestAnimationFrame(enterFrame);
        }
        break;

      case "stopRunning":
        isStoppedRunning = true;
        break;

      case "mousemove":
        mouse_move(appHandle, data.x, data.y);
        break;

      case "hover":
        // 设置 hover（高亮） 效果
        set_hover(appHandle, data.list);
        break;

      case "select":
        // 设置 选中 效果
        set_selection(appHandle, data.list);
        break;

      case "leftBtDown":
        left_bt_down(appHandle, data.pickItem, data.x, data.y);
        break;

      case "leftBtUp":
        left_bt_up(appHandle);
        break;

      case "blockRender":
        renderBlockTime = data.blockTime;
        break;

      case "autoAnimation":
        set_auto_animation(appHandle, data.autoAnimation);
        break;

      default:
        break;
    }
  };

  // 通知主线程 worker 已准备好
  self.postMessage({ ty: "workerIsReady" });
}
init_wasm_in_worker();

function createWorkerAppWindow(offscreenCanvas, devicePixelRatio) {
  // 创建渲染窗口
  create_window_by_offscreen_canvas(
    appHandle,
    offscreenCanvas,
    devicePixelRatio
  );

  // 查询就绪状态
  getPreparationState();

  // 开始帧循环
  requestAnimationFrame(enterFrame);
}

/**
 * 开始渲染帧
 *
 * https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/requestAnimationFrame
 * requestAnimationFrame 是与 window 绘制同步的，在此处手动限制帧率会造成因与 window 刷新频率不一致而视觉画面不流畅
 *
 * TODO: 前 3 帧之间先等待 1 秒
 */
let frameIndex = 0;
let frameCount = 0;
let frameFlag = 0;

function enterFrame(_dt) {
  if (appHandle === 0 || isStoppedRunning) return;

  // 当 app 准备好时，执行 app 的帧循环
  if (initFinished > 0) {
    if (
      frameIndex >= frameFlag ||
      (frameIndex < frameFlag && frameCount % 60 == 0)
    ) {
      enter_frame(appHandle);
      frameIndex++;
    }
    frameCount++;
  } else {
    getPreparationState();
  }
  requestAnimationFrame(enterFrame);
}

/** 获取 bevy app 就绪状态 */
function getPreparationState() {
  initFinished = is_preparation_completed(appHandle);
}

/** 发送 ray pick 结果 */
function send_pick_from_worker(pickList) {
  self.postMessage({ ty: "pick", list: pickList });
}

/** 执行阻塞 */
function block_from_worker() {
  const start = performance.now();
  while (performance.now() - start < renderBlockTime) {}
}
