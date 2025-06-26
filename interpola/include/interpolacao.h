
#ifndef __C_EXTENSOES_H__
#define __C_EXTENSOES_H__
#include <stdlib.h>
#include <stdbool.h>
#include <stdint.h>

 /* Pega um número positivo inteiro, então escreve todo ele por extenso.
  * O retorno será uma array do tipo narrow caractére com sua representação
  * textual. */
 char* escrita_por_extenso(size_t numero); 

 /* Dado um caminho a um diretório arbitrário, e se colocado a opção de 
  * visualizar os arquivos. As funções abaixo formam toda ela(seus arquivos
  * e subdiretórios), no formato de árvore, com recuo dependendo da subpasta,
  * e etc. A segunda função é o mesmo que a primeira, faz tudo que ela faz,
  * só que mais. Nela você pode definir a profundidade em que é preciso
  * parar de formar, ou também que diretórios pular. 
  */
 char* gera_arvore (const char* caminho, bool visivel);
 char* gera_arvore_config (
   const char* caminho, bool mostra_arquivos, int profundidade, 
   const char** exclusao, int quantia
 );

 /* Faz as tabelas de dadas colunas. Tais colunas são um tipo de estrutura
  * que porta um 'rótulo', este dizendo o que tipo de dado ela retém, e 
  * o 'rol' de determinado tamanho deste. */
 typedef struct Coluna {
   // Que tipo de dados estamos trabalhando.
   const char* rotulo;
   // Sequência de dados em sí, formatados como strings.
   char** rol;  
   // O comprimento deste 'rol'.
   const int length;
   // Maior string do 'rol'.
   size_t largura;

 } Coluna;

 struct Coluna nova_col
   (const char* rotulo, char** rol, int n);

 size_t linhas_col  (struct Coluna);
 size_t largura_col (struct Coluna);
 void   free_col    (struct Coluna*);
 void   debug_col   (struct Coluna);
 void   print_col   (struct Coluna);

 typedef struct Tabela { uint32_t *contador; } Tabela;

 Tabela  nova_table     (bool);
 void    adiciona_table (Tabela*, Coluna);
 void    print_table    (Tabela* );
 
#endif
