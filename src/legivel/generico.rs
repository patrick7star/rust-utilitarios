
/* tenta criar uma função que amplia 
 * a quantidade de tipos de argumentos
 * para vários tipos. */

use super::tempo_legivel;
use super::decimal::tempo_legivel_decimal;
use std::time::{Duration};

/** Aplica diretamente um duration na conversão. Facilita bastante enquanto
 * codifica, sem falar na legibilidade. Sim, a implementação é tão simples
 * quanto se pensa. Apenas chama a função interna que retorna o `count` de
 * segundos. */
pub fn tempo_legivel_duration(tempo: Duration, contracao: bool) -> String
{ 
   let decimal: f64 = tempo.as_secs_f64();
   let inteiro: u64 = tempo.as_secs();

   if tempo > Duration::from_secs(1) 
      { tempo_legivel(inteiro, contracao) } 
   else { 
      // O 'unwrap' é sempre seguro, pois Durations nunca são inválidos.
      // Oops, parece que a afirmação acima não se comprovou.
      match tempo_legivel_decimal(decimal, contracao)
      {
         Some(result) => result,
         None => format!("Inválido{:?}", tempo)
      }
   }
}


#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   #[ignore="só para visualizar o valor máximo(todos seus algarismos)"]
   fn visualiza_u64max() 
      { println!("{}", u64::MAX); }
   #[test]
   fn closure_e_numero_inteiro() {
      let e_numero_inteiro =  |s: String| {
         s.chars()
         .all(|ch| ch.is_ascii_digit())
      };
      let entradas = [
         "1029382", "000012938", "99132",
         "1293842.2","192384i3012", "38",
         "41i", "1", "m", "999", "999.999"
      ];
      let saidas = [
         true, true, true, false, false,
         true, false, true, false, true, false
      ];
      for (e, s) in entradas.iter().zip(saidas.iter())
         { assert_eq!(e_numero_inteiro(e.to_string()), *s); }
   }
   #[test]
   fn closure_e_fracao() {
      let e_numero_inteiro = |string: &str| {
         string.chars()
         .all(|ch| ch.is_ascii_digit())
      };
      let apenas_zeros = |s: &str| s.chars().all(|c| c == '0');
      let e_fracao = |s: &str| {
         let restante = {
            match s.find('.') {
               Some(p) => {
                  match s.get(p+1..) {
                     Some(rst) => dbg!(rst),
                     None => 
                        { return false; }
                  }
               } None =>
                  { return false; }
            }
         };
         s.starts_with("0.") && 
         e_numero_inteiro(restante)
         && !apenas_zeros(restante)
      };
      let entradas = [
         "0.012398", "0.38", "1.0382",
         "0000.61832","2.232", "0.000021",
         "0.000000", "0.0002", "0000.0081",
         "321.123", "0.123.12"
      ];
      let saidas = [
         true, true, false, false, false,
         true, false, true, false, false,
         false
      ];
      for (e, s) in entradas.iter().zip(saidas.iter())
         { assert_eq!(e_fracao(dbg!(e)), dbg!(*s)); }
   }
   #[test]
   fn legibilidade_de_durations() {
      let amostras = [
         Duration::from_secs_f32(3701.1234),
         Duration::from_secs(100_023),
         Duration::from_millis(5_398_001),
         Duration::from_secs(303),
         Duration::from_millis(303),
         Duration::from_micros(29),
         Duration::from_nanos(702),
      ];

      for item in amostras
         { println!("{}", tempo_legivel_duration(item, true)); }
   }
}
