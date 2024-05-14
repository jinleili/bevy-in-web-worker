# Bevy-in-Web-Worker

Engine 放进 Web Worker 后与主线程交互的通信时延与性能验证

**验证点：**

1. 主线程与 Worker 之间事件通信成本是否会很高。
2. 异步的 pick 接口是否会导致用户交互体验变差。
3. 主线程阻塞对 事件/ worker 的影响。

**针对上面验证点的设计：**

1. 不对鼠标事件触发频率做任何 **Throttle**, 每一个 mousemove 事件都会发送给 worker, 由 worker 执行一次 ray pick 后将结果发回给主线程;
2. 设计了高交互复杂度 **选中/高亮** 逻辑：
    1. 主线程 postMsg 给 worker -> 
    2. worker 转交任务给 engine 执行 ray pick -> 
    3. 将结果从 engine 发给 worker -> 
    4. 由 worker postMsg 给主线程 -> 
    5. 主线程再 postMsg 给 worker 执行需要的 **选中/高亮**
3. 模拟了主线程阻塞的场景，可以控制单帧阻塞时长，同时左上角有一个**主线程卡顿指示器**便于观察阻塞结果

## 代码运行

```sh
# 编译 WASM：
# 添加 Rust WebAssembly target
rustup target add wasm32-unknown-unknown
# 安装 wasm-bindgen 命令行工具
cargo install -f wasm-bindgen-cli --version 0.2.92

# 运行：
# 先安装 http server
cargo install basic-http-server
# 然后使用下边的脚本运行
sh ./run-wasm.sh

# 构建 release 包
sh ./build-wasm.sh
```
