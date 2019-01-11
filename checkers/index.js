fetch('./checkers.wasm')
  .then((response) => response.arrayBuffer())
  .then((bytes) => WebAssembly.instantiate(bytes, {
    events: {
      piecemoved (fX, fY, tX, tY) {
        console.log('moved', { fX, fY, tX, tY })
      },
      piececrowned (x, y) {
        console.log('crowned at', x, y)
      },
      errormove (x, y, x2, y2) {
        console.error('Error moving', { x, y, x2, y2 })
      }
    }
  })).then((results) => {
    const { instance } = results
    instance.exports.initBoard()
    console.log('owner', instance.exports.getTurnOwner())

    instance.exports.move(0, 6, 0, 5) // B
    instance.exports.move(1, 0, 1, 1) // W
    instance.exports.move(0, 5, 0, 4) // B
    instance.exports.move(1, 1, 1, 0) // W
    instance.exports.move(0, 4, 0, 3) // B
    instance.exports.move(1, 0, 1, 1) // W
    instance.exports.move(0, 3, 0, 2) // B
    instance.exports.move(1, 1, 1, 0) // W
    instance.exports.move(0, 2, 0, 0) // B - this will get a crown
    instance.exports.move(1, 0, 1, 1) // W
    instance.exports.move(0, 0, 0, 2) // B move the crown piece out
    console.log('at end, owner is', instance.exports.getTurnOwner())
  }).catch(console.error)