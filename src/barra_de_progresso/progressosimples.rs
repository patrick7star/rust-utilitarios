

/**!
 O progresso simples que há, que trabalha
 em 'tamanho dos dados' e 'quantia total',
 sem falar que retorna muitas strings por
 microsegundo, também há algumas que 
 mostram os rótulos. Vamos mesclar tudo
 numa única função.
 */

use super::{
   progresso, progresso_data, 
   ProgressoPercentual, Logo
};
use std::fmt::{
   Display, Formatter, 
   Result as Result_fmt
};
use std::ops::AddAssign;

// da estrura codificada aqui:
/// Atalho para `ProgressoSimples`.
pub type PS<'a> = ProgressoSimples<'a>;
/// Atalho para `ProgressoPercentual`.
pub type PP = ProgressoPercentual;

/** 
 legenda:
   Dado - é imformação baseado no tamanho
           atual dos dados, e quanto já foi
           "consumido".
    Quantia - é a forma total e atual como
             foi dada, e está sendo acumulada.
    Detalhe - consiste numa informação baseado
            na mensagem cedida(uma string), com
            um mesclado de percentual com 'quantia
            pura'. 
 */
pub enum Formato { Dado, Quantia, Detalhe }

/** 
 progresso de dados mais para saídas
 simples, com argumentos opcionais,
 que depedendo da sua inclusão habilitam
 ou não configurações.
 Portanto, tal estrutura na verdade não
 é simples em opções adicionais e saídas,
 simples apenas que simula a primeira 
 barra de progresso feita.
 */
pub struct ProgressoSimples<'b> {
   // rótulo de uma string a girá, é opcional.
   rotulo: Option<Logo<'b>>,
   /* usa o progresso com auxílio, sua saída
    * será reescrita, mas ele tem toda estrutura
    * para simulação deste, sem falar que este,
    * também teria uma 'saída' baseado no
    * percentual. */
   progresso_auxiliar: PP,
   // registradores do "percentual".
   total: u64, atual: u64,
   /* como será mostrados, em dados, quantias
    * ou informação detalhada. */
   tipo: Formato 
   
}

impl <'b>ProgressoSimples<'b> {
   // método auxiliar.
   fn percentual(&self) -> f32 
      { (self.atual as f32) / (self.total as f32) }

   /* método que dá impressão da string se
    * for necessário. */
   pub fn imprime(&mut self) -> Option<String> {
      /* segue a frequência da barra de progresso
       * interna a estrutura. */
      match self.progresso_auxiliar.imprime() {
         Some(_) => Some(self.to_string()),
         None => None
      }
   }

   // método construtor.
   pub fn novo(total: u64, rotulo: Option<Logo<'b>>, tipo: Formato)
   -> ProgressoSimples<'b> {
      let progresso = PP::cria(total);
      Self { 
         total, rotulo, tipo, atual: 0, 
         progresso_auxiliar: progresso 
      }
   }

   /* diz estado do PS, porém apenas usa
    * o progresso interno(o PP) como
    * referência. */
   pub fn esgotado(&self) -> bool 
      { self.progresso_auxiliar.esgotado }
}

impl AddAssign<u64> for PS<'_> {
   // atualização do progresso da barra.
   fn add_assign(&mut self, valor:u64) { 
      if !self.progresso_auxiliar.esgotado {
         // atualiza valor ao invés de incrementar.
         self.atual = valor;
         /* apesar de parecer incrementar, está
          * apenas atualizando o valor. Que é o
          * que foi implementado para esta
          * estrutura também.  */
         self.progresso_auxiliar += valor;
         /* têm seu ciclo próprio de movimento. */
         match &mut self.rotulo {
            Some(logo) => {
               { logo.movimenta_letreiro(); }
            } None => ()
         };
      }
   }
}

