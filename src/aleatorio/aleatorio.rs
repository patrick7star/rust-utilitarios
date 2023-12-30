/*!  
 # Geração de números aleatórios
  Neste código vamos apresentar algumas funções
 que geram, números inteiros e flutuantes, 
 arrays inteiras e intervalos de números
 aleatórios.

  O algoritmo é simples, pega o clock ou 
 tempo no exato momento de execução, como geralmente
 é um número na casa das dezenas de milhares, 
 obtem seu último dígito, que vária muito, e
 soma com outros mais cem números obtidos pelo
 mesmo processo, e, novamente arranca-se o
 valor unitário do número, que sempre varia
 de zero à dez. Em cima desta função que dá
 valores de 0 à 9 de forma randomizada, se
 constrói todo o ferramental de aleatoriaedade.
 
  Tais funções não foram feito para serem gerados
 em grandes escalas randomizadas, apenas um 
 em escalas de milisegundos a geração massissa
 estocástica funciona bem, portanto, um algoritmo
 extremamente lento.
*/

// complementar:
mod metodo_i;
mod metodo_ii;
mod impressao;

// complementar, porém é uma gama de testes:
mod testes_unitarios;

// re-exportando ...
pub use metodo_ii::sortear;
// #[cfg(target_os="linux")]
pub use metodo_i::randomico;

// troca valores dentro de uma array genérica.
fn troca<A>(lista: &mut Vec<A>, p1: usize, p2:usize) {
   let remocao = lista.remove(p1);
   lista.insert(p2, remocao);
} 

/// embaralha a array referênciada.
pub fn embaralha<B>(array: &mut Vec<B>) {
   let mut tamanho = array.len();
   let ultimo: u64 = (tamanho - 1) as u64;

   // se houver apenas dois elementos, pode trocar ou não.
   if tamanho == 2 {
      if sortear::bool() 
         { troca(array, 0, 1); }
   } else if tamanho <= 1 {
      // apenas abandona o programa; nada a fazer.
      return ();
   } else {
      // faz o embaralho o "tamanho da lista" vezes.
      while tamanho > 0 {
         let i = sortear::u64(0..=ultimo) as usize;
         let j = sortear::u64(0..=ultimo) as usize;
         if j != i 
            { troca(array, i, j); }
         tamanho -= 1;
      }
   }
}

pub fn escolhe<C, S>(colecao: S, tamanho: usize) -> Option<C>
  where S: IntoIterator<Item=C> 
{
   let mut colecao_iter = colecao.into_iter();
   if tamanho == 0
      { return None; }
   
   let escolha ={
      if cfg!(unix) {
         randomico::usize(0..=(tamanho-1))
      } else {
         sortear::usize(0..=(tamanho - 1))
      }
   };
   colecao_iter.nth(escolha)
} 

/* O mesmo que acima, mas com dois "porém's". O
 * primeiro é que sua complexidade algoritímica 
 * é O(n), já que despeja todo o conteúdo numa 
 * nova array; a outra, que é uma vantagem, não 
 * necessita do parametro 'tamanho' da coleção. */
#[allow(non_snake_case)]
pub fn escolheI<C, D>(colecao: C) -> Option<D>
  where C: IntoIterator<Item=D>
{
   let mut array: Vec<D>;
   array = colecao.into_iter().collect();
   let tamanho = array.len();
   if tamanho == 0
      { return None; }
   let ultimo = tamanho - 1;
   /* escolhendo o índice nela, e removendo na mesma
    * array o índice desejado. */
   Some(array.swap_remove(randomico::usize(0..=ultimo)))
}

/* Quase o mesmo que o acima, porém exige a slice
 * do objeto para fazer a seleção entre. Sem falar
 * que, retorna a referência de tal, ao invés de 
 * o elemento em sí. Entretanto, possívelmente só
 * funciona com alguns 'collections'. */
