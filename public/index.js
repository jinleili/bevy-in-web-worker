const worker = new Worker("./worker.js");

// worker 是否准备就绪
let workerIsReady = false;

// 监听 worker 发来的消息
worker.onmessage = async (event) => {
  let data = event.data;
  switch (data.ty) {
    case "workerIsReady":
      workerIsReady = true;
      addMouseEventObserver();
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

// 添加鼠标事件监听
function addMouseEventObserver() {
  let container = document.getElementById("container");
  container.addEventListener("mousemove", function (event) {
    worker.postMessage({ ty: "mousemove", x: event.offsetX, y: event.offsetY });
  });
}
