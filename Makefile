# 変数の定義
TARGET = touch
DEST = /usr/local/bin
BUILD_DIR = ./target/debug

.PHONY: all build install clean

# デフォルトターゲット
all: build

# Rustプロジェクトのビルド
build:
	cargo build

# インストール
install: build
	cp $(BUILD_DIR)/$(TARGET) $(DEST)/

# クリーンアップ
clean:
	cargo clean

