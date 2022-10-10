

/* testes unitários serão passados para
 * aqui, pois estão tomando muito espaço
 * no arquivo 'metodo_ii'.
 */

use std::collections::HashSet;
use super::metodo_i::randomico;
use super::impressao::*;
use std::time::Duration;

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   #[allow(non_snake_case)]
   fn GeraTodos_u16() {
      // conjunto com todos inteirios.
      let mut universo: HashSet<u16>;
      universo = HashSet::new();
      // conta quantia de chamadas.
      let mut contador = 0;
      let mut instancia = PP::novo(Duration::from_secs(1));
      let total = (2_i32.pow(16)-1) as usize;
      // sortea até pega todos inteiros possíveis do tipo.
      while universo.len() != total {
         let sorteio = randomico::u16();
         universo.insert(sorteio);
         contador += 1;
         // informação:
         let mensagem = format!(
            "adições até o momento: {}
            \ratual número: {}
            \rchamadas: {}",
            universo.len(), sorteio,
            contador
         );
         instancia.dispara(mensagem.as_str());
      }
      assert!(true);
   }

}
