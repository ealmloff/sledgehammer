<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/sledgehammer">
    <img src="https://img.shields.io/crates/v/sledgehammer.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/sledgehammer">
    <img src="https://img.shields.io/crates/d/sledgehammer.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs -->
  <a href="https://docs.rs/sledgehammer">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

# sledgehammer

**Breaking the WASM<->JS peformance boundry one brick at a time**
### Status: There are cracks in the wall.

# What is Sledgehammer?
Sledgehammer provides faster rust bindings for dom manipuations by batching calls to js.

# Benchmarks

- js-framework-benchmark that trust the implementation and only measures operation time (not paint time):
https://demonthos.github.io/wasm_bindgen_sledgehammer/
This gives more consistant results than the official js-framework-benchmark because it excludes the variation in paint time. Because sledgehammer and wasm-bindgen implementations result in the same dom calls they should have the same paint time.

- A few runs of the js-framework-benchmark (seems to be quite a bit of variation)
![image](https://user-images.githubusercontent.com/66571940/197082775-e720b258-0691-47e3-acdc-d5c15c7cceab.png)
![image](https://user-images.githubusercontent.com/66571940/197093432-0df1aa04-ef3b-40f2-b829-fedca9f307ea.png)
![image](https://user-images.githubusercontent.com/66571940/197096143-ed517c1e-a526-491b-9595-b0c629943ed1.png)

# How does this compare to wasm-bindgen/web-sys:
wasm-bindgen is a lot more general, and ergonomic to use than sledgehammer. It has bindings to a lot of apis that sledgehammer does not. For most users wasm-bindgen is a beter choice. Sledgehammer is specifically designed for web frameworks that want low level, fast access to the dom.
