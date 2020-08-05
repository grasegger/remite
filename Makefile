SHELL = /bin/bash
.PHONY: extension dockershell structure in-docker Dockerfile 

debug: clean
	cargo build -vv
 
release: clean
	cargo build -v --release

clean:
	cargo clean -p remite
	cd build && git clean -fxd

Dockerfile:
	docker build -t remite .

debug-in-docker: Dockerfile
	docker run --rm -it -v ${shell pwd}:/src remite bash -c ". ~/.cargo/env; make debug"

release-in-docker: Dockerfile
	docker run --rm -it -v ${shell pwd}:/src remite bash -c ". ~/.cargo/env; make release"