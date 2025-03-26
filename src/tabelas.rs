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

   #[test]
   fn verificando_importacao_de_macros() {
      let col_a = crate::cria_coluna!(
         "Animais de fazenda": [
            "coelho", "galinha", "ovelha", "boi", "vaca", "galo",
            "cachorro", "porco", "cadumongo", "avestruz"
         ]
      );
      let col_b = crate::cria_coluna!(
         "Suas cores": [
            "branco", "amarela-queimado", "branco", "preto", "preto e branco"
            , "azul-escuro", "caramelo", "rosa", "marron"
         ]
      );
      let mut table = Tabela::nova(true);

      println!("colunas: {}\n{}", col_a, col_b);
      table += col_a;
      table += col_b;
      println!("table:\n{}", table);

      let table_a = crate::cria_tabela!(
         "Itens": [ "copo", "relógio", "livro", "cadeado", "cesto" ];
         "Preços(RS)": [5.12, 34.2, 13.4, 8.99];
         "estoque(unidades)": [5, 9, 28, 50, 30]
      );
      println!("{table_a:}");
   }
}
