wasm-pack build --target web
cp -a site build
cp pkg/graemarch.js build/
cp pkg/graemarch_bg.wasm build/
