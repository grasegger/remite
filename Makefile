SHELL = /bin/bash
.PHONY: extension dockershell structure in-docker Dockerfile package.json

extension: packages package.json clean
	touch manifest.yml
	touch icon.svg
	cargo build
	cp node_modules/milligram/dist/milligram.min.css build/lib/milligram.css

package.json:
	npm i

clean:
	cd build && git clean -fxd

packages: clean
	wasm-pack build --target no-modules --no-typescript -d ../../build/background/ --out-name background packages/remite-background
	wasm-pack build --target no-modules --no-typescript -d ../../build/options/ --out-name options packages/remite-options

Dockerfile:
	docker build -t remite .

in-docker: Dockerfile
	docker run --rm -it -v ${shell pwd}:/src remite bash -c ". ~/.cargo/env; make extension"

dockershell:
	docker run --rm -it -v ${shell pwd}:/src remite bash