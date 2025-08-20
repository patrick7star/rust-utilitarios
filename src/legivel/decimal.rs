#![allow(unused)]
/* cuidado especial com frações de 
 * segundos. A outra função que há no
 * módulo cuida apenas com segundos
 * inteiros. Também traduz o tipo 
 * 'Duration'.
 */
use std::convert::{TryInto};

/// Equivalente de um milhar de segundo.
pub const MILISEG: f64 = 1.0 / 1000.0;
/// Equivalente de um milhão de segundo.
pub const MICROSEG:f64 = 1.0 / 1.0e6;
/// Equivalente de um bilhão de segundo.
pub const NANOSEG: f64 = 1.0 / 1.0e9;
/// Equivalente de um trilhão de segundo.
pub const PICOSEG: f64 = 1.0 / 1.0e12;

/** Mesmo que a função original de legibilidade, porém aplicado neste 
 * caso para frações de segundos.
 ```
 use utilitarios::legivel::tempo_legivel_decimal;

 assert_eq!(
   tempo_legivel_decimal(0.006, true),
   Some(String::from("6.0 ms"))
 );
 assert_eq!(
   tempo_legivel_decimal(0.006, false),
   Some(String::from("6.0 milisegundos"))
 );

 assert_eq!(
   tempo_legivel_decimal(0.00027, false),
   Some(String::from("270.0 microsegundos"))
 );
 ```
*/
pub fn tempo_legivel_decimal<T>(t: T, contracao:bool) -> Option<String>
  where T: Into<f64>, T: From<f32>
{
   // Renomeação da variável a comparar e computar.
   let t = t.into();
   let calculo: f64;
   let sigla: &str;

   if t >= MILISEG && t < 1.0  {
      sigla = if contracao {"ms"} else {"milisegundos" };
      calculo = t * 1_000.0 ;
   } else if t >= MICROSEG && t < MILISEG {
      sigla = if contracao{"μs"} else{"microsegundos"};
      calculo = t * 1_000_000.0;
   } else if t >= NANOSEG && t < MICROSEG {
      sigla = if contracao{"ns"} else{"nanosegundos"};
      calculo = t * 10.0f64.powf(9.0);
   } else if t >= PICOSEG && t < NANOSEG {
      sigla = if contracao{"ps"} else{"picosegundos"};
      calculo = t * 10.0f64.powf(12.0);
   } else 
      { return None; }

   // criando formatação.
   Some(format!("{:0.1} {}", calculo, sigla))
} 


#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn amostras_basicas() {
      let amostras: [f64; 11] = [
         0.00129, 0.010292012, 0.000072,
         0.29, 0.00000031, 0.00000000588,
         0.000000075, 0.000000067, 
         0.0000000000019, 0.000093,
         0.000000000000519,
      ];
      for a in amostras.iter() {
         match tempo_legivel_decimal(*a, true) {
            Some(traducao) =>
               { println!("{} >> {}", a, traducao); }
            None =>
               { println!("não é possível valores como {}", *a); }
         };
      }
   }

   #[test]
   fn amostras_basicas_com_decimal_de_32bits() {
      let amostras = [
         0.00129, 0.010292012, 0.000072,
         0.29_f32, 0.00000031, 0.00000000588,
      ];
      for a in amostras.iter() {
         match tempo_legivel_decimal(*a, true) {
            Some(traducao) =>
               { println!("{} >> {}", a, traducao); }
            None =>
               { println!("não é possível valores como {}", *a); }
         };
      }
   }
}
