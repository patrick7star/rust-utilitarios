
/*! 
 # Dimensao do terminal(tela)
  Tenta obter as dimensões do terminal,
 pois tal biblioteca é muito extensa, e
 utiliza de tal funções bastantes, e nisso
 recorre a bibliotecas externas(aliás
 está usando uma agora). Por isso vamos 
 tentar criar tais funções manualmente
 para desutilizar bibliotecas externas 
 que são utilizadas junto com esta, com
 as usadas internamente aqui para uma 
 função tão simples como tal. 

  O que está sendo feito aqui é na verdade
 uma cópia da atual biblioteca "terminal_size",
 só que feita manualmente, pelos meus meios
 encontrados. Não vi o código original como
 é feito. A estrutura deste será igual a
 dele, dentro de uma estrutura e tal. Porém
 sem a computabilidade de pixels.

  Claro que têm o fato de tal implementação
 ser intensamente mais lenta do que as 
 geralmente utilizadas.
*/

// biblioteca padrão do Rust.
use std::process::Command;
use std::str::FromStr;


/** Estrutura que "embrulha" um inteiro positivo
 de 16-bits, este significando a ***largura***
 do terminal. */
pub struct Largura(pub u16);
/** Estrutura que "embrulha" um inteiro positivo
 de 16-bits, este significando a **altura**
 do terminal. */
pub struct Altura(pub u16);

// apelido para melhorar legibilidade.
type TerminalDimensao = Option<(Largura, Altura)>;
type TermLargura = Result<Largura, &'static str>;
type TermAltura = Result<Altura, &'static str>;


/// de forma direta retorna o Enum contendo
/// apenas a largura do terminal.
pub fn terminal_largura() -> TermLargura {
   // executa comando para obter largura primeiramente ...
   let mut resultado:Vec<u8> = {
      match Command::new("tput").arg("cols").output() {
         // retorna array de bytes que é o resultado.
         Ok(r) => r.stdout,
         Err(_) => 
            { return Err("não foi possível obter 'Largura'"); }
      }
   };

   // removendo quebra de linha.
   resultado.pop();
   // transformando em número.
   let mut caracteres:String = String::new();

   for ch in resultado.into_iter()
      { caracteres.push(ch as char); }

   /* converte para um inteiro positivo, e 
    * e registra valor para retorno, posteriormente. */
   let largura = u16::from_str(caracteres.as_str()).unwrap();
   // retornando encapsulado para possível erro.
   Ok(Largura(largura))
}

/** diretamente retorna o Enum apenas com um
 inteiro de 16-bits *encapsulado* como dado
 dentro dele. */
pub fn terminal_altura() -> TermAltura {
   // executa comando para obter largura primeiramente ...
   let mut resultado:Vec<u8> = {
      match Command::new("tput").arg("lines").output() {
         // retorna array de bytes que é o resultado.
         Ok(r) => r.stdout,
         Err(_) => 
            { return Err("não foi possível obter 'Largura'"); }
      }
   };

   // removendo quebra de linha.
   resultado.pop();
   // transformando em número.
   let mut caracteres:String = String::new();

   for ch in resultado.into_iter()
      { caracteres.push(ch as char); }

   /* converte para um inteiro positivo, e 
    * e registra valor para retorno, posteriormente. */
   let altura = u16::from_str(caracteres.as_str()).unwrap();
   // retornando encapsulado para possível erro.
   Ok(Altura(altura))
}

/** função retorna tupla com dimensão, porém implementação
 distinta da anterior, encapsulando valores com 
 "structs" ao invés de Enum's. */
pub fn dimensao() -> TerminalDimensao {
   /* usando construto acima de auxilio, para 
    * não ter que fazer a mesma coisa de novo.
    */
   let altura:u16 = match terminal_altura() {
      Ok(Altura(h)) => h,
      Err(_) => {return None; }
   };
   let largura:u16 = match terminal_largura() {
      Ok(Largura(l)) => l,
      Err(_) => { return None; },
   };

   // retorno, porém removendo valores dos enum's.
   Some((Largura(largura), Altura(altura)))
}


#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn testa_dimensao() {
      let (largura, altura):(u16, u16);
      if let Some((Largura(l), Altura(h))) = dimensao()
         { largura = l; altura = h; }
      else 
         { largura = u16::MIN; altura = u16::MAX; }
      assert!(largura > altura);
   }
}
