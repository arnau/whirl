# Whirl

This is an experimental library to support the [Seachess] build process.
It uses [napi-rs] to interface between Node.js and Rust.

![https://github.com/arnau/whirl/actions](https://github.com/arnau/whirl/workflows/CI/badge.svg)


## Build

```
yarn build
```

The result is a `index.[darwin|win32|linux].node` file in project root. This is the native
addon built from [lib.rs](./src/lib.rs).


## Test

TODO


## Release

Github Actions prebuild the binaries for each platform, then each platform is
released as an independent NPM package. A generic package is also released to
serve as a dispatcher for the user platform.

1. Ensure you have set a **NPM_TOKEN** in `Settings/Secrets`.
2. Bump the version `yarn version xxx`
3. Push with tags `git push --follow-tags`
4. (automatic) Github Actions build binaries.


## Licence

The codebase is licensed under the [MIT licence](./LICENCE).



[Seachess]: https://github.com/arnau/seachess
[napi-rs]: https://napi.rs/
