
/* Reescrita do código de tabelar
 * tais colunas, baseado no código
 * similar feito em Python. Se 
 * ficar mais simples que o atual,
 * este será substituído pelo novo.
 */

// biblioteca padrão do Rust:
use std::string::ToString;
use std::fmt::{
   Display,  Formatter,
   Result as Resultado
};
use std::marker::Copy;
// extensão do módulo:
use super::objeto::Coluna;

// componente que lacra a tabela.
const BARRA: char = '#';

struct ColunaStr {
   forma_padrao: String,
   /* quantia padrão de linhas da Coluna,
    * ou uma nova preenchido com campos
    * em branco. */
   pub altura: Option<usize>,
   // células e cabeçalho mais espaços vázios.
   iterador: Vec<String>,
   // posição do atual item no iterador.
   posicao: usize,
   // largura da ColunaStr, que é a mesma
   // da Coluna passada na instância.
   pub largura: usize
}

// valor da diferença em absoluto de inteiros:
fn abs(x: usize, y:usize) -> usize {
   if x >= y { x - y }
   else { y - x }
}

impl ColunaStr {
   // método construtor:
   pub fn nova<X>(coluna: Coluna<X>, aumento: Option<usize>) 
   -> Self where X: Display + Copy + Clone {
      let mut iterador: Vec<String> = Vec::new();
      let forma_padrao = format!("{}", coluna);

      for linha in forma_padrao.lines() 
         { iterador.push(linha.to_string()); }

      match aumento {
         Some(valor) => {
            let lrg = coluna.largura();
            //let cv = &"-".repeat(lrg);
            let cv = campo_vago(lrg);
            let diferenca = abs(valor, coluna.linhas());

            for _ in 1..=diferenca 
               { iterador.push(cv.clone()); }
         } None => ()
      };

      Self {
         forma_padrao, altura: aumento, 
         iterador, posicao: 0, 
         largura: coluna.largura()
      }
   }
   // atualiza a quantia de campos vázios da ColunaStr.
   pub fn atualiza(&mut self, aumento: Option<usize>) {
      match aumento {
         Some(valor) => {
            let l = self.largura;
            let ql = self.altura.unwrap();
            let cv = campo_vago(l);
            let diferenca = valor-ql;
            // adicionando a 'diferença' ...
            for _ in 1..=diferenca 
               { self.iterador.push(cv.clone()); }
            // resetando qualquer iteração.
            self.posicao = 0;
         } None => ()
      };
   }
}

// fazendo do objeto um iterador ...
impl Iterator for ColunaStr {
   type Item = String;

   // retorna a próxima célula string da Coluna.
   fn next(&mut self) -> Option<Self::Item> {
      if self.posicao <= self.iterador.len()-1 {
         self.posicao += 1;
         Some(self.iterador[self.posicao-1].clone())
      } else { self.posicao = 0; None }
   }
}

fn campo_vago(comprimento: usize) -> String {
   let mut vago = (&" ").repeat(comprimento);
   // computa o meio da string, recuado alguns passos.
   let meio = comprimento/2 + 1 - 3;
   vago.insert_str(meio, "---");
   // remove os extremos por causa dos "recuos".
   vago.pop().unwrap();
   vago.pop().unwrap();
   vago.pop().unwrap();
   return vago;
}

struct Tabela {
   // lista das strings já consertadas das Colunas.
   lista: Vec<ColunaStr>,
   /* diz se quer que use o máximo da 
    * largura do atual terminal preenchendo
    * com o restante da tabela, ao invés de
    * fazer um contínuo "pergaminho". */
   preenche_tela: bool,
   /* Salva construção inicialmente feita, pois
    * o processo de faze-lô é sempre custoso, 
    * então não dá para ficar reconstruindo 
    * em cada chamada.  */
   tabela_str: String,
   // maior quantia de linhas até agora da tabela.
   maior_ql: Option<usize>,
   // se é a primeira inserção.
   primeira: bool,
}

