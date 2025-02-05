#include "interpolacao.h"
#include "teste.h"
#include <stdio.h>
#include <limits.h>

int main(int total, char* args[], char* envs[]) 
{
   const char* caminho = args[1];
   const char* visivel = args[2];
   const char* alcance = args[3];
   char lista[4 * UCHAR_MAX];

   bool a = str_to_bool((char*)visivel);
   int b = atoll((char*)alcance);
   
   printf("path: \"%s\"\ndepth: %d\n", caminho, a, b);
   gera_arvore_config(caminho, a, b, NULL, 0);
   return EXIT_SUCCESS;
}
