# wasm_lindera_example
rust + webassembly + next.js + typescriptで形態素解析するサンプル

# Usage

- 初回やる
```
## init
npm install
```

- wasmのbuild
```
wasm-pack build src/tokenize-text
```

- localhost:3000にアプリが立つやつ
```
npm run build
npm run start
```

# Thanks

- https://github.com/rustwasm/create-wasm-app/

et al.
