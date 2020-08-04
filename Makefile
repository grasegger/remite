SHELL = /bin/bash
.PHONY: extension dockershell structure in-docker

extension: packages package.json clean
	cargo build
	cp node_modules/milligram/dist/milligram.min.css build/lib/milligram.css

in-docker: Dockerfile
	docker run --rm -it -v ${shell pwd}:/src remite make extension

Dockerfile:
	docker build -t remite .

dockershell:
	docker run --rm -it -v ${shell pwd}:/src remite bash

package.json:
	npm i

clean:
	cd build && git clean -fxd

packages: clean
	wasm-pack build --target no-modules --no-typescript  --release -d build/background/ --out-name background packages/remite-background
	wasm-pack build --target no-modules --no-typescript  --release -d build/options/ --out-name options packages/remite-options