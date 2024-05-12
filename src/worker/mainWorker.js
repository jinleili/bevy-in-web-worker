
import * as wasm from '../../../..';

export function attachMain() {}

onmessage = event => {
  // Initailzes wasm.
  const { default: wbg_init } = wasm;
  wbg_init(event.data[0]).then(() => {

    // Initializes our main worker.
    onmessage = async (event) => {
      const ready = await wasm.main_onmessage_init(event);
      if (ready) {

        // Now, worker is ready for work.
        onmessage = wasm.main_onmessage;
      }
    }
  });
}

