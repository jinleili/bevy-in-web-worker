const worker = new Worker("./worker.js");

// worker 是否准备就绪
let workerIsReady = false;

// 监听 worker 发来的消息
worker.onmessage = async (event) => {
  let data = event.data;
  switch (data.ty) {
    case "workerIsReady":
      workerIsReady = true;
      break;
    default:
      break;
  }
};

// 创建 app 窗口
function createAppWindow() {
  delayExecute(() => {
    if (workerIsReady) {
      // 创建渲染窗口
      let canvas = document.getElementById("app-canvas");
      let offscreenCanvas = canvas.transferControlToOffscreen();
      let devicePixelRatio = window.devicePixelRatio;
      worker.postMessage(
        { ty: "init", canvas: offscreenCanvas, devicePixelRatio },
        [offscreenCanvas]
      );

      enterFrame();

      return true;
    }
    return false;
  });
}

function delayExecute(fn, delay = 50) {
  function execute() {
    if (fn()) {
      clearInterval(timer);
    }
  }
  const timer = setInterval(execute, delay);
}

function launch() {
  window.onresize = window_resized;
  window.dispatch_resize_event = dispatch_resize_event;
  window.canvas_resize_completed = canvas_resize_completed;

  if ("navigator" in window && "gpu" in navigator) {
    navigator.gpu
      .requestAdapter()
      .then((_adapter) => {
        // 浏览器支持 WebGPU
        createAppWindow();
      })
      .catch((_error) => {
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

let lastTime = performance.now();
let maxFrameRate = 90; // 最多每秒90帧

function enterFrame() {
  const currentTime = performance.now();
  const elapsedTime = currentTime - lastTime;

  if (elapsedTime >= 1000 / maxFrameRate) {
    // 执行渲染的帧循环
    worker.postMessage({ ty: "enterFrame" });

    lastTime = currentTime;
  }
  requestAnimationFrame(enterFrame);
}

launch();
