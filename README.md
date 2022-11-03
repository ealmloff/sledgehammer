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
### Status: There are some holes in the wall.

# What is Sledgehammer?
Sledgehammer provides faster rust bindings for dom manipuations by batching calls to js.

# Benchmarks

- js-framework-benchmark that trust the implementation and only measures dom operation time (not paint time):
https://demonthos.github.io/wasm_bindgen_sledgehammer/
This gives more consistant results than the official js-framework-benchmark because it excludes the variation in paint time. Because sledgehammer and wasm-bindgen implementations result in the same dom calls they should have the same paint time.

- A few runs of the js-framework-benchmark (seems to be quite a bit of variation)
![image](https://user-images.githubusercontent.com/66571940/197082775-e720b258-0691-47e3-acdc-d5c15c7cceab.png)
![image](https://user-images.githubusercontent.com/66571940/197093432-0df1aa04-ef3b-40f2-b829-fedca9f307ea.png)
![image](https://user-images.githubusercontent.com/66571940/197096143-ed517c1e-a526-491b-9595-b0c629943ed1.png)

# How does this compare to wasm-bindgen/web-sys:
wasm-bindgen is a lot more general, and ergonomic to use than sledgehammer. It has bindings to a lot of apis that sledgehammer does not. For most users wasm-bindgen is a beter choice. Sledgehammer is specifically designed for web frameworks that want low level, fast access to the dom.

# Why is it fast

## String decoding

- Decoding strings are expensive to decode, but the cost doesn't change much with the size of the string. Wasm-bindgen calls TextDecoder.decode for every string. Sledehammer only calls TextEncoder.decode once per batch.

- If the string is small it is faster to decode the string in javascript to avoid the constant overhead of TextDecoder.decode

- See this benchmark: https://jsbench.me/4vl97c05lb/5

## Byte encoded operations

- In sledehammer every operation is encoded as a sequence of bytes packed into an array. Every operation takes 1 byte plus whatever data is required for it.

- Booleans are encoded as part of the operation byte to reduce the number of bytes read.

- Each operation is encoded in a batch of four as a u32. Getting a number from an array buffer has a high constant cost, but getting a u32 instead of a u8 is not more expensive. Sledgehammer reads the u32 and then splits it into the 4 individual bytes.

- See this benchmark: https://jsbench.me/csl9lfauwi/2

## Minimize passing ids

- A common set of operations for webframeworks to perform is traversing dom nodes after cloning them. Instead of assigning an id to every node, sledgehammer allows you to perform operations on the last node that was created or navigated to. This means traversing id takes only one byte per operation instead of 5.
