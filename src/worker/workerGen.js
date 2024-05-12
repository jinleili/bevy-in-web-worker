export function createWorker(kind, name) {
  switch (kind) {
    case 'main':
      const main = new Worker(new URL('./mainWorker.js', import.meta.url), {
        type: 'module',
        /* @vite-ignore */ name, // vite doesn't allow non static value here.
      });
      return main;
    default:
      console.log("unsurpported type of worker: ", kind);
      return undefined;
  }
}
