# Dep Grapher

dependancy tree viewer

## Todo

- [ ] options page
  - [ ] dark / light / system
  - [ ] default registry api urls
- [ ] support opening local projects
  - [ ] rust (Cargo.toml)
  - [ ] node (package.json)
  - [ ] deno (deno.json/jsonc)
  - [ ] python (requirement.txt, PipFile, pyproject.toml)
  - [ ] zig (build.zig.zon)
- [ ] support opening git projects
  - [ ] local
  - [ ] remote
  - [ ] submodules
- [ ] suport opening from registries
  - [ ] crate.io
  - [ ] npm
  - [ ] jsr
  - [ ] pypi
- [ ] graph view
  - use <https://reactflow.dev/>
  - use <https://d3js.org/d3-force>
  - [ ] save file
  - [ ] load file .dgf
  - [ ] handle dependencies better
    - [ ] platform specific
    - [ ] features
    - [ ] dev / build

## Info

crate.io registry api: <https://crates.io> <https://doc.rust-lang.org/cargo/reference/registry-web-api.html>

npm registry api: <https://registry.npmjs.org/> <https://github.com/npm/registry/blob/main/docs/REGISTRY-API.md>

jsr registry api: <https://api.jsr.io> <https://jsr.io/docs/api-reference#tag/default/GET/packages>

pypl registry api: <https://pypi.org/simple/> <https://packaging.python.org/en/latest/specifications/simple-repository-api/>

## Maybe

- add cmake, meson, Bazel, Premake, and MSBuild support for c/c++ projects
- add test (js and rust) and add to ci
- <https://github.com/tauri-apps/tauri-action>
