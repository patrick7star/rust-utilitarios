/*!
 # O que são estes Utilitários
   Todos códigos que não tiverem elaborações bem complexas, ou seus esboços
 iniciais e simples, porém bem úteis ficarão aqui. Isto é muito melhor que
 ao invés de criar um `crate` para cada um. Códigos que são também repetidos por várias `libs`, serão colocadas aqui, por motivos óbvios. Como disse
 antes tais funções e estruturas de cada módulo executam coisas muitos
 simples.

   Outra razão para termos isso aqui é, um jeito de burlar a internet e
 pacotes de terceiros. Toda vez que são compilados *caixotes* do **Rust**,
 aqueles que depedem de pacotes teceiros, se não houver *artefatos* no
 computador, então procuram na internet, se não houver qualquer conexão,
 você não conseguirá compilar, consequentemente, executar o programa; por
 isso é importante uma biblioteca que dependa menos e menos de internet, ou
 quase nada. Também ficam implementações do ***autor***, que não são para
 ser um substituto, mas uma tentativa de implementação própria, seja para
 aprendizado, ou apenas diversão, e apromimoramento de novas técnicas
 recentemente aprendidas da **linguagem Rust**, ou de computação em geral.
*/


/** Dado um diretório desenha uma árvore em string baseando nos arquivos e
 * diretórios do atual(raíz) e seus subdirs. */
pub mod arvore;

/** Talvez o mais complexo do pacote, cria uma estrura para manipular de
 * forma maleável a impressão de texto e desenhos simples no terminal. */
pub mod tela;

/** Converte valores inteiros e fluantes, que representam grandezas
 * importantes em computação, para valores legíveis, tais na formatação de
 * strings.
 */
pub mod legivel;

/** Pega lista e arrays de dados e faz uma tabela delas para simples
  * impressões.
  */
pub mod tabelas;

/** Gerador de números pseudo-aleatórios. Todos tipos de dados inteiros
 * estão inclusos no pacote, inclusive não-inteiros como valores de
 * "decimais".
 */
pub mod aleatorio;

/** Transforma strings e inteiros que representam valores decimais/ou
 * binários para números romanos, o inverso também, ou seja, romanos para
 * números decimais/inteiros. */
pub mod romanos;

/// Incrementa vários modos de impressão na tela.
pub mod impressao;

/** Variádos tipos de barras de progressos, que informam de forma dinâmica
 * como a computabilidade de tais dados abordados está indo. */
pub mod progresso;

/** Obtém a dimensão do terminal que roda tal lib. Portanto agora, podemos
 * excluir a biblioteca externa utilizada para fazer tal, substituindo por
 * esta. */
pub mod terminal;

/// escreve um número dado por extenso.
pub mod porextenso;


/** Macro importante para agilizar na criação
 de um Mapa(HashMap). A forma que tem que ser
 escrito é, dado dois tipos, lado esquerdo
 fica a chave, lado direito o valor. Até que
 se saiba, funciona com quase todos tipos
 primitivos.
 ```
 let dicio_1 = mapa! { 'a': 97,'b': 98,'c': 99 };
 let mut dicio_2 = HashMap::<char, usize> = HashMap::new();

 dicio_2.insert(&'a', 97);
 dicio_2.insert(&'b', 98);
 dicio_2.insert(&'c', 99);

 assert_eq!(dicio_1, dicio2);
 ```
 */
#[macro_export]
macro_rules! mapa {
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
   fn converte_romano() {
      // número romano digitado.
      let nr = super::terminal::lanca_prompt("digite um número romano");
      // conversão para decimal.
      let nd = super::romanos::romano_para_decimal(nr.as_str());
      // decimal esperado.
      let de = terminal::lanca_prompt("qual o decimal esperado");
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
