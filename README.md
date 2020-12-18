# wasm_lindera_example
rust + lindera + webassembly + next.js + typescriptで形態素解析するサンプル

![img](https://github.com/vaaaaanquish/wasm_lindera_example/blob/main/img/sample.gif?raw=true)

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

- https://github.com/lindera-morphology/lindera.git
- https://github.com/rustwasm/create-wasm-app/
 et al.
