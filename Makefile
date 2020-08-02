.PHONY: all dockerfile packages parts extension

extension: manifest packages dockericons
	cp src/background_loader.js build/background/loader.js

dockerfile:
	docker build -t remite .

dockershell:
	docker run --rm -it -v ${shell pwd}:/src remite bash

dependencies: dockerfile
	docker run --rm -it -v ${shell pwd}:/src remite bash -c 'npm i'

structure: dockerfile
	mkdir -p build/icons
	rm -rf build/background

manifest: structure
	docker run --rm -it -v ${shell pwd}:/src -v remite:/tmp remite bash -c 'catmandu convert YAML < src/manifest.yaml | jq .[0] > build/manifest.json'

packages: dockerfile
	docker run --rm -it -v ${shell pwd}:/src remite bash -c 'source ~/.cargo/env && cd packages/remite-background && wasm-pack build --target no-modules --no-typescript  --release -d "../../build/background/" --out-name background'

icons:
	convert src/icons/feather_clock.svg -alpha set -channel RGBA -fuzz 40% -fill none -floodfill +0+0 white -shave 1x1 -trim +repage -resize 48x48 build/icons/remite-48.png
	convert src/icons/feather_clock.svg -alpha set -channel RGBA -fuzz 40% -fill none -floodfill +0+0 white -shave 1x1 -trim +repage -resize 96x96 build/icons/remite-96.png
	convert src/icons/feather_clock.svg -alpha set -channel RGBA -fuzz 40% -fill none -floodfill +0+0 white -shave 1x1 -trim +repage -resize 32x32 build/icons/remite-32.png
	convert build/icons/remite-32.png -channel RGB -negate build/icons/remite-32-light.png

dockericons: structure dockerfile
	docker run --rm -it -v ${shell pwd}:/src remite bash -c 'make icons'
