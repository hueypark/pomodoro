# Makefile for Bevy WASM Web Deployment

# Variables
APP_NAME := pomodoro
TARGET := wasm32-unknown-unknown
RELEASE_DIR := target/$(TARGET)/release
DIST_DIR := dist
ASSETS_DIR := assets

.PHONY: all build copy-assets copy-wasm copy-html dist clean serve

all: dist

build:
	cargo build --release --target $(TARGET)

copy-wasm: build
	@mkdir -p $(DIST_DIR)
	cp $(RELEASE_DIR)/$(APP_NAME).wasm $(DIST_DIR)/

copy-assets:
	@mkdir -p $(DIST_DIR)
	cp -r $(ASSETS_DIR) $(DIST_DIR)/

copy-html:
	@mkdir -p $(DIST_DIR)
	cp index.html $(DIST_DIR)/

dist: copy-wasm copy-assets copy-html

clean:
	rm -rf $(DIST_DIR)

serve: dist
	wasm-server-runner $(DIST_DIR)/$(APP_NAME).wasm
