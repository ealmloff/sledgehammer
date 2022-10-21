# sledgehammer

**Breaking the WASM<->JS peformance boundry one brick at a time**
### Status: There are cracks in the wall.

# What is Sledgehammer?
Sledgehammer provides faster rust bindings for dom manipuations by batching calls to js. On the js-framework benchmarks it results in about half of the overhead of wasm-bindgen.

# A few runs of the js-framework-benchmark (seems to be quite a bit of variation)
![image](https://user-images.githubusercontent.com/66571940/197082775-e720b258-0691-47e3-acdc-d5c15c7cceab.png)
![image](https://user-images.githubusercontent.com/66571940/197093432-0df1aa04-ef3b-40f2-b829-fedca9f307ea.png)
![image](https://user-images.githubusercontent.com/66571940/197096143-ed517c1e-a526-491b-9595-b0c629943ed1.png)

# How does this compare to wasm-bindgen/web-sys:
wasm-bindgen is a lot more general, and ergonomic to use than sledgehammer. It has bindings to a lot of apis that sledgehammer does not. For most users wasm-bindgen is a beter choice. Sledgehammer is specifically designed for web frameworks that want low level, fast access to the dom.
