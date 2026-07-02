/** Colocando a coluna aqui, novamente, por motivos de organização. Tudo 
 * fica no mesmo arquivo dificulta "exponencialmente" o processo de 
 * releitura do que já foi codificado. */

// biblioteca do Rust:
use std::fmt::{Formatter, Display, Result as Resultado};
use super::string_complemento::{StringExtensao as StrExt};

// Abreviação por motivo de legibilidade.
/** 
 * Uma estrutura que representa uma coluna numa tabela de dados. Necessário 
 * um **rótulo** e uma array representando o **rol** de dados da legenda.
 */
#[derive(Debug, Clone)]
pub struct Coluna <U: ToString + Clone> {
   // Legenda do rol de dados.
   rotulo: String,

   // Rol de dados.
   rol:Vec<U>,

   // Largura máxima da coluna.
   largura: usize,
}

impl <U>Coluna<U> where U: ToString + Clone 
{
   pub fn nova(rotulo: &str, rol: Vec<U>) -> Self {
      let maior_do_rol: usize = {
         rol.iter()
         .map(
            |s| StrExt::len(&s.to_string())
         ).max().unwrap()
      };
      let largura = {
         [maior_do_rol, rotulo.len()]
         .iter().max()
         .unwrap()
         .clone()
      };

      Self { rotulo: rotulo.to_string(), rol, largura }
   }

   pub fn linhas(&self) -> usize
      { self.rol.len() + 1 }
   pub fn largura(&self) -> usize
      { return self.largura; }
}

impl<U:Display + Clone> Display for Coluna <U> {
   fn fmt(&self, molde:&mut Formatter<'_>) -> Resultado 
   {
      // String com todas concatenações.
      let mut output = String::from("");
      // Maior comprimento de caractéres:
      let mut max_length: u8;
      let mut length: u8;
      let mut ajuste: String;

      // A referência de maior string é a do rótulo.
      max_length = self.rotulo.len() as u8;
      
      // Busca maior comprimento que a atual referência...
      #[allow(non_snake_case)]
      for X in self.rol.clone() 
      {
         length = X.to_string().len() as u8;

         if length > max_length 
            { max_length = length; }
      }

      ajuste = calibra_str(&self.rotulo, max_length);
      output.push_str(ajuste.as_str());
      output.push('\n');

      for v in self.rol.clone() {
         ajuste = calibra_str(
            v.clone().to_string()
            .as_str(), max_length
         );
         output.push_str(&ajuste);
         output.push('\n');
      }
      // escrevendo no formatdor...
      write!(molde, "{}", output)
   }
}

impl<U> PartialEq for Coluna<U> 
  where U: PartialEq + Eq + Clone + ToString
{
   fn eq(&self, outro: &Self) -> bool {
      if self.rotulo != outro.rotulo { false }
      else {
         self.rol.iter().zip(outro.rol.iter())
         .all(|(s, r)| s == r)
      }
   }
   fn ne(&self, outro: &Self) -> bool 
      { !self.eq(outro) }
}

/** Centraliza uma string dado a largura onde ela deve estar envolvida. */
fn calibra_str(s:&str, limite:u8) -> String {
   // diferença entre a slice-string e o máximo aceitável.
   //let d:u8 = limite - s.len_2_bytes();
   let d: u8 = limite - (StrExt::len(s) as u8);
   // verifica se é ímpa?
   let e_impa = d % 2 != 0;
   // string para concatenação.
   let mut outra_str = String::from(s);
   
   // se for ímpa, adiciona 'char' à esquerda para calibrar.
   if e_impa { outra_str.push(' '); }
   
   // se for dois, adiciona apenas uma vez manualmente.
   if d == 0 
      { return String::from(s); }
   if d == 2 {
      outra_str.push(' ');
      outra_str.insert(0,' ');
   }
   // ser for um par maior que doiz adiciona 'd/2' vezes
   // em cada lado.
   else {
      let fim = d / 2;
      for _ in 0..fim {
         outra_str.push(' ');
         outra_str.insert(0,' ');
      }
   }
   // retorna string ajustada.
   return outra_str;
}


#[cfg(test)]
mod tests {
   use super::*;

   #[test] 
   fn tipo_generico_teste() {
      let idds = Coluna::nova(
         "idade(anos)",
         vec![12,15, 18, 20,10, 5, 3]
      );
      let pesos = Coluna::nova(
         "massa(kg)",
         vec![60.32, 58.21, 70.32, 
             55.1, 37.1, 10.08]
      );
      let generos = Coluna::nova(
         "gênero(macho/fêmea)", 
         vec!['F','M','M','F','M','M','F','F','F','M']
      );

      println!("{}",pesos);
      println!("{}", idds);
      println!("{}", generos);

      assert!(true);
   }
}
