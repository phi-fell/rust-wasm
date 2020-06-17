wasm-pack build --target web
cp -a site/. build
cp pkg/gotg_client.js build/
cp pkg/gotg_client_bg.wasm build/
