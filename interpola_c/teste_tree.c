#include "cextensoes.h"
#include <stdio.h>

void cria_simples_arvore_e_imprime(char* path)
{
   char* output = gera_arvore(path, true);

   printf("\n%s\n", output);
   free(output);
}

int main(int total, char* args[]) 
{
   if (total == 2) { 
      const char* caminho = args[1];

      cria_simples_arvore_e_imprime(caminho);
   } else {
      perror("Argumentos inválidos.")
      abort();
   }

   return 0;
}
