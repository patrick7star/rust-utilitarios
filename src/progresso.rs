/*!
 # Progressos para todas situações
 Código que dá utilitário que retorna barra de progresso, dados uma quantia
 atual e total a ser atingida.
*/

// Todos submódulos deste:
mod letreiro;
// mod unitarios;
mod rotulo;
mod barra;

// Módulos do próprio projeto:
use crate::legivel::{tamanho_legivel};
// Submódulos deste módulo:
pub use letreiro::*;
use barra::{
   cria_barra_ascii_sem_cor, progresso_ascii_sem_cor,
   progresso_unicode_e_colorido, CAPACIDADE
};
// Biblioteca do Rust:
use std::time::{Duration, Instant};
use std::fmt::{Display, Formatter, Result as Result_fmt};
use std::ops::{AddAssign};

// Exportando dos submódulos:
pub use rotulo::*;

// Apelidos pra estruturas definidas abaixo, assim como saídas específicas:
pub type PT = ProgressoTemporal;
type Impressao = Option<String>;

/**  Dita mais ou menos o que um progresso tem como funcionalidades. Alguns
 * funcionalidades básicas como: o método construtor obviamente, se o progresso se *esgotou*, ou o retorno
 * do decimal. */
pub trait ProgressoComando
  where Self: AddAssign<usize> + AddAssign<f32> 
   + Sized + Display
{
   /// Nova instância apenas com o valor *total* a ser alcançado, partindo 
   /// do zero obviamente.
   fn novo(total: usize) -> Self;

   /// Verifica se o progresso foi finalizado com sucesso.
   fn terminado(&self) -> bool;

   /// Retorno o percentual que se encontra tal **progresso**.
   fn percentual(&self) -> f32;

   /// Retorna uma possível `string` com a formatação do progresso.
   fn imprime(&mut self) -> Impressao;
}

/** Nome de quase todos(pra alguns, a tradução simplesmente não existe ou não
 * vale à pena) os métodos acima, porém em Inglês. */
pub trait Traducao: ProgressoComando {
   /** New instance, that accept only a value *total* this have to be reached
    * in progress, starting with zero. */
   fn new(total: usize) -> Self;

   /// Verify if *progress* defined and executed, it is finished.
   fn finished(&self) -> bool;

   /// May return a `String` where represents a formatting of progress-bar.
   fn print(&mut self) -> Impressao;
}



/** Cálculos percentuais e, suas representação numérica. Mesmo que o acima, 
 * o consumo de `CPU` é dispedício, não use em **loops**. */
#[deprecated(since="1.3.5",
note="usado agora apenas internamente")]
pub fn progresso_data(qtd_atual:u64, qtd_total:u64) -> String {
   let percentagem:f32 = (qtd_atual as f32)/(qtd_total as f32);
   let percent_100:f32 = percentagem*100.0;

   // caso de erro.
   if percentagem > 1.0_f32
      { panic!("os valores de atual supera o total!"); }
   else if percentagem == 1.00 {
      let total_bytes = tamanho_legivel(qtd_total, true);
      return format!(
         "{}/{} [{}] {}%\n",
         total_bytes,
         total_bytes,
         cria_barra_ascii_sem_cor(1.0, CAPACIDADE),
         100.0
      );
   }
   else {
      // strings dos valores.
      let qa = tamanho_legivel(qtd_atual, true);
      let qt = tamanho_legivel(qtd_total, true);
      let qtd_algs = qt.len() as usize;
      let molde:String = format!(
         "{0:>espaco$}/{1} [{2}]{3:>5.1}%",
         qa, qt, cria_barra_ascii_sem_cor(percentagem, CAPACIDADE),
         percent_100,espaco=qtd_algs
      );
      return molde;
   }
}

/** Progresso com rótulo dinâmico, então quando o rótulo é muito maior do 
 * que cabe na tela, ele move o rótulo tipo aqueles slogans de neon em 
 * mercearias e shops. Ele também é de dados, o resto do seu núcleo. */
#[deprecated(
  since="1.3.5",
  note="confuso, e raramente utilizado"
)]
pub fn progresso_data_rotulo<'a>(rotulo:&'a str, qtd_atual:u64, 
  qtd_total:u64) -> String
{
   // cálculando a porcetagem.
   let percentagem:f32 = (qtd_atual as f32)/(qtd_total as f32);
   // caso de erro.
   if percentagem > 1.0_f32 {
      panic!("os valores de atual supera o total!");
   }
   else if percentagem == 1f32 {
      let total_bytes = tamanho_legivel(qtd_total, true);
      return format!(
         "[{}] {}/{} [{}] {}%\n",
         rotulo,
         total_bytes,
         total_bytes,
         cria_barra_ascii_sem_cor(1.0, CAPACIDADE),
         100.0
      );
   }
   else {
      // strings dos valores.
      let qa = tamanho_legivel(qtd_atual, true);
      let qt = tamanho_legivel(qtd_total, true);
      let qtd_algs = qt.len() as usize;
      let molde:String = format!(
         "[{4}] {0:>espaco$}/{1} [{2}]{3:>5.1}%",
         qa, qt, cria_barra_ascii_sem_cor(percentagem, CAPACIDADE),
         percentagem*100.0,
         rotulo,
         espaco=qtd_algs,
      );
      return molde;
   }
}

