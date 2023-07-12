
/*! Macros importantes para este módulo. Estes
 aqui são os macros mais importante até agora
 na biblioteca como todo. A estrutura de tabela
 é muito importante, porém bem difícil de 
 construir-se manualmente. Porém tais macros,
 reduzem o tempo de codifição bastante, sem falar
 que facilita em escrever quase tudo no automático.
 */

#[allow(unused_imports)]
use crate::tabelas::{Coluna, Tabela};

/// macro auxiliar para a tabela, porém também será exportado.
#[macro_export]
macro_rules! cria_coluna {
   ( $rotulo:tt : [ $( $celula:expr ),+  ]  ) => (
      Coluna::nova($rotulo, vec![$($celula),+])
   );
}

/** Macro que gera cada tabela, evitando assim
 a situação chata de ter que instancia-la, assim
 como instanciar cada coluna, e adiciona-las 
 posteriormente. */
#[macro_export]
macro_rules! cria_tabela {
   ( $( $rotulo:tt : [ $( $items:expr ),+ ] );* ) => ({
      let mut table = Tabela::nova(false);
      $( 
         let arrai = vec![ $($items),+ ];
         let column: Coluna<_>;
         column = Coluna::nova($rotulo, arrai);
         table.adiciona(column);
      )*
      table
   });
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;

   #[test]
   fn macroCriaColuna() {
      let cA = Coluna::nova(
         "lançamento moedas",
         vec![
            "Cara", "Coroa",
            "Coroa", "Cara",
            "Cara", "Cara",
            "Coroa", "Coroa",
         ]
      );
      println!("{:#?}", cA);
      let cB = cria_coluna!(
         "lançamento moedas": [
            "Cara", "Coroa",
            "Coroa", "Cara",
            "Cara", "Cara",
            "Coroa", "Coroa"
         ]
      );
      println!("versão mais direta e organizada:\n{:#?}", cB);
      assert_eq!(cA, cB);
   }
   #[test]
   fn macroTabela() {
      let moedas_magicas = Coluna::nova(
         "moedas mágicas(qtd.)",
         vec![198, 1923, 038, 932, 38_839,
            3, 12, 538, 752, 657]
      );
      let salarios = Coluna::nova(
         "salário(R$)",
          vec![5288.32, 1_000.10, 893.25, 1_839.00]
      );
      let generos = cria_coluna!{
         "gênero(macho/fêmea)" : [
            'F','F','F','F','M','M',
             'F','M','M','F','F','F','M'
          ]
      };
      let nomes = cria_coluna!(
         "candidatos": ["Claúdio", "Ana Joana",
         "Marcelo", "Flávia", "Henrique Barcelos"]
      );

      // método antigo.
      let mut tA = Tabela::nova(false);
      tA.adiciona(moedas_magicas);
      tA.adiciona(generos);
      tA.adiciona(salarios);
      tA.adiciona(nomes);

      let tB = cria_tabela!(
         // total de moedas mágicas:
         "moedas mágicas(qtd.)": [
            198, 1923, 038, 932, 38_839,
            3, 12, 538, 752, 657
         ];
         // equivalente gêneros:
         "gênero(macho/fêmea)" : [
            'F','F','F','F','M','M',
             'F','M','M','F','F','F','M'
          ];
         // cédulas de salários:
         "salário(R$)": [5288.32, 1_000.10, 893.25, 1_839.00];
         // nome de alguns candidatos:
         "candidatos": [
            "Claúdio", "Ana Joana", "Marcelo", 
            "Flávia", "Henrique Barcelos"
         ]
      );
      println!("método antigo:\n{}\nmétodo novo:\n{}", tA, tB);
      // confirmação visual.
      assert!(true);
   }
}
