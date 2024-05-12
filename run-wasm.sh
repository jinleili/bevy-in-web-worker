set -e 

RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --no-default-features \
--target wasm32-unknown-unknown 

# Generate bindings
for i in target/wasm32-unknown-unknown/debug/*.wasm;
do
    # wasm-bindgen --no-typescript --target no-modules --out-dir wasm --web "$i";
    wasm-bindgen --no-typescript --out-dir wasm --target no-modules "$i";
done

cp wasm/bevy_in_web_worker.js public/bevy_in_web_worker.js
cp wasm/bevy_in_web_worker_bg.wasm public/bevy_in_web_worker_bg.wasm

basic-http-server public

