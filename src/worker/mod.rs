use std::{fmt::Debug, ops::Deref};
use wasm_bindgen::prelude::*;

/// Binds JS.
#[wasm_bindgen(module = "/src/worker/workerGen.js")]
extern "C" {
    /// Spawn new worker in JS side in order to make bundler know about dependency.
    #[wasm_bindgen(js_name = "createWorker")]
    fn create_worker(kind: &str, name: &str) -> web_sys::Worker;
}

/// Binds JS.
/// This makes wasm-bindgen bring `mainWorker.js` to the `pkg` directory.
/// So that bundler can bundle it together.
#[wasm_bindgen(module = "/src/worker/mainWorker.js")]
extern "C" {
    /// Nothing to do.
    #[wasm_bindgen]
    fn attachMain();
}

pub struct MainWorker {
    handle: web_sys::Worker,
    name: String,
    _callback: Closure<dyn FnMut(web_sys::Event)>,
}

impl Drop for MainWorker {
    /// Terminates web worker *immediately*.
    fn drop(&mut self) {
        self.handle.terminate();
        crate::log!("Worker({}) was terminated", &self.name);
    }
}

impl Debug for MainWorker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MainWorker")
            .field("handle", &self.handle)
            .field("name", &self.name)
            .finish()
    }
}

impl MainWorker {
    /// Spawns main worker from the window context.
    pub fn spawn(name: &str, id: usize) -> Result<Self, JsValue> {
        // Creates a new worker.
        let handle = create_worker("main", name);

        // Sets default callback.
        let callback = Closure::new(|_ev| {
            // Implement if you want.
            unimplemented!()
        });
        handle.set_onmessage(Some(callback.as_ref().unchecked_ref()));

        // Initializes the worker.
        let msg = js_sys::Array::new_with_length(2);
        msg.set(0, wasm_bindgen::module());
        msg.set(1, id.into());
        handle.post_message(&msg)?;

        Ok(Self {
            handle,
            name: name.to_owned(),
            _callback: callback,
        })
    }
}

impl Deref for MainWorker {
    type Target = web_sys::Worker;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}
