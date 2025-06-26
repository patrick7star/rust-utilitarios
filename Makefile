VERSAO 	= 1.6.2
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
#-lteste -ltempo -llegivel -lterminal -lm


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

compila-lib-to-c:
	@echo $(LD_LIBRARY_PATH)
	cargo rustc -q --release --package interpolacao \
		--crate-type staticlib -crate-type cdylib 

compila-lib-to-c-debug:
	cargo rustc -p interpolacao --crate-type staticlib --crate-type cdylib \
   -- -C debug-assertions -C debuginfo=full

compila-table-test:
	@gcc -I$(HEADER_C) -ggdb -O0 -D__unit_tests__								\
      -o interpola/bin/ut_teste_tables interpola/tests/teste_tables.c	\
		-L$(SEARCH_PATH_C) -linterpolacao
	@echo "'teste_tables.c' compilado em 'interpola/bin'."

compila-tree-test:
	@gcc -I$(HEADER_C) -o interpola/bin/ut_teste_tree			\
		interpola/tests/teste_tree.c									\
		-L$(SEARCH_PATH_C) -linterpolacao
	@echo "'teste_tree.c' compilado em 'interpola/bin'."		
	@gcc -I$(HEADER_C) -o interpola/bin/ut_teste_tree_config \
		interpola/tests/teste_tree_config.c							\
		-L$(SEARCH_PATH_C) -linterpolacao							\
		-L./interpola/lib/												\
			-lteste -lterminal -ltempo -llegivel -lm
	@echo "'teste_tree_config.c' compilado em 'interpola/bin'."

compila-writting-numbers-test:
	@gcc -I$(HEADER_C) -O0									\
		-o interpola/bin/ut_teste_writting_numbers	\
			interpola/tests/teste_writting_numbers.c	\
		-L$(SEARCH_PATH_C) -linterpolacao
	@echo "'teste_wn.c' compilado em 'interpola/bin'."

atualiza-binarios-compilados:
	@cp --update=older --verbose				\
		target/release/libinterpolacao.so	\
		target/release/libinterpolacao.a		\
		./lib/
	@echo "Nova compilação da 'interpolação' foram copiadas."

compila-interpolacao-testes:		\
	compila-writting-numbers-test \
	compila-tree-test					\
	compila-table-test
		
