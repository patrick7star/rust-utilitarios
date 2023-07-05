
/*!
 # O que são estes Utilitários 
  Todos códigos que não tiverem elaborações bem 
 complexas, ou seus esboços iniciais e simples, 
 porém bem úteis ficarão aqui. Isto é muito melhor
 que ao invés de criar um `crate` para cada um.
 Códigos que são também repetidos por várias `libs`,
 serão colocadas aqui, por motivos óbvios.
  Como disse antes tais funções e estruturas de cada
 módulo executam coisas muitos simples.

 Outra razão para termos isso aqui é, um jeito de 
 burlar a internet e pacotes de terceiros. Toda 
 vez que são compilados *caixotes* do **Rust**, aqueles
 que depedem de pacotes teceiros, se não houver 
 *artefatos* no computador, então procuram na 
 internet, se não houver qualquer conexão, você 
 não conseguirá compilar, consequentemente, executar
 o programa; por isso é importante uma biblioteca 
 que dependa menos e menos de internet, ou quase 
 nada. Também ficam implementações do ***autor***,
 que não são para ser um substituto, mas uma tentativa
 de implementação própria, seja para aprendizado, ou
 apenas diversão, e apromimoramento de novas técnicas
 recentemente aprendidas da **linguagem Rust**, ou
 de computação em geral.
*/


/** dado um diretório desenha uma árvore em string
 baseando nos arquivos e diretórios do atual(raíz)
 e seus subdirs.
*/
pub mod arvore;

mod tela_auxiliar;
/** talvez o mais complexo do pacote, cria uma estrura
 para manipular de forma maleável a impressão de
 texto e desenhos simples no terminal.
*/
pub mod tela;

/** converte valores inteiros e fluantes, que representam
 grandezas importantes em computação, para valores
 legíveis, tais na formatação de strings. 
*/
pub mod legivel;

/** 
  Pega lista e arrays de dados e faz uma tabela
 delas para simples impressões.
*/
pub mod tabelas;

/** Gerador de números pseudo-aleatórios. Todos
 tipos de dados inteiros estão inclusos no pacote,
 inclusive não-inteiros como valores de "decimais".
 */
pub mod aleatorio;

/** Transforma strings e inteiros que representam valores
 decimais/ou binários para números romanos, o inverso
 também, ou seja, romanos para números decimais/inteiros.
*/
pub mod romanos;

/// Incrementa vários modos de impressão na tela.
pub mod impressao;

/** 
  Variádos tipos de barras de progressos, que informam
 de forma dinâmica como a computabilidade de tais 
 dados abordados está indo. */
pub mod barra_de_progresso;

/** 
  Obtém a dimensão do terminal que roda tal lib. 
 Portanto agora, podemos excluir a biblioteca externa
 utilizada para fazer tal, substituindo por esta. 
*/
pub mod terminal_dimensao;

/// escreve um número dado por extenso.
pub mod por_extenso;


use std::io::{Write, stdin, stdout};
/** Como sem um nome de módulo no momento, vamos 
  colocar aqui a implementação de um prompt 
  genérico. 
 */
pub fn lanca_prompt(dica:&str) -> String {
   // formantando dica.
   let dica = format!("{}: ", dica);
   let mut saida = stdout();
   let entrada = stdin();
   // buffer.
   let mut conteudo = String::new();

   // escreve o prompt.
   saida.write(dica.as_bytes()).unwrap();
   // sem guardar no buffer para ir antes do 'stdin'.
   saida.flush().unwrap();

   // retornando conteudo digitado.
   entrada.read_line(&mut conteudo).unwrap();
   // removendo quebra-de-linha.
   drop(conteudo.pop().unwrap());
   return conteudo;
}
/** Macro importante para agilizar na criação
 de um Mapa(HashMap). A forma que tem que ser
 escrito é, dado dois tipos, lado esquerdo
 fica a chave, lado direito o valor. Até que 
 se saiba, funciona com quase todos tipos
 primitivos.
 */
#[macro_export]
macro_rules! mapa {
   /* inferir a 'chave' e o 'valor'. 
    * Estamos falando numa declaração do tipo:
    * let dicionario = meu_mapa!(
    *    1: "primeiro",
    *    2: "segundo",
    *    3: "terceiro",
    * );
    * let dicionario = meu_mapa!(
    *    'A': 65,
    *    'B': 66,
    *    'C': 67,
    * );
    */
   ( $( $chave:tt : $valor:tt ),+ ) => ({
      use std::collections::HashMap;
      use std::iter::FromIterator;
      let mapa: HashMap<_, _>;
      let array = vec![ $( ($chave, $valor) ),+ ];
      mapa = HashMap::from_iter(array.into_iter());
      mapa
   });
   /*
   ({ $($chave:tt : $valor:tt),+ }) => {
      JSON::Object(Box::new(
         HashMap::<String, JSON>::from_iter(
            vec![$( ($chave.to_string(), json!($valor)) ),+ ]
            .into_iter().collect::<Vec<(String, JSON)>>()
         )
      ))
   };*/
}

#[cfg(test)]
mod tests {
   use super::*;
   use std::str::FromStr;
   #[test]
   fn testa_lanca_prompt() {
      let msg:&str = "digite mensagem-chave('chocolate')";
      let conteudo = super::lanca_prompt(msg);
      if cfg!(windows) { 
         // desconsiderando o recuo.
         let fim = conteudo.len() - 1;
         let chocolate = conteudo.get(0..fim).unwrap();
         assert_eq!(chocolate, "chocolate");
      } else 
        { assert_eq!(conteudo, "chocolate"); }
   }

   #[test]
   fn converte_romano() {
      // número romano digitado.
      let nr = super::lanca_prompt("digite um número romano");
      // conversão para decimal.
      let nd = super::romanos::romano_para_decimal(nr.as_str());
      // decimal esperado.
      let de = super::lanca_prompt("qual o decimal esperado");
      let de = u16::from_str(de.as_str()).unwrap();
      println!("{} ==> {}", nr, nd);
      assert_eq!(de, nd);
   }

   #[test]
   fn declaracao_de_mapas_via_macro() {
      let dicio = mapa!(
         'a': 10,
         'z': 100,
         'b': 1000,
         'y': 10_000,
         'c': 100_000
      );
      println!("{:?}", dicio);
      assert!(
         dicio.contains_key(&'a') &&
         dicio.contains_key(&'b') &&
         dicio.contains_key(&'c') &&
         dicio.contains_key(&'z') &&
         dicio.contains_key(&'y') 
      );
      // declaração embarca também outros tipos:
      let _dicio = mapa![
         'a': 10,
         'z': 100,
         'b': 1000,
         'y': 10_000,
         'c': 100_000
      ];
      assert!(
         _dicio.contains_key(&'a') &&
         _dicio.contains_key(&'b') &&
         _dicio.contains_key(&'c') &&
         _dicio.contains_key(&'z') &&
         _dicio.contains_key(&'y') 
      );
      let _dicio = mapa! {
         'a': 10,
         'z': 100,
         'b': 1000,
         'y': 10_000,
         'c': 100_000
      };
      assert!(
         _dicio.contains_key(&'a') &&
         _dicio.contains_key(&'b') &&
         _dicio.contains_key(&'c') &&
         _dicio.contains_key(&'z') &&
         _dicio.contains_key(&'y') 
      );
   }
}
