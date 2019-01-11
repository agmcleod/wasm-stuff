fetch('./func_test.wasm')
  .then((response) => response.arrayBuffer())
  .then((bytes) => WebAssembly.instantiate(bytes))
  .then((results) => {
    console.log(results.instance);
    const { instance } = results;

    const white = 2;
    const black = 1;
    const crownedWhite = 6;
    const crownedBlack = 5;

    const offset = instance.exports.offsetForPosition(3, 4);
    console.log(offset);

    console.debug('White is white', instance.exports.isWhite(white));
    console.debug('Black is black', instance.exports.isBlack(black));
    console.debug('Black is not white', instance.exports.isWhite(black));
    console.debug('Uncrowned white', instance.exports.isWhite(instance.exports.withoutCrown(crownedWhite)));
    console.debug('Uncrowned black', instance.exports.isBlack(instance.exports.withoutCrown(crownedBlack)));

    console.debug('Crowned black', instance.exports.isCrowned(crownedBlack));
    console.debug('Crowned white', instance.exports.isCrowned(crownedWhite));
})