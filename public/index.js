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
  if ("navigator" in window && "gpu" in navigator) {
    navigator.gpu
      .requestAdapter()
      .then((_adapter) => {
        resizeCanvas();
        // 浏览器支持 WebGPU
        createAppWindow();
      })
      .catch((_error) => {
        console.log(_error);
        showAlert();
      });
  } else {
    // 浏览器不支持 navigator.gpu
    showAlert();
  }
}
launch();

// 由于 render 装进了 worker 线程，requestAnimationFrame 的频率可能太高，需要有限制策略
let lastTime = performance.now();
let maxFrameRate = 60; // 最多每秒90帧
let frameCount = 0;
function enterFrame() {
  const currentTime = performance.now();
  const elapsedTime = currentTime - lastTime;

  if (elapsedTime >= 1000 / maxFrameRate) {
    // 执行渲染的帧循环
    worker.postMessage({ ty: "enterFrame" });

    lastTime = currentTime;
    frameCount++;
  }
  requestAnimationFrame(enterFrame);
}

function showAlert() {
  alert("请使用 Chrome 或者 Edge 113+ 浏览器版本");
}

function resizeCanvas() {
  let elem = document.getElementById("container");
  let canvas = document.getElementById("app-canvas");
  let ratio = window.devicePixelRatio;
  canvas.width = elem.clientWidth * ratio;
  canvas.height = elem.clientHeight * ratio;
  canvas.style.width = elem.clientWidth + "px";
  canvas.style.height = elem.clientHeight + "px";
  canvas.style.maxWidth = elem.clientWidth + "px";
  canvas.style.maxHeight = elem.clientHeight + "px";
}