#[allow(non_snake_case)]
pub fn escolheII<'s, D>(fatia: &'s [D]) -> Option<&'s D> {
   let tamanho = fatia.len();
   let ultimo = tamanho - 1;
   fatia.get(randomico::usize(0..=ultimo))
}

type Intervalo = RangeInclusive<u8>;
use std::ops::RangeInclusive;
/** cria uma string ascii de forma aleatória. */
pub fn string_ascii(code: Intervalo, comprimento: usize) 
  -> Result<String, &'static str>
{
   let a = code.start();
   let b = code.end();
   if *a < 33 || *a >= 127 || *b >= 127
      { return Err("não é um caractére válido!"); }
   let mut string = String::with_capacity(comprimento);
   for _ in 1..=comprimento {
      let intervalo = code.clone();
      let codigo = sortear::u8(intervalo);
      match char::from_u32(codigo as u32) {
         Some(caractere) =>
            { string.push(caractere); }
         None =>
            { return Err("código inválido!"); }
      };
   }
   Ok(string)
}

// que tipo de letra deseja-se.
#[derive(Debug)]
pub enum Modo {
   Maiuscula,
   Minuscula,
   Ambos
}

#[derive(Debug)]
pub enum Classe { 
   Alfabeto(Modo), 
   Numerico, 
   Pontuacao,
   // à implementar:
   Acentuacao,
   Pares,
   Matematico
}


use std::collections::BTreeSet;
use std::iter::FromIterator;
// conjunto com inteiros positivos.
type Codigos = BTreeSet<u8>;

fn string_do_conjunto(codigos: Codigos, tamanho: usize) -> String {
   let mut string = String::with_capacity(tamanho);
   let total = codigos.len();
   /* colocado tudo numa array para que se possa
    * selecionar seus índices. */
   let linear = codigos.iter().collect::<Vec<&u8>>();

   // encher até alcançar o demandado.
   while string.len() < tamanho {
      let indice = randomico::usize(0..=(total-1));
      let codigo: u32 = (*linear[indice]).into();
      let caractere = char::from_u32(codigo).unwrap();
      string.push(caractere);
   }

   // retornando a construção randômica.
   string
}

pub fn string_classe(tipo: Classe, comprimento: usize)
  -> Result<String, &'static str>
{
   match tipo {
      Classe::Alfabeto(qual_tipo_de_string) => {
         match qual_tipo_de_string {
            Modo::Maiuscula => 
               { string_ascii(0x41..=0x5A, comprimento) }
            Modo::Minuscula => 
               { string_ascii(0x61..=0x7A, comprimento) }
            Modo::Ambos => {
               let tipo_alfabetico: Classe;
               // tipo de 'cap' no cara ou coroa.
               if sortear::bool()
                  { tipo_alfabetico = Classe::Alfabeto(Modo::Minuscula); }
               else
                  { tipo_alfabetico = Classe::Alfabeto(Modo::Maiuscula); }
               /* chama função recursivamente, porém agora
                * como os argumentos bem definidos. */
               string_classe(tipo_alfabetico, comprimento)
            }
         } 
      } 
      Classe::Numerico =>
         { string_ascii(0x30..=0x39, comprimento) }
      Classe::Pontuacao => {
         let codes = Codigos::from_iter(vec![
            0x21, 0x22, 0x27, 0x2E,
            0x3A, 0x3B, 0x3F, 0x5F
         ].drain(..));
         Ok(string_do_conjunto(codes, comprimento))
      }
      #[allow(unreachable_patterns)]
      _ => unimplemented!()
   }
}

#[cfg(test)]
mod tests {
   use super::{string_classe, Classe, Modo};

   #[test]
   #[should_panic]
   fn intervalo_passado_errado_u16() {
      /* argumento incorreto, pois na base
       * matemática o inteiro -13 é menor
       * que -5, logo o programa será 
       * interrompido! */
      super::sortear::i16(-5..=-13);
   }

