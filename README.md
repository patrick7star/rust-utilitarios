# Utilitarios
Quase todas mesmas funções dos `utilitarios_em_python` porém para o Rust. 

módulos:
  - `por_extenso`: pega qualquer número inteiro positivo de 64-bits, e escreve ele em texto.
  - `arvore`: dado um caminho, ele desenha seus subdiretórios e arquivos de forma esquematizada, sendo que se pode escolher se quer apenas sub-diretórios ou arquivos também.
  - `romanos`: escreve números romanos dado um inteiro positivo de 16-bits, dentro do limite é claro. Também transforma uma string representando um romano num inteiro positivo de 16-bits, ou seja, a forma decimal posicional.
  - `legivel`: pega valores de tamanhos(em bytes)/ou tempos(em segundos) grandes e retorna uma string com tais reduziadas a notações científicas, com unidades mais palátaveis.
  - `barra_de_progresso`: tem um monte de "barras de progressos" para variadas situações.
  - `aleatorio`: jeito fácil de gerar números inteiros de 8-bits(não é escalável). Também pode selecionar randomicamente valores de arrays pequenas.
  - `impressao`: listar várias coisas na tela, máximizando o uso da mesma.
  - `tela`: escrever strings, ou realizar vários desenhos na tela, em qualquer posição dada. 
  - `terminal_dimensao`: implementação simples para obter as dimensões do terminal usado.
  - `tabela_visualizacao`: jeito melhor de representar dados de uma tabela no terminal.

### Considerações:
O módulo `tela` só aceita no máximo uma tela de 256(caractéres) de largura/altura, pois é medida em caractéres tal dimensão. No futuro será extendido para uma medida na casa das dezena de milhar.
