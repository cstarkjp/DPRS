cd DPRS

uv venv --python=3.14
source .venv/bin/activate
uv pip install mkdocstrings mkdocs-material mkdocs-caption mkdocstrings-python black ruff

cd dprs_web
cargo install wasm-pack
npm install typescript
make
make js
make transfer

cd ../web
# mkdocs build
mkdocs serve