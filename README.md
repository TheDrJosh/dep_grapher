# Dep Grapher

dependancy tree viewer

## Ideas

- support for local and remote repositorys (git and package registrys)
- support for rust (cargo), js (npm and jsr), python (PyPl), zig
- suport for local / git package files
  - cargo.toml (rust)
  - package.json (javascript)
  - deno.json / deno.jsonc (javascript)
  - build.zig.zon (zig)
  - requirements.txt (python)
  - Pipfile (python)

## Todo

- [ ] options page
  - [ ] dark / light / system
  - [ ] default registry api urls
- [ ] support opening local projects
  - [x] rust
  - [ ] node
  - [ ] deno
  - [x] python
  - [x] zig
- [ ] support opening git projects
  - [ ] local
  - [ ] remote
- [ ] suport opening from registries
  - [ ] crate.io
  - [ ] npm
  - [ ] jsr
  - [ ] pypi
- [ ] graph view

## Info

crate.io registry index: [[https://github.com/rust-lang/crates.io-index]]
npm registry api: [[https://registry.npmjs.org/]]
