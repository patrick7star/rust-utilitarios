#include "interpolacao.h"
#include <stdio.h>

void aplica_traducao_do_numero(size_t numero)
{
   char* num_str = escrita_por_extenso(numero); 

   printf("\t\b\b\b%zu ===> %s\n", numero, num_str);
   free(num_str);
}

int main(int total, char* args[]) 
{
   if (total >= 1) 
   {
      for (int k = 1; k < total; k++) 
      {
         char* numero_str = args[k];
         size_t numero = atoll(numero_str);

         aplica_traducao_do_numero(numero);
      }
   } else 
      puts("Você tem que digitar os valores.");

   return 0;
}
