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
function createAppWindow() {
  delayExecute(() => {
    if (workerIsReady) {
      let loading = document.getElementById("loading");
      loading.style.display = "none";

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
    // 在将 mouse move 事件发送给 worker 之前，清空上次的 pick 缓存
    latestPick = [];

    worker.postMessage({ ty: "mousemove", x: event.offsetX, y: event.offsetY });
  });

  container.addEventListener("click", function (event) {
    if (Array.isArray(latestPick) && latestPick.length > 0) {
      worker.postMessage({
        ty: "select",
        list: latestPick,
      });
    }
  });
}
