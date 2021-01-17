const { loadBinding } = require('@node-rs/helper')

/**
 * First param: `__dirname` means load native addon from the current dir.
 * Second param: The name of the nativ emodule.
 * Third param: The name of the package.
 *
 * The `loadBinding` helper loads `whirl.[PLATFORM].node` from `__dirname`.
 * If it fails to do so, it falls back to load from `@arnau/whirl-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'whirl', '@arnau/whirl')
