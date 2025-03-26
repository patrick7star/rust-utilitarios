/*!
 # Visualiazação de tabelas
  Um bom modo de organizar dados tabelados. O módulo possuí uma estrutura 
 onde você pega todo 'rol' de dados, cede um 'rótulo' a ele e toda vez que 
 impresso será visualizado fechado por caractéres `Unicode` de uma maneira 
 bem formatada no terminal. A estrutura `Coluna` que proporciona isso, 
 também aceita a impressão de outras juntas, formando assim uma bonita 
 tabela.
*/

// Extensão deste código:
mod string_complemento;
mod objeto;
mod tabelacao;
mod matriz_texto;
mod revestimento;
mod iterador;
mod macros;
// Re-exportando construções dos submódulos ...
pub use objeto::Coluna;
pub use tabelacao::Tabela;

// -------- testes do módulo ------------
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
