# Makefile for Bevy WASM Web Deployment

# Variables
APP_NAME := pomodoro
TARGET := wasm32-unknown-unknown
RELEASE_DIR := target/$(TARGET)/release
DIST_DIR := docs
ASSETS_DIR := assets
WASM_BINDGEN := wasm-bindgen

.PHONY: all build bindgen copy-assets copy-html dist clean serve

all: dist

build:
	cargo build --release --target $(TARGET)

bindgen: build
	$(WASM_BINDGEN) --target web --no-typescript --out-dir $(DIST_DIR) $(RELEASE_DIR)/$(APP_NAME).wasm

copy-assets:
	@mkdir -p $(DIST_DIR)
	cp -r $(ASSETS_DIR) $(DIST_DIR)/

copy-html:
	@mkdir -p $(DIST_DIR)
	cp index.html $(DIST_DIR)/

dist: bindgen copy-assets copy-html

clean:
	rm -rf $(DIST_DIR)

serve: dist
	wasm-server-runner $(DIST_DIR)/$(APP_NAME).wasm