/* == == == == == == == == == == == == == == == == == == == == == == == == =
 *                   Progresso Baseado no Percentual
 * == == == == == == == == == == == == == == == == == == == == == == == = */
/** Um tipo de progresso que leva em conta, na hora de imprimir, a quantia 
 de progresso ocorrido, e não apenas dá uma impressão dados a *quantia atual*
 e *total*. Tal estrutura é perfeita para uma grande entrada de dados. */
pub struct ProgressoPercentual {
   // quantidade atual passada.
   qtd_atual: usize,
   // o total inicialmente marcado.
   qtd_total: usize,
   // antigo percentual para comparação.
   antigo_percentual: f32,
   /* Públicos apenas para o acesso em computação. Ainda ver como fazer para
    * que não se altere. */
   // status da barra:
   esgotado: bool,
   // percentual computado.
   percentual: f32,
}

impl ProgressoPercentual {
   pub fn cria(total: usize) -> Self {
      ProgressoPercentual {
         qtd_atual: 0,
         qtd_total: total,
         percentual: 0.0f32,
         antigo_percentual: 0.0f32,
         esgotado: false
      }
   }

   pub fn imprime(&mut self) -> Impressao {
      // diferença de percentuais.
      let diferenca = self.percentual - self.antigo_percentual;
      let progresso_ii = progresso_unicode_e_colorido;

      // verifica se há uma variação de
      // no mínimo 0,5%.
      if diferenca >= 0.005 {
         // atualiza o 'antigo percentual' com o valor do "atual".
         self.antigo_percentual = self.percentual;
         // criando barra...
         let bp = progresso_ii(self.qtd_atual, self.qtd_total);
         // retornando barra, porém embrulhado.
         return Some(bp);
      } else if self.qtd_atual == 0 {
         // não chamada nenhuma vez?
         let bp = progresso_ii(self.qtd_atual, self.qtd_total);
         return Some(bp);
      } else if self.esgotado {
         Some(progresso_ii(self.qtd_atual, self.qtd_total))
      } else { None }
   }
}

/* Agora com o atribuição, faz a mesma coisa do que o acima, porém sem 
 * retorno. No 'background' é atualização de valor. */
impl AddAssign<usize> for ProgressoPercentual {
   // implementação da atualização de dados.
   fn add_assign(&mut self, valor: usize) {
      // atualizando valores...
      if valor >= self.qtd_total as usize {
         self.esgotado = true;
         self.qtd_atual = self.qtd_total;
      } else
         { self.qtd_atual = valor; }

      // computando novo percentual.
      self.percentual = {
         let x:f32 = self.qtd_atual as f32;
         let y:f32 = self.qtd_total as f32;
         if x > y { 1.0f32 }
         else { x / y }
      };
   }
}

/* Visualização do atual estado da barra se for requisitado. Seria como usar
 * a função 'progresso' acima, não tem diferença não há limite para a quantia
 * de vezes que a chama. */
impl Display for ProgressoPercentual {
   // implementando visualização em sí.
   fn fmt(&self, formatador:&mut Formatter<'_>) -> Result_fmt
   {
      // nomeando por legibilidade.
      let qa = self.qtd_atual;
      let qt = self.qtd_total;
      // obtendo 'String' baseado no atual/total.
      let barra_de_progresso = progresso_unicode_e_colorido(qa, qt);
      // "imprimindo" visualização da atual barra.
      write!(formatador, "{}", barra_de_progresso)
   }
}

impl AddAssign<f32> for ProgressoPercentual {
   // implementação da atualização de dados.
   fn add_assign(&mut self, valor: f32) {
      *self += valor as usize; 
   }
}

impl ProgressoComando for ProgressoPercentual {
   fn terminado(&self) -> bool
      { self.esgotado }

   fn novo(total: usize) -> Self
      { ProgressoPercentual::cria(total) }

   fn percentual(&self) -> f32
      { self.qtd_atual as f32 / self.qtd_total as f32 }

   fn imprime(&mut self) -> Impressao
      { self.imprime() }
}

impl Traducao for ProgressoPercentual {
   fn new(total: usize) -> Self
      { ProgressoPercentual::novo(total) }

   fn finished(&self) -> bool
      { self.terminado() }

