set -e 

# worker 中运行时，debug 模式下会有前几帧须要拉长帧时间间隔的问题
# https://github.com/bevyengine/bevy/issues/13345
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' \
cargo build --no-default-features --profile dev-opt --target wasm32-unknown-unknown

# Generate bindings
for i in target/wasm32-unknown-unknown/dev-opt/*.wasm;
do
    wasm-bindgen --no-typescript --out-dir wasm --web "$i";
    # Worker 中加载的脚本无法使用 ES6 module
    wasm-bindgen --no-typescript --out-dir wasm-no-modules --target no-modules "$i";
done

cp wasm/bevy_in_web_worker.js public/bevy_in_main_thread.js
cp wasm-no-modules/bevy_in_web_worker.js public/bevy_in_web_worker.js
# 两份 js 共用同一个 wasm
cp wasm/bevy_in_web_worker_bg.wasm public/bevy_in_web_worker_bg.wasm

ADDR=${BASIC_HTTP_ADDR:-127.0.0.1:4000}
basic-http-server -a "$ADDR" public
