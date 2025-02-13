
const char* const amostras_de_frutas[] = {
   "banana", "morango", "pêra", "maçã", "melância",
   "laranja", "tangerina", "uva", "manga", "melão",
   "jaca"
};
const int size_adf = sizeof(amostras_de_frutas) / sizeof(char*);

char* AmostraDeFrutas(void)
   { return (char**)amostras_de_frutas; }

int AmostraDeFrutasSize(void)
   { return sizeof(amostras_de_frutas) / sizeof(char*); }
