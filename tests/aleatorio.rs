
/*
 * Vamos testar ao máximo que poder como
 * se distribui as amostra de grandes 
 * quantias de valores gerados aleatóriamente
 * para ver se está tudo ok!
*/

extern crate utilitarios;
use utilitarios::aleatorio;

#[test]
fn distribuicoes_iguais_do_u8() {
   // amostras aleatórias:
   let mut vinte = 0;
   let mut duzentos_e_trinta_e_dois = 0;
   let mut sessenta_e_sete = 0;

   // total de lançamentos.
   let total = 100_000;
   for _ in 1..=total {
      // sorteio de 0 à 255.
      let numero = aleatorio::sortear::u8(0..=255);

      // contando eventos ...
      if numero == 20 
         { vinte += 1; }
      else if numero == 232
         { duzentos_e_trinta_e_dois += 1; }
      else if numero == 67
         { sessenta_e_sete += 1; }
   }
   
   /* porcentagens de todos tem que alcançar a
    * probabilidade global. */
   let pg:f32 = 1f32 / 256_f32;
   let p20 = (vinte as f32)/(total as f32);
   let p232 = (duzentos_e_trinta_e_dois as f32)/(total as f32);
   let p67 = (sessenta_e_sete as f32)/(total as f32);

   // margem de erro têm que estar em 20%
   let margem = dbg!((pg-p20).abs());
   assert!(margem <= 0.20);
   let margem = dbg!((pg-p232).abs());
   assert!(margem <= 0.20);
   let margem = dbg!((pg-p67).abs());
   assert!(margem.abs() <= 0.20);

   // margem de erro mais estreita: 5%.
   let margem = dbg!((pg-p20).abs());
   assert!(margem <= 0.05);
   let margem = dbg!((pg-p232).abs());
   assert!(margem <= 0.05);
   let margem = dbg!((pg-p67).abs());
   assert!(margem.abs() <= 0.05);

   // margem de$erro AINDA mais estreita: 1%.
   let margem = dbg!((pg-p20).abs());
   assert!(margem <= 0.01);
   let margem = dbg!((pg-p232).abs());
   assert!(margem <= 0.01);
   let margem = dbg!((pg-p67).abs());
   assert!(margem.abs() <= 0.01);
}

#[test]
fn distribuicoes_iguais_do_i8() {
   // amostras aleatórias:
   let mut oitenta_e_oito = 0;
   let mut menos_quinze = 0;
   let mut cento_e_dois = 0;
   let mut menos_trinta_e_sete = 0;

   // total de lançamentos.
   let total = 100_000;
   for _ in 1..=total {
      // sorteio de 0 à 255.
      let numero = aleatorio::sortear::i8(-128..=127);

      // contando eventos ...
      if numero == 88 
         { oitenta_e_oito += 1; }
      else if numero == -15
         { menos_quinze += 1; }
      else if numero == 102
         { cento_e_dois += 1; }
      else if numero == -37 
         { menos_trinta_e_sete += 1; }
   }
   
   /* porcentagens de todos tem que alcançar a
    * probabilidade global. */
   let pg:f32 = 1f32 / 256_f32;
   let p1 = (oitenta_e_oito as f32)/(total as f32);
   let p2 = (menos_quinze as f32)/(total as f32);
   let p3 = (cento_e_dois as f32)/(total as f32);
   let p4 = (menos_trinta_e_sete as f32)/(total as f32);

   // margem de erro têm que estar em 20%
   for erro in [0.20, 0.05, 0.01, 0.005] {
      let margem = dbg!((pg-p1).abs());
      assert!(margem <= erro);
      let margem = dbg!((pg-p2).abs());
      assert!(margem <= erro);
      let margem = dbg!((pg-p3).abs());
      assert!(margem.abs() <= erro);
      let margem = dbg!((pg-p4).abs());
      assert!(margem.abs() <= erro);
   }
}
