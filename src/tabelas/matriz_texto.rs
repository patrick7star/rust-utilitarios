/*!
 Fazendo um objeto que cuida do desenho, como funções relacionadas a ele 
 diretamente: concatenação, aparação de espaços em brancos e etc.
*/

/* Capacidade baseado na dimensão máxima do laptop, que foi primeiramente 
 * codificado. Resolução da tela dele é 1366x768 pixels. */
const MAX_LARGURA: usize = 250;
const MAX_ALTURA: usize = 1000;
const FUNDO: char = ' ';

// Apelidos para facilitar na codificação.
type MultiArray = Vec<Vec<char>>;
pub type MT = MatrizTexto;


#[derive(Clone)]
pub struct MatrizTexto {
   matriz: Vec<Vec<char>>,
   altura: u16,
   largura: u16,
}

impl MatrizTexto {
   pub fn cria(altura: u16, largura: u16) -> Self {
      let mut matriz: MultiArray;
      matriz = Vec::with_capacity(MAX_ALTURA);
      
      for _ in 0..altura {
         let mut linha: Vec<char>;
         linha = Vec::with_capacity(MAX_LARGURA);
         for _ in 0..largura
            { linha.push(FUNDO); }
         matriz.push(linha);
      }
      return MatrizTexto { matriz, altura, largura }
   }

   // Muda célula na matriz.
   pub fn set(&mut self, y: u16, x: u16, ch: char) 
      { self.matriz[y as usize][x as usize] = ch; }

   // Obtém célula na matriz.
   pub fn get(&self, y: u16, x: u16) -> char 
      { self.matriz[y as usize][x as usize] }

   /* Tupla com dimensão da matriz: altura e largura respectivamente. */
   pub fn dimensao(&self) -> (u16, u16) 
      { (self.altura, self.largura) }

   /* Transforma uma string -- é necesário que ela seja múltilinha -- numa 
    * matriz-texto. */
   pub fn to_matriz(string: &str) -> Self {
      let largura: usize = {
         string.lines()
         .map(|linha| linha.chars().count())
         .max().unwrap()
      };
      let altura: u16 = string.lines().count() as u16;
      let mut matriz = MatrizTexto::cria(
         altura as u16, 
         largura as u16
      );

      for (y, linha) in string.lines().enumerate() {
         let mut x: u16 = 0;
         for char in linha.chars() {
            matriz.set(y as u16, x, char);
            x += 1;
         }
      }

      return matriz
   }
}

impl ToString for MatrizTexto {
   fn to_string(&self) -> String {
      let l = self.largura as usize;
      let a = self.altura as usize;
      /* quantia de caractéres, mais as
       * quebra-de-linhas. Não está alocado para
       * todo o tamanho, porque alguns caractéres
       * não são ASCII, então custosos
       * redimensionamentos serão feitos, porém
       * algumas unidades de vezes nada
       * estrememanete lento. */
      let mut esboco = String::with_capacity(a * l + l);

      for y in 0..a {
         for x in 0..l 
            { esboco.push(self.matriz[y][x]); }
         // acaba com uma linha, adiciona quebra-de-linha.
         esboco.push('\n');
      }
      return esboco;
   }
}

