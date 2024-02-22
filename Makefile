all: build

build:
	wasm-pack build --target web

build-debug:
	wasm-pack build --target web --dev

web-server:
	python3 -m http.server 7878 -d .

test:
	wasm-pack test --headless --firefox

publish:
	wasm-pack publish

clean:
	rm -rf ./pkg
	cargo clean
