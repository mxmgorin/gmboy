# oxGBC — common developer tasks.
# Run `make` (or `make help`) to list targets.

PORT ?= 8080
WEB  := crates/web

.DEFAULT_GOAL := help
.PHONY: help serve web icons favicon favicon-ox app-icon mac-icon win-icon logo

help: ## List available targets
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| sort \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-12s\033[0m %s\n", $$1, $$2}'

serve: ## Serve the web frontend at http://localhost:$(PORT)
	python3 -m http.server -d $(WEB) $(PORT)

web: ## Build the WASM module + JS bindings into crates/web/pkg
	./$(WEB)/build.sh

icons: favicon app-icon mac-icon win-icon logo ## Regenerate every brand asset

favicon: ## Regenerate the Game Boy Color favicon (crates/web/assets)
	python3 tools/gen_favicon_gbc.py

favicon-ox: ## Regenerate the "ox" wordmark favicon (crates/web/assets)
	python3 tools/gen_favicon.py

app-icon: ## Regenerate the Android launcher icon (media/icon.svg + mipmaps)
	python3 tools/gen_icon.py

mac-icon: ## Regenerate the macOS app iconset (media/icon.svg -> media/oxgbc.iconset)
	python3 tools/gen_macos_icon.py

win-icon: ## Regenerate the Windows app icon (media/icon.svg -> media/oxgbc.ico)
	python3 tools/gen_windows_icon.py

logo: ## Regenerate the oxGBC wordmark logo (media/logo.svg)
	python3 tools/gen_logo.py
