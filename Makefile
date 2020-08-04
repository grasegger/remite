SHELL = /bin/bash
.PHONY: extension dockershell structure in-docker

extension: packages icons package.json
	cargo build
	cp node_modules/milligram/dist/milligram.min.css build/lib/milligram.css

in-docker:
	docker run --rm -it -v ${shell pwd}:/src remite make extension

Dockerfile:
	docker build -t remite .

dockershell:
	docker run --rm -it -v ${shell pwd}:/src remite bash

package.json:
	npm i

structure:
	mkdir -p build/lib
# tood use git delete 


packages: structure
	source ~/.cargo/env && cd packages/remite-background && wasm-pack build --target no-modules --no-typescript  --release -d "../../build/background/" --out-name background
	source ~/.cargo/env && cd packages/remite-options && wasm-pack build --target no-modules --no-typescript  --release -d "../../build/options/" --out-name options

icons: structure
	convert icon.svg -alpha set -channel RGBA -fuzz 40% -fill none -floodfill +0+0 white -shave 1x1 -trim +repage -resize 48x48 build/icons/remite-48.png
	convert icon.svg -alpha set -channel RGBA -fuzz 40% -fill none -floodfill +0+0 white -shave 1x1 -trim +repage -resize 96x96 build/icons/remite-96.png
	convert icon.svg -alpha set -channel RGBA -fuzz 40% -fill none -floodfill +0+0 white -shave 1x1 -trim +repage -resize 32x32 build/icons/remite-32.png
	convert build/icons/remite-32.png -channel RGB -negate build/icons/remite-32-light.png