impl Display for PS<'_> {
   // implementando visualização em sí.
   fn fmt(&self, formatador:&mut Formatter<'_>) 
   -> Result_fmt {
      // nomeando por legibilidade.
      let qa = self.atual;
      let qt = self.total;
      // obtendo 'String' baseado no atual/total.
      let barra_de_progresso = {
         match self.tipo {
            Formato::Quantia => progresso(qa, qt),
            Formato::Dado => progresso_data(qa, qt),
            Formato::Detalhe => format!(
               "já está em {}, {:0.1}%", 
               self.atual,
               self.percentual() * 100.0
            )
         }
      };
      // "imprimindo" visualização da atual barra.
      match &self.rotulo {
         Some(rotulo) =>  
            write!(
               formatador, "{{{}}} {}", 
               rotulo,
               barra_de_progresso
            ),
         None =>
            write!(formatador, "{}", barra_de_progresso)
      }
   }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;
   use std::thread;
   use std::time::{Instant, Duration};
   use crate::legivel::tamanho;

   #[test]
   fn criandoInstanciaPS() {
      /* mesmo tamanhos, e sem rótulo, porém
       * escolhendo formatações diferentes. */
      let mut instancias = [
         PS::novo(30, None, Formato::Dado),
         PS::novo(30, None, Formato::Detalhe),
         PS::novo(30, None, Formato::Quantia)
      ];
      for instancia in instancias.iter_mut() {
         println!("{}", instancia);
         *instancia += 5;
         println!("{}", instancia);
         *instancia += 14;
         println!("{}", instancia);
         *instancia += 26;
         println!("{}", instancia);
         assert!(!instancia.esgotado())
      }
      /* total diferente, más não é o importante
       * aqui. Aqui, o que importa é o rótulo que
       * foi incluso.  */
      let mensagem = concat!(
         "banco-de-dados da Instituição ",
         "Federal-de-Educação de São Gonçales ",
         "do Pernambuco"
      );
      // rótulo dinâmico.
      let r = Logo::novo(mensagem).unwrap();
      let total: u64 = 30;
      let mut instancias = vec![
         PS::novo(total, Some(r.clone()), Formato::Quantia),
         PS::novo(total, Some(r.clone()), Formato::Detalhe),
         PS::novo(total, Some(r), Formato::Dado),
      ];
      for mut instancia in instancias.drain(..) {
         for k in 1..=total {
            assert!(!instancia.esgotado());
            match instancia.imprime() {
               Some(conteudo) =>
                  { println!("{}", conteudo); }
               None => (),
            };
            instancia += k;
            thread::sleep(Duration::from_millis(300));
         }
         println!("\n{}\n\n", instancia);
         assert!(instancia.esgotado());
      }
   }

   type Saidas = Vec<String>;

   #[test]
   fn metodoImprimeEconomicoEmMemoria() {
      let mut P = PS::novo(80_000, None, Formato::Dado);
      let mut saidaI = Saidas::with_capacity(300);
      let mut saidaII = Saidas::with_capacity(80_001);

      for novo in 1..=80_000 {
         if let Some(conteudo) = P.imprime() 
            { saidaI.push(conteudo); }
         saidaII.push(P.to_string());
         P += novo;
      }
      assert!(P.esgotado());
      assert_ne!(dbg!(saidaI.len()), dbg!(saidaII.len()));

      fn size(objeto: &Saidas) -> usize 
         { objeto.iter().map(|s| s.len()).sum() }

      let t1 = size(&saidaI);
      let t2 = size(&saidaII);
      println!(
         "fator de diminução: x{}
         \reconômia de memória: {}",
         (t2/t1), tamanho((t2 - t1) as u64, true)
      );
   }
   #[test]
   fn metodoImprimeEconomiaEmCPU() {
      let mut saida = Saidas::with_capacity(80_001);
      for t in [1_000, 2_000, 10_000, 20_000, 80_000] {
         let mut cronometro = Instant::now();
         let mut P = PS::novo(t, None, Formato::Quantia);
         for novo in 1..=t {
            if let Some(conteudo) = P.imprime() 
               { saida.push(conteudo); }
            P += novo;
         }
         let Ta = cronometro.elapsed();
         assert!(P.esgotado());
         // limpando para o próximo.
         saida.clear();
         // agora o método via impressão contínua.
         let P1 = PS::novo(t, None, Formato::Quantia);
         cronometro = Instant::now();
         for _ in 1..=t
            { saida.push(P1.to_string()); }
         saida.push(P1.to_string());
         let Tb = cronometro.elapsed();
         println!(
            "fator de tempo(n={1}): {0}", 
            Tb.as_nanos() / Ta.as_nanos(), t
         );
         assert!(Ta < Tb);
      }
   }
}
