VERSAO 	= 1.6.3
CAMINHO 	= ../versões/utilitarios

compila-bibliotecas: compila-lib-to-c
	@cargo build --release --quiet
	@echo "Biblioteca estática do Rust compilada."
	@cargo rustc -q --release --lib --crate-type dylib
	@echo "Biblioteca compartilhada do Rust compilada."
	@mkdir -p lib/
	@mv -v target/release/*.rlib target/release/*.so target/release/*.a lib/
	@echo "Artefatos compilados movidos para diretório 'lib'."

salva:
	tar --exclude=target -cvf $(CAMINHO).v$(VERSAO).tar \
		Cargo.toml Makefile README.md src/ tests/
	@echo "Backup da versão $(VERSAO) realizado com sucesso."

backups:
	@echo "\nListagem de todos backups feitos no computador:\n"
	@ls --human-readable --size --sort=time -1 $(CAMINHO)*.tar

clean:
	@cargo clean
	@rm -r lib/
	@echo "Tudo foi devidamente removido."

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
export LD_LIBRARY_PATH := "$(LD_LIBRARY_PATH):./target/debug"

HEADER_C				= ./interpola/include/
SEARCH_PATH_C_DBG = ./target/debug
SEARCH_PATH_C_RLS = ./target/release
LIB_UTILS			= ./interpola/lib
SEARCH_PATH_C		= $(SEARCH_PATH_C_DBG)


UTILS = $(CCODES)/utilitarios-em-c
# Comando só funciona na minha máquina local(nunca execute). Na sua máquina
# os binários já virão instalados no pacote.
instala-bibliotecas-necessarias:
	@cp -u -v $(UTILS)/bin/static/libterminal.a $(UTILS)/include/terminal.h \
		./interpola/lib/
	@cp -u -v $(UTILS)/bin/static/libteste.a $(UTILS)/include/teste.h \
		./interpola/lib/
	@cp -u -v $(UTILS)/bin/static/libtempo.a $(UTILS)/include/tempo.h \
		./interpola/lib/
	@cp -u -v $(UTILS)/bin/static/liblegivel.a $(UTILS)/include/legivel.h \
		./interpola/lib/
	@mv -v ./interpola/lib/*.h ./interpola/include

test-tree:
	@gcc -I$(HEADER_C) -o bin/test-tree			\
		interpola/tests/tree.c									\
		-L$(SEARCH_PATH_C) -linterpolacao -lm
	@echo "'teste_tree.c' compilado em 'interpola/bin'."		
	@gcc -I$(HEADER_C) -o bin/test-tree-config \
		interpola/tests/tree-config.c							\
		-L$(SEARCH_PATH_C) -linterpolacao							\
		-L./interpola/lib/												\
			-lteste -lterminal -ltempo -llegivel -lm
	@echo "'teste_tree_config.c' compilado em 'interpola/bin'."

test-writing-numerals:
	@gcc -I$(HEADER_C) -o bin/test-writing-numerals			\
		interpola/tests/writing-numbers.c									\
		-L$(SEARCH_PATH_C) -linterpolacao -lm
	@echo "'writing-numerals.c' compilado em 'interpola/bin'."		
