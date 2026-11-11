# Makefile de conveniencia para el lenguaje LCE (Qt).
#
# Uso:
#   make build        Compila el motor en modo release.
#   make test         Ejecuta toda la batería de tests con cargo.
#   make test-verbose Igual que test pero mostrando la salida de cada test.
#   make ejemplos     Compila todos los ejemplos .es a binarios nativos.
#   make deb          Construye el paquete .deb.
#   make skill        Instala la skill de OpenCode en ~/.config/opencode/skills.
#   make todo         build + test + ejemplos + deb.
#   make limpiar      Elimina artefactos de compilación.

CARGO ?= cargo
EJEMPLOS_DIR := ejemplos
SALIDA_DIR := /tmp/lce-ejemplos

.PHONY: build test test-verbose ejemplos deb skill todo limpiar

build:
	cd motor && $(CARGO) build --release

test:
	cd motor && $(CARGO) test

test-verbose:
	cd motor && $(CARGO) test -- --nocapture

ejemplos: build
	@mkdir -p $(SALIDA_DIR)
	@for f in $(EJEMPLOS_DIR)/*.es; do \
		nombre=$$(basename $$f .es); \
		printf "[%s] " "$$nombre"; \
		./motor/target/release/lce "$$f" -o "$(SALIDA_DIR)/$$nombre" \
			&& echo "OK"; \
	done

deb: build
	./empaquetado/build-deb.sh

skill:
	@mkdir -p ~/.config/opencode/skills
	@rm -rf ~/.config/opencode/skills/lce
	@cp -r .opencode/skills/lce ~/.config/opencode/skills/lce
	@echo "✔ Skill 'lce' instalada en ~/.config/opencode/skills/lce"

todo: build test ejemplos deb

limpiar:
	cd motor && $(CARGO) clean
	rm -f temp.cpp *.deb
	rm -rf $(SALIDA_DIR)
