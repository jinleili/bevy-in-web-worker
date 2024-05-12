set -e 

RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --no-default-features --profile wasm-release \
--target wasm32-unknown-unknown 

# Generate bindings
for i in target/wasm32-unknown-unknown/wasm-release/*.wasm;
do
    # wasm-bindgen --no-typescript --out-dir wasm --web "$i";
    wasm-bindgen --no-typescript --out-dir wasm --target no-modules "$i";
done

# 优化 wasm 包大小
wasm-opt -Oz --output public/bevy_in_web_worker_bg.wasm wasm/bevy_in_web_worker_bg.wasm
cp wasm/bevy_in_web_worker.js public/bevy_in_web_worker.js