   #[test]
   fn testa_embaralha() {
      let mut array = vec![1,2,3,4,5];
      let copia = array.clone();
      let mut p = 0.0;
      for _ in 1..=100 {
         super::embaralha(&mut array);
         println!("{:?}", array);
         if array != copia
            { p += 1.0/100.0; }
      }
      assert!(dbg!(p) > 0.95);
   }

   use std::thread::{JoinHandle, spawn};
   #[test]
   fn e_multi_thread() {
      let mut selecoes: Vec<u8> = Vec::new();
      let mut outra_selecoes: Vec<u8> = Vec::new();

      let thread_i: JoinHandle<Vec<u8>>;
      thread_i = spawn(move || {
         for _ in 1..=30_000 
            { outra_selecoes.push(super::sortear::u8(0..=100)); }
         outra_selecoes
      });

      for _ in 1..=30_000 
         { selecoes.push(super::sortear::u8(0..=100)); }

      match thread_i.join() {
         Ok(resultado) => {
            let iterador = selecoes.iter().zip(resultado);
            for (a, b) in iterador.rev().take(30)
               { println!("{} <==> {}", a, b); }
         } Err(_) => ()
      };

      assert!(false);
   }

   #[test]
   fn string_ascii_aleatoria_amostra() {
      for _ in 1..=12 { 
         let codigo_aleatorio = super::string_ascii(35..=113, 15);
         println!("==> {}", codigo_aleatorio.unwrap() ); 
      }
   }

   #[test]
   fn string_classe_alfabeto() {
      let tipo1 = Classe::Alfabeto(Modo::Maiuscula);
      let tipo2 = Classe::Alfabeto(Modo::Minuscula);
      let s = string_classe(tipo1, 45).unwrap();
      let r =  string_classe(tipo2, 37).unwrap();
      assert_eq!(s.len(), 45);
      assert_eq!(r.len(), 37);
      println!("==> {}\n==> {}", s, r); 
   }

   #[test]
   fn string_especificadas_tres_primeiras() {
      let entradas = vec![
         string_classe(Classe::Alfabeto(Modo::Maiuscula), 45),
         string_classe(Classe::Alfabeto(Modo::Minuscula), 41),
         string_classe(Classe::Numerico, 26),
         string_classe(Classe::Pontuacao, 26),
      ];
      for entry in entradas.iter()
         {  println!("==> {}", entry.as_ref().unwrap() ); }
      
      assert!(
         entradas[0].as_ref().unwrap().chars().all(
         |c| c.is_ascii_alphabetic() 
         && c.is_ascii_uppercase()
      ));
      assert!(
         entradas[1].as_ref().unwrap().chars().all(
         |c| c.is_ascii_alphabetic() 
         && c.is_ascii_lowercase()
      ));
      assert!(
         entradas[2].as_ref().unwrap().chars()
         .all(|c| c.is_ascii_digit())
      );
      assert!(
         entradas[3].as_ref().unwrap().chars()
         .all(|c| c.is_ascii_punctuation())
      );
   }

   #[test]
   fn escolhe_com_vetor() {
      let entrada = vec!['a', 'x', 'p'];
      let t = entrada.len();
      let saida = super::escolhe(entrada, t);
      debug_assert!(
         saida == Some('a') ||
         saida == Some('x') ||
         saida == Some('p'),
         "saida = {:?}", saida
      );
   }

   use std::collections::HashSet;
   use std::iter::FromIterator;
   #[test]
   fn escolhe_com_hashset() {
      let mut amostra = vec![
        'A', 'j', 'M', 'b',
        'K', 'l', 'V', 'o'
      ];
      let entrada = HashSet::<char>::from_iter(amostra.drain(..));
      let t = entrada.len();
      let funcao = super::escolhe;
      assert!(
         (1..=100)
         .map(|_| funcao(entrada.clone(), t))
         .any(|saida| 
            saida == Some('l') ||
            saida == Some('M') ||
            saida == Some('b')
         )
      );
   }
}
