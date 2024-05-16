import init, {
  init_bevy_app,
  is_preparation_completed,
  create_window_by_canvas,
  enter_frame,
  mouse_move,
  left_bt_down,
  left_bt_up,
  set_hover,
  set_selection,
  release_app,
  set_auto_animation,
} from "./bevy_in_main_thread.js";

let appHandle = 0;
let initFinished = 0;
let isStoppedRunning = false;

async function launchMainApp() {
  await init();
  create_main_app();
}
launchMainApp();

function create_main_app() {
  if (appHandle > 0) return;

  appHandle = init_bevy_app();
  // 创建渲染窗口
  let devicePixelRatio = window.devicePixelRatio;
  create_window_by_canvas(appHandle, "main-thread-canvas", devicePixelRatio);

  // 开始动画
  requestAnimationFrame(enterFrame);
}

// 启动 engine 实例
window.start_main_app = () => {
  isStoppedRunning = false;
  requestAnimationFrame(enterFrame);
  setContainerOpacity("100%");
};

// 停止 engine 实例
window.stop_main_app = () => {
  //   release_app(appHandle);
  //   // 重置状态
  //   appHandle = 0;
  //   initFinished = 0;
  //   window.release_app = undefined;
  isStoppedRunning = true;
  setContainerOpacity("50%");
};

window.mouse_move = (x, y) => {
  if (initFinished > 0) mouse_move(appHandle, x, y);
};

window.left_bt_down = (pickItem, x, y) => {
  if (initFinished > 0) left_bt_down(appHandle, pickItem, x, y);
};

window.left_bt_up = () => {
  if (initFinished > 0) left_bt_up(appHandle);
};

window.set_hover = (list) => {
  if (initFinished > 0) set_hover(appHandle, list);
};

window.set_selection = (list) => {
  if (initFinished > 0) set_selection(appHandle, list);
};

window.set_main_app_auto_animation = (needsAnimate) => {
  if (initFinished > 0) set_auto_animation(appHandle, needsAnimate);
};

function enterFrame(_dt) {
  // 当 app 准备好时，执行 app 的帧循环
  if (appHandle === 0 || isStoppedRunning) return;

  if (initFinished > 0) {
    enter_frame(appHandle);
  } else {
    // 查询就绪状态
    initFinished = is_preparation_completed(appHandle);
  }
  requestAnimationFrame(enterFrame);
}

function setContainerOpacity(opacity) {
  let ele = document.getElementById("main-thread-container");
  ele.style.opacity = opacity;
}
