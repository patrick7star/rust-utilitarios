
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
const BARRA: char = '+';

struct ColunaStr {
   forma_padrao: String,
   ajuste: Option<usize>,
   iterador: Vec<String>,
   posicao: usize
}

impl ColunaStr {
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
            let diferenca = valor-coluna.linhas();

            for _ in 1..=diferenca 
               { iterador.push(cv.clone()); }
         } None => ()
      };

      Self {
         forma_padrao, ajuste: aumento, 
         iterador, posicao: 0
      }
   }
}

impl Iterator for ColunaStr {
   type Item = String;

   fn next(&mut self) -> Option<Self::Item> {
      if self.posicao <= self.iterador.len()-1 {
         self.posicao += 1;
         Some(self.iterador[self.posicao-1].clone())
      } else { None }
   }
}

fn campo_vago(comprimento: usize) -> String {
   let mut vago = (&" ").repeat(comprimento);
   let meio = comprimento/2 + 1 - 3;
   vago.insert_str(meio, "---");
   vago.pop().unwrap();
   vago.pop().unwrap();
   vago.pop().unwrap();
   vago
}
      

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   #[allow(non_snake_case)]
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
}
