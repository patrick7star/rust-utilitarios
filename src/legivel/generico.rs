
/* tenta criar uma função que amplia 
 * a quantidade de tipos de argumentos
 * para vários tipos. */

use super::tempo;
use super::fracao_seg::tempo_fracao;

use std::str::FromStr;
/** retorna uma string contendo o valor legendado
  porém numa faixa mais legível. */
#[allow(unused)]
pub fn tempo_humano<V>(segundos: V, contracao:bool) 
  -> Option<String> where V: ToString  
{
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
                  Some(rst) => rst,
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
   let meio_termo = |s: &str| {
      match s.split_once('.') {
         Some(partes) => {
            e_numero_inteiro(partes.0)
            && e_numero_inteiro(partes.1)
         } None => false
      }
   };
   // obtendo sua forma em string.
   let forma_str = segundos.to_string();
   
   if e_numero_inteiro(&forma_str) {
      /* verifica se o número do parametro é 
       * menor que o máximo permitido(inteiro
       * de 64-bits). */
      let valor_menor = || {
         let maximo = u64::MAX.to_string();
         // se tem mais algarismos do que o valor máximo, então é.
         if maximo.len() < forma_str.len()
            { false }
         /* com comprimentos iguais, verifica se tem qualquer
          * algarismo que seja maior que o equivalente
          * do máximo, qualquer um maior, indica que o valor
          * é maior. */
         else if maximo.len() == forma_str.len() {
            maximo.chars().zip(forma_str.chars()) 
            .any(|tupla| 
               char::to_digit(tupla.0, 10) > 
               char::to_digit(tupla.1, 10)
            )
         } else 
            /* se chegar até aqui provavelmente tem menos
             * algarismos que o do "máximo", portanto é 
             * um valor menor, assim válido. */
            { true }
      };
      if valor_menor() {
         // renomeação da variável a comparar e computar.
         let t  = u64::from_str(&forma_str).unwrap();
         Some(tempo(t, contracao))
      } else 
         { None }
   } else if e_fracao(&forma_str) { 
      let double_str = f64::from_str(&forma_str).unwrap();
      tempo_fracao(double_str, contracao)
   } else if meio_termo(&forma_str) {
      let double_str = f64::from_str(&forma_str).unwrap();
      tempo_fracao(double_str, contracao)
   } else { None }
} 


#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn tempo_generico_varios_tipos() {
      let t = 36u8;
      println!("{} ==> {:?}", t, tempo_humano(t, false));
      let t = 38910_u16;
      println!("{} ==> {:?}", t, tempo_humano(t, true));
      let t = 1_039_842_u32;
      println!("{} ==> {:?}", t, tempo_humano(t, false));
      let t = 30_489_123_918_u64;
      println!("{} ==> {:?}", t, tempo_humano(t, true));
      let t = 99_990_192_152_u64;
      println!("{} ==> {:?}", t, tempo_humano(t, true));
      let t = u128::MAX - u128::MAX/2;
      println!("{} ==> {:?}", t, tempo_humano(t, false));
      assert!(true);
   
   }
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
   #[allow(non_snake_case)]
   fn tempo_em_varias_escalas() {
      let entradasI = [3.8, 0.082, 0.00031, 0.000000006];
      let entradasII = [
         28, 77, 4_981, 96_091, 589_091, 
         2_001_0002, 81_901_001
      ];
      println!("pontos flutuante:");
      for e in entradasI
         { println!("{:?}", tempo_humano(e, true)); }
      println!("inteiros:");
      for e in entradasII
         { println!("{:?}", tempo_humano(e, true)); }
   }
}
