#include "interpolacao.h"
#include <stdio.h>

void cria_simples_arvore_e_imprime(char* pathname)
{
   char* output = gera_arvore(pathname, true);

   /* Imprime a árvore formada, então libera a string. Na formação, tal 
    * string cuspida é construída dinamicamente, portanto, alocada na 
    * heap do stack de memória. */
   printf("\n%s\n", output);
   free(output);
}

int main(int total, char* args[], char* envs[]) 
{
   if (total == 2) { 
      const char* caminho = args[1];

      cria_simples_arvore_e_imprime((char*)caminho);
   } else {
      perror("Argumentos inválidos.");
      abort();
   }

   return EXIT_SUCCESS;
}
