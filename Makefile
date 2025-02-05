VERSAO 	= 1.6.1
CAMINHO 	= ../versões/utilitarios

compila-bibliotecas:
	cargo build --release 
	cargo rustc --release --lib --crate-type dylib
salva:
	tar --exclude=target -cvf $(CAMINHO).v$(VERSAO).tar \
		Cargo.toml Makefile README.md src/ tests/
	@echo "Backup da versão $(VERSAO) realizado com sucesso."

backups:
	@echo "\nListagem de todos backups feitos no computador:\n"
	@ls --human-readable --size --sort=time -1 $(CAMINHO)*.tar

# === === ===  === === === === === === === === === === === === === === ====
#						Todos Testes do Rust via Makefile	
# === === ===  === === === === === === === === === === === === === === ====
TodosTestesUnitariosSimples = funcaoTL funcaoTA testa_dimensao \
										funcaoNativaDoOS
TestesUnitariosComOutputDinamico = valorRecuperadoEmTempoDeExecucao


$(TodosTestesUnitariosSimples):
	cargo test --quiet -- --show-output $@

$(TestesUnitariosComOutputDinamico):
	cargo test --quiet -- --nocapture --show-output $@

check:
	cargo check --tests

# === === ===  === === === === === === === === === === === === === === ====
#						Iterpolaridade com código C/C++.
# === === ===  === === === === === === === === === === === === === === ====
HEADER_C = interpola_c/

compila-lib-to-c:
	cargo rustc --release --package interpola_c \
		--crate-type staticlib --crate-type cdylib 

compila-lib-to-c-debug:
	cargo rustc -p interpola_c --crate-type staticlib --crate-type cdylib 

compila-interpola-c-testes:
	@mkdir -p interpola_c/bin
	gcc -I$(HEADER_C) -o interpola_c/bin/ut_teste_wn \
		interpola_c/tests/teste_wn.c \
		-Ltarget/debug/ -linterpola_c
	gcc -I$(HEADER_C) -o interpola_c/bin/ut_teste_tree \
		interpola_c/tests/teste_tree.c \
		-Ltarget/debug/ -linterpola_c
