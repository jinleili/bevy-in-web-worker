import init, {
  init_bevy_app,
  finish_init,
  enter_frame,
} from "./bevy_in_web_worker.js";

let appHandle = 0;
let initFinished = 0;
async function initApp() {
  await init();
  appHandle = init_bevy_app();
  initFinished = finish_init(appHandle);
  // 开始动画
  requestAnimationFrame(enterFrame);
}

function launch() {
  window.onresize = window_resized;
  window.dispatch_resize_event = dispatch_resize_event;
  window.canvas_resize_completed = canvas_resize_completed;

  if ("navigator" in window && "gpu" in navigator) {
    navigator.gpu
      .requestAdapter()
      .then((adapter) => {
        // 浏览器支持 WebGPU
        initApp();
      })
      .catch((error) => {
        showAlert();
      });
  } else {
    // 浏览器不支持 navigator.gpu
    showAlert();
  }
}

function window_resized() {
  clearTimeout(this.timeOutFunctionId);
  if (this.can_resize_canvas || this.missedResizeCount > 10) {
    this.missedResizeCount = 0;
    this.timeOutFunctionId = setTimeout(this.dispatch_resize_event, 100);
  } else {
    // 等待 rust 端 resize 完成
    this.missedResizeCount++;
    this.timeOutFunctionId = setTimeout(this.window_resized, 100);
  }
}

function showAlert() {
  alert("请使用 Chrome 或者 Edge 113+ 浏览器版本");
}

let can_resize_canvas = false;
function canvas_resize_completed() {
  can_resize_canvas = true;
}

function dispatch_resize_event() {
  can_resize_canvas = false;
  let elem = document.getElementById("container");
  let canvas = elem.getElementsByTagName("canvas")[0];
  if (canvas != null) {
    let ratio = window.devicePixelRatio;
    canvas.width = elem.clientWidth * ratio;
    canvas.height = elem.clientHeight * ratio;
    canvas.style.width = elem.clientWidth + "px";
    canvas.style.height = elem.clientHeight + "px";
    canvas.style.maxWidth = elem.clientWidth + "px";
    canvas.style.maxHeight = elem.clientHeight + "px";
  }
}

function enterFrame() {
  // 当 app 准备好时，执行 app 的帧循环
  if (initFinished > 0) {
    enter_frame(appHandle);
  } else {
    initFinished = finish_init(appHandle);
  }
  requestAnimationFrame(enterFrame);
}

launch();
