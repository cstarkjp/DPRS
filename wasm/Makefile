.PHONY: all start_http serve help zip dprs.zip transfer

# WASM_PACK = wasm-pack build --dev --target web
# WASM_PACK = WASM_PACK_WASM_OPT=false wasm-pack build --release --target web
# WASM_PACK = WASM_PACK_WASM_OPT=true wasm-pack build --dev --target web
# WASM_PACK = wasm-pack build --dev --target web
WASM_PACK = WASM_PACK_WASM_OPT=true wasm-pack build --release --target web

all:
	${WASM_PACK}

start_http:
	python3 -m http.server 3003

serve:
	@echo "Starting a local web server: point your browser to http:://127.0.0.1:3003/http"
	python -m http.server 3003

help:
	@echo "If wasm-pack is installed this should build:"
	@echo "   cargo install wasm-pack"
	@echo "Then:"
	@echo "   make"
	@echo
	@echo "To run a local server:"
	@echo "    make serve"
	@echo "Then open a browser to http:://127.0.0.1:3003/http"

zip: dprs.zip
	@echo "Package zipped"

dprs.zip:
	@echo "Zipping package"
	zip dprs.zip favicon.ico
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

transfer:
	@echo "Copying http/ and pkg/ to ../docs/live-demo"
	cp -R http ../documentation/docs/live-demo
	cp -R pkg ../documentation/docs/live-demo
