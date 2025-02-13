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
HEADER_C = -Iinterpola/include/
LIB_C = -Ltarget/debug -linterpolacao
LIB_II_C = -Linterpola/lib/ -lteste -ltempo -llegivel -lterminal -lm


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
	cargo rustc -q --release --package interpolacao \
		--crate-type staticlib --crate-type cdylib 

compila-lib-to-c-debug:
	cargo rustc -p interpolacao --crate-type staticlib --crate-type cdylib 

compila-interpolacao-testes:
	@mkdir -p interpola/bin
	@gcc $(HEADER_C) -o interpola/bin/ut_teste_wn \
		interpola/tests/teste_wn.c $(LIB_C)
	@echo "'teste_wn.c' compilado em 'interpola/bin'."
	@gcc $(HEADER_C) -o interpola/bin/ut_teste_tree \
		interpola/tests/teste_tree.c $(LIB_C)
	@echo "'teste_tree.c' compilado em 'interpola/bin'."
	@gcc $(HEADER_C) -o interpola/bin/ut_teste_tree_config \
		interpola/tests/teste_tree_config.c $(LIB_C) $(LIB_II_C) 
	@echo "'teste_tree_config.c' compilado em 'interpola/bin'."
		