impl Tabela {
   // método construtor:
   pub fn nova(maximo_de_tela: bool) -> Self {
      Self { 
         lista: Vec::new(), 
         preenche_tela: maximo_de_tela,
         tabela_str: String::new(),
         maior_ql: None, 
         primeira: maximo_de_tela
      }
   }
   // adiciona nova Coluna passada.
   pub fn adiciona<Y>(&mut self, coluna: Coluna<Y>) 
   where Y: Display + Clone + Copy {
      // atualizando primeiramente altura da tabela.
      match self.maior_ql {
         Some(h) => {
            if h < coluna.linhas()
               { self.maior_ql = Some(coluna.linhas()); }
         } None => 
               { self.maior_ql = Some(coluna.linhas()); }
      };
      // transforma a mesma numa versão textual.
      let aumento: Option<usize>;
      if self.primeira { aumento = None; }
      else { aumento = self.maior_ql; }
      let coluna_str = ColunaStr::nova(coluna, aumento);
      // insere na memória da tabela.
      self.lista.push(coluna_str);
      // refaz desenho da tabela.
      self.desenha_tabela();
   }
   // método auxiliar para construção da tabela como string.
   fn desenha_tabela(&mut self) {
      // limpa string antiga.
      self.tabela_str.clear();
      /* atualizando altura das devidas
       * colunas para que estejam todas
       * numa "altura" comum. */
      for coluna_str in self.lista.iter_mut() 
         { coluna_str.atualiza(self.maior_ql); }

      let mut mql:usize = self.maior_ql.unwrap();
      /* concatenando cada célula de cada 
       * 'coluna textual', capturando elas 
       * dos estágios em suas devidas estágio
       * da iteração. */
      while mql > 0 {
         for coluna_str in self.lista.iter_mut() {
            let ts = coluna_str.next().unwrap();
            self.tabela_str.push(BARRA);
            self.tabela_str.push_str(ts.as_str());
         }
         // quebra de linha.
         if !self.primeira
            { self.tabela_str.push(BARRA); }
         self.tabela_str.push('\n');
         mql -= 1;
      }
   }
}

impl Display for Tabela {
   fn fmt(&self, molde:&mut Formatter<'_>) -> Resultado 
      { write!(molde, "{}", self.tabela_str) }
}
      

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;

   #[test]
   fn testa_struct_ColunaStr() {
      let moedas_magicas = Coluna::nova(
         "moedas mágicas(qtd.)",
         vec![198, 1923, 038, 932, 38_839,
            3, 12, 538, 752, 65, 129]
      );

      let salarios = Coluna::nova(
         "salário(R$)",
          vec![5288.32, 1_000.10, 893.25, 1_839.00]
      );

      let generos = Coluna::nova(
         "gênero(macho/fêmea)", 
          vec!['F','F','F','F','M','M',
             'F','M','M','F','F','F','M']
      );

      let altura = Some(generos.linhas());

      let mut strI = ColunaStr::nova(moedas_magicas, altura);
      let mut strII = ColunaStr::nova(salarios, altura);
      let mut strIII = ColunaStr::nova(generos, altura);


      for _ in 1..=altura.unwrap() {
         let c1 = strI.next().unwrap();
         let c2 = strII.next().unwrap();
         let c3 = strIII.next().unwrap();
         println!("{}#{}#{}#", c1, c2, c3);
      }

      // avaliação manual.
      assert!(false);
   }

   #[test]
   fn testa_struct_Tabela() {
      let moedas_magicas = Coluna::nova(
         "moedas mágicas(qtd.)",
         vec![198, 1923, 038, 932, 38_839,
            3, 12, 538, 752, 657]
      );

      let salarios = Coluna::nova(
         "salário(R$)",
          vec![5288.32, 1_000.10, 893.25, 1_839.00]
      );

      let generos = Coluna::nova(
         "gênero(macho/fêmea)", 
          vec!['F','F','F','F','M','M',
             'F','M','M','F','F','F','M']
      );

      let nomes = Coluna::nova(
         "candidatos", vec!["Claúdio", "Ana Joana",
         "Marcelo", "Flávia", "Henrique Barcelos"]
      );

      let mut t = Tabela::nova(false);
      t.adiciona(moedas_magicas);
      println!("primeiro resultado:\n{}", t);
      t.adiciona(generos);
      println!("\tsegundo resultado:\n{}", t);
      t.adiciona(salarios);
      println!("\t\tterceiro resultado:\n{}", t);
      t.adiciona(nomes);
      println!("\n{}", t);
   }
}