   fn print(&mut self) -> Impressao
      { self.imprime() }
}


/* == == == == == == == == == == == == == == == == == == == == == == == == =
 *                   Progresso Baseado em Tempo 
 * == == == == == == == == == == == == == == == == == == == == == == == = */
/**
   Uma estrutura que mostra uma barra de progresso, configurado os 
 milisegundos de disparo. Portanto, ele só forma a barra passado total de 
 **ms**, se for configurado com um valor maior ou igual à *1seg*, ou na 
 faixas dos micro segundos, tal estrutura não se forma, para o programa. Tal 
 restrição é necessária. Tal estrututa, assim com a percentual, são perfeitos
 para entrada de dados, assim não desaceleram o programa, por ficar gerando 
 a todo *ciclo* uma nova string formantando a barra.
*/
pub struct ProgressoTemporal {
   // quantidade atual passada.
   atual: usize,
   // o total inicialmente marcado.
   total: usize,
   // cronômetro para registra período de tempo.
   cronometro:Instant,
   // tempo delimitado para impressão em mili-seg.
   tempo: Duration
}

// implementação do método.
impl ProgressoTemporal {
   pub fn cria(total: usize, tempo: Duration) -> Self {
      let miliseg = tempo.as_millis();
      let cronometro = Instant::now();

      if miliseg < 10 && miliseg > 1000
         { panic!("Não é permitido tal rítmo!"); }

      ProgressoTemporal { atual: 0, total, cronometro, tempo }
   }

   /* Impressão limitada. Vê se houve um um tempo considerável desde a 
    * última impressão, se sim, então imprime novamente, se assim for 
    * requisitado. */
   pub fn imprime(&mut self) -> Impressao {
      // tempo passado desde o último registro.
      let tp = self.cronometro.elapsed();
      /* no mínimo, um terço de segundo deve-se
       * passar para liberar uma nova impressão. */
      if  tp >= self.tempo {
         // obtendo a barra de progresso simples.
         let bp = progresso_ascii_sem_cor(self.atual, self.total);
         // reiniciando o crônometro para recontagem de tempo.
         self.cronometro = Instant::now();
         // impressão em sí agora...
         Some(bp)
      }
      // se houver atingindo o máximo possível também.
      else if self.atual == self.total {
         // obtendo a barra de progresso simples.
         let bp = progresso_ascii_sem_cor(self.atual, self.total);
         // reiniciando o crônometro para recontagem de tempo.
         self.cronometro = Instant::now();
         // impressão em sí agora...
         Some(bp)
      }
      else { None }
   }

   pub fn terminado(&self) -> bool
      { self.atual >= self.total }
}

/* Agora com o atribuição, faz a mesma coisa do que o acima, porém sem 
 * retorno. No 'background' é atualização de valor. */
impl AddAssign<usize> for ProgressoTemporal {
   fn add_assign(&mut self, valor: usize) {
      // Não incrementa, apenas atualiza o atual valor.
      self.atual = valor;
   }
}

/* Visualização do atual estado da barra se for requisitado. Seria como usar 
 * a função 'progresso' acima, não tem diferença não há limite para a quantia
 * de vezes que a chama. */
impl Display for ProgressoTemporal {
   fn fmt(&self, formatador:&mut Formatter<'_>) -> Result_fmt {
      let qa = self.atual;
      let qt = self.total;
      // Obtendo 'String' baseado no atual/total.
      let barra_de_progresso = progresso_ascii_sem_cor(qa, qt);
      // Injetando na saída do console.
      write!(formatador, "{}", barra_de_progresso)
   }
}


/* == == == == == == == == == == == == == == == == == == == == == == == == =
 *                      Testes Unitários
 * == == == == == == == == == == == == == == == == == == == == == == == = */
#[cfg(test)]
mod tests {
   use super::*;
   use std::thread::{self};

   #[test]
   #[allow(deprecated)]
   fn teste_progresso_com_rotulo() {
      let rotulo:&str = "isso e um teste basico, sem panico";
      let mut logo:Logo = Logo::novo(rotulo).unwrap();
      for k in 1..=600_000 {
         let bp = progresso_data_rotulo(
            logo.para_string(),
            k, 600_000
         );
         print!("\r{}",bp);
         logo.movimenta_letreiro();
      }
      assert!(true);
   }

   #[test]
   fn progresso_com_barra_unicode() {
      let mut obj = ProgressoPercentual::cria(300);
      let pausa = Duration::from_millis(200); 
      let mut contador = 0;
      
      println!("\nO progresso está iniciando ...\n");

      while !obj.esgotado {
         if let Some(formatacao) = obj.imprime()
            { print!("\r{formatacao:}"); }

         obj += contador;
         thread::sleep(pausa);
         contador += 1;
      }
      print!("\n");
   }
}
