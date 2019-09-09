


build-deb:
	docker run -ti --rm -v $(PWD):/app -w /app nodester-builder bash cargo build