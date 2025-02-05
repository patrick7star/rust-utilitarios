
#ifndef  TERMINAL_H
# define TERMINAL_H
#include <stdint.h>

 /* A dimensão é apenas uma array com dois inteiros positivos de 8-bits,
  * onde o primeiro valor é a 'altura', e o segundo a 'largura'. */
 typedef uint8_t *Dimensao, dimensao_t, DIMENSAO;
 /* Resultado com linhas e colunas representando a dimensão da tela do 
  * console. Tipo de retorno que não é preciso desalocar posteriormente.*/
 struct TerminalSize { uint8_t linhas; uint8_t colunas; };

 /* Tupla com dimensão do terminal, onde o primeiro elemento é o nª de 
  * linhas, e o segundo só pode ser as colunas. */
 extern Dimensao dimensao_terminal(); 
 /* Retorna a dimensão, nos devidos campos nomeados, numa tupla. */
 struct TerminalSize obtem_dimensao (void);

  // Destrói o tipo de dado específico.
  extern void destroi_dimensao(Dimensao d);

#endif 
