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
HEADER_C = -Iinterpola/include/
LIB_C = -Ltarget/debug -linterpolacao
LIB_II_C = -Linterpola/lib/ -lteste -ltempo -llegivel -lterminal -lm

compila-lib-to-c:
	cargo rustc --release --package interpolacao \
		--crate-type staticlib --crate-type cdylib 

compila-lib-to-c-debug:
	cargo rustc -p interpolacao --crate-type staticlib --crate-type cdylib 

compila-interpolacao-testes:
	@mkdir -p interpola/bin
	gcc $(HEADER_C) -o interpola/bin/ut_teste_wn \
		interpola/tests/teste_wn.c $(LIB_C)
	gcc $(HEADER_C) -o interpola/bin/ut_teste_tree \
		interpola/tests/teste_tree.c $(LIB_C)
	gcc $(HEADER_C) -o interpola/bin/ut_teste_tree_config \
		interpola/tests/teste_tree_config.c $(LIB_C) $(LIB_II_C) 
		
