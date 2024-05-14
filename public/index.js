/**
 * 为避免浏览器同时下载同一个 Wasm 文件，会先执行 worker 加载，
 * worker 内 engine 实例创建成功后，再利用浏览器缓存加载主线程的 engine 实例。
 */

const worker = new Worker("./worker.js");

// worker 是否准备就绪
let workerIsReady = false;

// 最新的 pick 结果
let latestPick = [];

// 监听 worker 发来的消息
worker.onmessage = async (event) => {
  let data = event.data;

  switch (data.ty) {
    case "workerIsReady":
      workerIsReady = true;
      // 开始加载主线程的实例
      loadMainThreadEngine();
      addMouseEventObserver();
      break;

    case "pick":
      // 依赖 pick 的其他业务逻辑
      // ...

      // 在网页上显示 pick 结果
      let ele = document.getElementById("pick-list");
      ele.innerText = data.list;

      latestPick = data.list;
      // 通知 worker 哪些 entity 启用 hover 效果
      worker.postMessage({ ty: "hover", list: latestPick });
      break;

    default:
      break;
  }
};

// 创建 app 窗口
function createWorkerAppWindow() {
  delayExecute(() => {
    if (workerIsReady) {
      let loading = document.getElementById("loading");
      loading.style.display = "none";

      // 创建渲染窗口
      let canvas = document.getElementById("worker-thread-canvas");
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

// 启动 App
function launch() {
  // 先检测浏览器环境
  if ("navigator" in window && "gpu" in navigator) {
    navigator.gpu
      .requestAdapter()
      .then((_adapter) => {
        // 调整画布样式
        resizeCanvasBy("main-thread-container");
        resizeCanvasBy("worker-thread-container");

        // 浏览器支持 WebGPU
        createWorkerAppWindow();
      })
      .catch((_error) => {
        console.error(_error);
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

// 基于父容器设置 canvas
function resizeCanvasBy(containerID) {
  let elem = document.getElementById(containerID);
  let canvas = elem.children[0];
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
  let workerContainer = document.getElementById("worker-thread-container");
  workerContainer.addEventListener("mousemove", function (event) {
    // 在将 mouse move 事件发送给 worker 之前，清空上次的 pick 缓存
    latestPick = [];

    worker.postMessage({ ty: "mousemove", x: event.offsetX, y: event.offsetY });
  });

  workerContainer.addEventListener("click", function (event) {
    if (Array.isArray(latestPick) && latestPick.length > 0) {
      worker.postMessage({
        ty: "select",
        list: latestPick,
      });
    }
  });

  let mainContainer = document.getElementById("main-thread-container");
  mainContainer.addEventListener("mousemove", function (event) {
    // 清空上次的 pick 缓存
    latestPick = [];
    window.mouse_move(event.offsetX, event.offsetY);
  });

  mainContainer.addEventListener("click", function (_event) {
    window.set_selection(latestPick);
  });
}

// 装载主线程的 engine 实例
function loadMainThreadEngine() {
  var script = document.createElement("script");
  script.type = "module";
  script.src = "./main_thread_engine_instance.js";
  document.body.appendChild(script);
}

/** 主线程的 engine 实例发来的 ray pick 结果 */
function send_pick_from_rust(pickList) {
  // 在网页上显示 pick 结果
  let ele = document.getElementById("pick-list");
  ele.innerText = pickList;
  latestPick = pickList;

  window.set_hover(latestPick);
}
window.send_pick_from_rust = send_pick_from_rust;
