

/* cria um print com impressão
 * programada(em segundos). */


pub struct PrintProgramatico {
   cronometro: Instant,
   periodo: Duration,
   contador: usize,
   ja_feito: bool
}

impl PrintProgramatico {
   // cria scheduler ...
   pub fn novo(periodo: Duration) -> Self {
      return PrintProgramatico {
         cronometro: Instant::now(),
         periodo,
         contador: 0,
         ja_feito: false
      };
   }
   // mostra mensagem:
   pub fn dispara(&mut self, mensagem: &str) {
      let decorrido = self.cronometro.elapsed();
      if decorrido.as_secs() % periodo.as_secs() == 0 {
         if self.ja_feito 
            { continue; }
         println!("\n{}\n", mensagem); 
         self.cronometro = Instant::now();
         self.contador += 1;
         self.ja_feito = true;
      }
      if decorrido.as_secs() % (periodo.as_secs() + 1) == 0 
         { self.ja_feito = false; }
   }
   // obtém total de disparo realizados.
   pub fn total_de_disparos() -> usize
      { self.contador }
}
