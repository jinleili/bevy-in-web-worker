# Bevy in Web Worker

展示如何在 Web Worker 中运行 Bevy 引擎，以及 HTML 元素与 Bevy 之间的交互。

实现了 `CanvasViewPlugin` 来替代 `bevy_winit`，使 Bevy 引擎能够脱离主线程，在 Web Worker 中高效运行，从而显著提升了应用的性能和响应速度。同时，还演示了 HTML 元素和 Bevy 引擎之间的双向通信，实现了复杂的交互模式来验证 Engine 放进 Web Worker 后与主线程交互的通信时延与性能。

**验证点：**

1. 主线程与 Worker 之间事件通信成本是否会很高。
2. 异步的 pick 接口是否会导致用户交互体验变差。
3. 主线程阻塞对 事件/ worker 的影响。
4. 鼠标事件，onmessage 及 render 内阻塞对体验上的影响。

**针对上面验证点的设计：**

1. 不对鼠标事件触发频率做任何  **Throttle**, 每一个 mousemove 事件都会发送给 worker, 由 worker 执行一次 ray pick 后将结果发回给主线程;
2. 设计了高交互复杂度  **选中/高亮**  逻辑：
   1. 主线程 postMsg 给 worker ->
   2. worker 转交任务给 engine  执行 ray pick ->
   3. 将结果从 engine 发给 worker ->
   4. 由 worker postMsg 给主线程 ->
   5. 主线程再 postMsg 给 worker 执行需要的  **选中/高亮**
3. 模拟了主线程阻塞的场景，可以控制单帧阻塞时长，同时左上角有一个**主线程卡顿指示器**便于观察阻塞结果;
4. 界面中同时提供了 主线程与 Worker 线程 两个运行实例，方便直观对比;
5. 模拟了 mousemove, onmessage 及 render 内阻塞的场景;
6. 提供了按住鼠标左键拖动场景对象的功能;
7. 模拟了工具类 App 的帧渲染逻辑：关闭场景动画后，帧渲染就完全由鼠标事件驱动;

![Bevy in Web Worker](./screenshot.png)

## 代码运行

```sh
# 编译 WASM：
# 添加 Rust WebAssembly target
rustup target add wasm32-unknown-unknown
# 安装 wasm-bindgen 命令行工具
cargo install -f wasm-bindgen-cli --version 0.2.99

# 运行：
# 先安装 http server
cargo install basic-http-server
# 然后使用下边的脚本运行
sh ./run-wasm.sh

# 构建 release 包
sh ./build-wasm.sh
```
