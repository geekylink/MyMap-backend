all:
	cargo build
	cd www && yarn install && yarn build

clean:
	cargo clean
	rm -rf build/*
	cd www && rm -rf node_modules && rm -rf build/*

# Docker builds
docker-build:
	docker build -t rusty-build-srv -f 01-Build.Dockerfile .
