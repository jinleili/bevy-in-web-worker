import init, {
  init_bevy_app,
  is_preparation_completed,
  create_window_by_canvas,
  enter_frame,
  mouse_move,
  set_hover,
  set_selection,
} from "./bevy_in_main_thread.js";

let appHandle = 0;
let initFinished = 0;

async function launchMainApp() {
  await init();
  appHandle = init_bevy_app();

  // 创建渲染窗口
  let devicePixelRatio = window.devicePixelRatio;
  create_window_by_canvas(appHandle, "main-thread-canvas", devicePixelRatio);

  // 开始动画
  requestAnimationFrame(enterFrame);
}
launchMainApp();

window.mouse_move = (x, y) => {
  if (initFinished > 0) mouse_move(appHandle, x, y);
};

window.set_hover = (list) => {
  if (initFinished > 0) set_hover(appHandle, list);
};

window.set_selection = (list) => {
  if (initFinished > 0) set_selection(appHandle, list);
};

function enterFrame(_dt) {
  // 当 app 准备好时，执行 app 的帧循环
  if (initFinished > 0) {
    enter_frame(appHandle);
  } else {
    // 查询就绪状态
    initFinished = is_preparation_completed(appHandle);
  }
  requestAnimationFrame(enterFrame);
}
