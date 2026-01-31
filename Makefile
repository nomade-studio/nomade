.PHONY: run run-macos run-linux run-windows deps gen build clean

# Variables
APP_DIR := apps/nomade_app
PACKAGES_DIR := packages

# Run the Flutter App
run:
	cd $(APP_DIR) && flutter run

run-macos:
	cd $(APP_DIR) && flutter run -d macos

run-linux:
	cd $(APP_DIR) && flutter run -d linux

run-windows:
	cd $(APP_DIR) && flutter run -d windows

# Dependencies
deps:
	cd $(APP_DIR) && flutter pub get
	@for dir in $(PACKAGES_DIR)/*; do \
		if [ -d "$$dir" ]; then \
			echo "Getting dependencies for $$dir..."; \
			cd $$dir && flutter pub get && cd ../..; \
		fi \
	done

# Code Generation (build_runner & eventually FRB)
gen:
	@echo "Running build_runner..."
	cd $(PACKAGES_DIR)/nomade_domain && dart run build_runner build --delete-conflicting-outputs
	@echo "Running FRB generation..."
	cd $(PACKAGES_DIR)/nomade_native && flutter_rust_bridge_codegen generate

# Build Release
build:
	cd $(APP_DIR) && flutter build macos

# Clean
clean:
	cd $(APP_DIR) && flutter clean
	@for dir in $(PACKAGES_DIR)/*; do \
		if [ -d "$$dir" ]; then \
			cd $$dir && flutter clean && cd ../..; \
		fi \
	done

# CI/CD Checks (Local)
check: check-rust check-flutter

check-rust:
	@echo "Checking Rust..."
	cd core/nomade_core_rs && cargo fmt --all -- --check
	cd core/nomade_core_rs && cargo clippy --all -- -D warnings
	cd core/nomade_core_rs && cargo test --all

check-flutter:
	@echo "Checking Flutter..."
	@echo "Formatting..."
	dart format --set-exit-if-changed $(APP_DIR) $(PACKAGES_DIR)
	@echo "Analyzing..."
	cd $(APP_DIR) && flutter analyze
	cd $(PACKAGES_DIR)/nomade_domain && flutter analyze
	cd $(PACKAGES_DIR)/nomade_native && flutter analyze
	cd $(PACKAGES_DIR)/nomade_protocol && flutter analyze
	cd $(PACKAGES_DIR)/nomade_ui && flutter analyze
	@echo "Testing..."
	@if [ -d "$(APP_DIR)/test" ]; then cd $(APP_DIR) && flutter test; fi
	@if [ -d "$(PACKAGES_DIR)/nomade_domain/test" ]; then cd $(PACKAGES_DIR)/nomade_domain && flutter test; fi
	@if [ -d "$(PACKAGES_DIR)/nomade_protocol/test" ]; then cd $(PACKAGES_DIR)/nomade_protocol && flutter test; fi
	@if [ -d "$(PACKAGES_DIR)/nomade_ui/test" ]; then cd $(PACKAGES_DIR)/nomade_ui && flutter test; fi

format:
	@echo "Formatting Code..."
	cd core/nomade_core_rs && cargo fmt --all
	dart format $(APP_DIR) $(PACKAGES_DIR)
