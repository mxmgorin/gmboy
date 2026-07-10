# oxGBC — common developer tasks.
# Run `make` (or `make help`) to list targets.

PORT ?= 8080
WEB  := web

.DEFAULT_GOAL := help
.PHONY: help serve web icons favicon favicon-ox app-icon logo

help: ## List available targets
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| sort \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-12s\033[0m %s\n", $$1, $$2}'

serve: ## Serve the web frontend at http://localhost:$(PORT)
	python3 -m http.server -d $(WEB) $(PORT)

web: ## Build the WASM module + JS bindings into web/pkg
	./$(WEB)/build.sh

icons: favicon app-icon logo ## Regenerate every brand asset

favicon: ## Regenerate the Game Boy Color favicon (web/assets)
	python3 scripts/gen_favicon_gbc.py

favicon-ox: ## Regenerate the "ox" wordmark favicon (web/assets)
	python3 scripts/gen_favicon.py

app-icon: ## Regenerate the Android launcher icon (assets/icon.svg + mipmaps)
	python3 scripts/gen_icon.py

logo: ## Regenerate the oxGBC wordmark logo (assets/logo.svg)
	python3 scripts/gen_logo.py
