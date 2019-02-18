const index = import('./index')
index.then(() => {
  console.log('WebAssembly module loaded')
})