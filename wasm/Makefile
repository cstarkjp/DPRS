WASM_PACK = wasm-pack build --dev --target web
WASM_PACK = WASM_PACK_WASM_OPT=false wasm-pack build --release --target web
WASM_PACK = WASM_PACK_WASM_OPT=true wasm-pack build --dev --target web
WASM_PACK = wasm-pack build --dev --target web
WASM_PACK = WASM_PACK_WASM_OPT=true wasm-pack build --release --target web

.PHONY: all
all:
	${WASM_PACK}

start_http:
	python3 -m http.server 3003

help:
	echo "If wasm-pack is installed this should build"
	echo "   cargo install wasm-pack"
	echo
	echo "Then make"
	echo
	echo "to run a local server"
	echo
	echo "make start_http"
	echo
	echo "Then open a browser to http:://127.0.0.1:3003/http"


.PHONY: dprs.zip
dprs.zip:
	zip dprs.zip favicon.ico
	zip dprs.zip http/favicon.ico
	zip dprs.zip http/dk.js
	zip dprs.zip http/html.js
	zip dprs.zip http/log.js
	zip dprs.zip http/utils.js
	zip dprs.zip http/storage.js
	zip dprs.zip http/visualize.js
	zip dprs.zip http/simulation.js

	zip dprs.zip http/index.html

	zip dprs.zip pkg/package.json
	zip dprs.zip pkg/dprs_wasm.d.ts
	zip dprs.zip pkg/dprs_wasm.js
	zip dprs.zip pkg/dprs_wasm_bg.wasm
	zip dprs.zip pkg/dprs_wasm_bg.wasm.d.ts
