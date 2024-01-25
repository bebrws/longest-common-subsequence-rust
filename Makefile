all: longest_common_subsequence_rustjs

longest_common_subsequence_rustjs: pkg/longest_common_subsequence_rust.js
	wasm-pack build --target web
