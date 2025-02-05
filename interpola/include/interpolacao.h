
#ifndef __C_EXTENSOES_H__
#define __C_EXTENSOES_H__
#include <stdlib.h>
#include <stdbool.h>

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

#endif
