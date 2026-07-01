#include "interpolacao.h"
#include <stdio.h>
#include <sys/stat.h>


void criacao_e_visualizacao_de_uma_coluna(void)
{
   struct Coluna input = nova_col(
   "tipos de moedas", (char*[]){
      "penny", "nickel", "quarter", 
      "half-dollar", "dollar" }, 5
   );

   debug_col(input);
   printf("\nUma semirepresentação da estrutura:\n");
   print_col(input);
   free_col(&input);
}

void criacao_de_tabela(void)
{
   struct Coluna in_a = nova_col(
      "tipos de moedas", (char*[]){
         "penny", "nickel", "quarter", 
         "half-dollar", "dollar" }, 5
   );
   struct Coluna in_b = nova_col(
      "quantias", (char*[]){ "15", "8", "23", "2"}, 4
   );
   print_col(in_a); print_col(in_b);

   struct Tabela out = nova_table(true);

   adiciona_table(&out, in_a);
   adiciona_table(&out, in_b);
   print_table(&out);

   free_col(&in_a); free_col(&in_b);
}

static bool JA_ALIMENTADO = false;

static void alimenta(void) {
   if (!JA_ALIMENTADO) {
      FILE* stream = fopen("/dev/random", "rb");
      int bytes[3];
      
      bytes[0] = fgetc(stream);
      bytes[1] = fgetc(stream);
      bytes[2] = fgetc(stream);
      JA_ALIMENTADO = true;

      fclose(stream);
      srand(bytes[0] * bytes[1] * bytes[2]);
   }
}

static const char* genero_aleatorio(void) {
   alimenta();

   switch (rand() % 4) {
      case 0:
         return "F";
      case 1:
         return "M";
      case 2:
         return "B";
      case 3:
         return "S";
   }
}

static char* inteiro_aleatorio(void) { 
   alimenta();

   char* output = calloc(10, sizeof(char));
   int sorteio = 99 + rand() % 1000;

   sprintf(output, "%d", sorteio);
   return output; 
}

static off_t tamanho_arquivo(FILE* arquivo) {
   int fd = fileno(arquivo);
   struct stat info;

   fstat(fd, &info);
   return info.st_size;
}

static char* animal_aleatorio(void) {
   alimenta();

   const char* const PATHNAME = "interpola/tests/animais.txt";
   FILE* arquivo = fopen(PATHNAME, "rt");
   const int sz = sizeof(char), VAZIO = -9;
   int a = VAZIO, b = VAZIO;
   int filesize = tamanho_arquivo(arquivo);
   int distancia = rand() %  (filesize - 10);
   char* output = calloc(30, sz), cursor = '\0';

   fseek(arquivo, distancia, SEEK_CUR);

   do {
      cursor = fgetc(arquivo);

      if (cursor == '\n' && a == VAZIO)
         a = ftell(arquivo);
      else if (cursor == '\n' && b == VAZIO)
         b = ftell(arquivo);

      if (a != VAZIO && b != VAZIO)
         break;

   } while (!feof(arquivo));
   fclose(arquivo);

   arquivo = fopen(PATHNAME, "rt");
   fseek(arquivo, a, SEEK_SET);
   fread(output, sz, b - a -1, arquivo);
   fclose(arquivo);
   
   return output;
}

static void free_matriz_string(char** matriz, int n) {
   for (int q = 0; q < n; q++)
      free(matriz[q]);
   free(matriz);
}

void impressao_de_mais_de_uma_vez_da_tabela(void) {
   srand(*((int*)&genero_aleatorio));

   const int TOTAL = 10 + rand() % 34;
   char** data_c = calloc(TOTAL, sizeof(char*));
   char** data_b = calloc(TOTAL, sizeof(char*));
   char** data_a = calloc(TOTAL, sizeof(char*));

   for (int t = 0; t < TOTAL; t++) 
   { 
      data_b[t] = (char*)genero_aleatorio();
      data_c[t] = inteiro_aleatorio();
      data_a[t] = animal_aleatorio();
   }
   
   struct Coluna In_a = nova_col("Animais", data_a, TOTAL);
   struct Coluna In_b = nova_col("Quantias", data_b, TOTAL);
   struct Coluna In_c = nova_col("Gêneros", data_c, TOTAL);

   struct Tabela out = nova_table(true);

   adiciona_table(&out, In_a);
   adiciona_table(&out, In_b);
   adiciona_table(&out, In_c);

   print_table(&out);
   puts("\n\nImprimindo novamente apenas para verificar se funcionou:");
   print_table(&out);

   // NOTA: 'data_b' não tem memória dinâmica alocada.
}

static void sorteia_animais(void) {
   alimenta();

   for (int n = 1; n <= 60; n++) {
      char* output = animal_aleatorio();

      if (n % 9 == 0)
         putchar('\n');
      printf("'%s', ", output);
   }
   puts("\b\b\n");
}

int main(int total, char* args[], char* envs[]) 
{
   #if defined(__unit_tests__) && defined(__linux__)
   criacao_e_visualizacao_de_uma_coluna();
   criacao_de_tabela();
   // [OFF] sorteia_animais();
   impressao_de_mais_de_uma_vez_da_tabela();
   #else
   #endif

   return EXIT_SUCCESS;
}
