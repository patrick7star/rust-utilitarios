
pub const CAPACIDADE: u8 = 50;
const COMPONENTE: &'static str = "#";
const VAZIO: &'static str = ".";


// cálcula número de algarismos de dado número.
pub fn conta_algarismos(valor: usize) -> usize {
   let mut d = valor as f32;
   // contador de divisões.
   let mut contador = 0;

   while d > 1.0 {
      // cada divisão por dez, conta um.
      d = d / 10.0;
      contador += 1;
   }

   // retorno a contagem contabilizando um ...
   contador + 1
}

pub fn cria_barra_unicode_colorida(taxa: f32, comprimento: u8) -> String
{
   let c = comprimento as usize;
   let p = comprimento as f32;
   let componente_a: &str = "\u{25a0}";
   let componente_b: &str = "\u{00b7}";
   let mut barra = String::with_capacity(c);
   let quantia = (taxa * p) as usize;
   let consumido = componente_a.repeat(quantia);
   let vazio = componente_b.repeat(c - quantia);

   barra.push_str(&consumido);
   barra.push_str(&vazio);
   barra
}

pub fn progresso_unicode_e_colorido(n: usize, t: usize) -> String {
   let percentagem = (n as f32) / (t as f32);
   // qtd. de algarismos que será alcançado o valor atual.
   let qtd_algs = conta_algarismos(t);
   // let delimitadores = ['\u{ff3b}', '\u{ff3d}'];
   let delimitadores = ['\u{2772}', '\u{2773}'];
   let barra = cria_barra_unicode_colorida(percentagem, CAPACIDADE);

   if percentagem > 1.0 || (n > t)
      { panic!("os valores de atual supera o total!"); }
      
   /* Molde da string retornada representando por inteiro a barra de 
    * progresso. */
   format!(
      "{0:>espaco$} de {1} {4}{2}{5}{3:>5.1}%",
      n, t, barra, percentagem * 100.0, delimitadores[0],
      delimitadores[1], espaco = qtd_algs
   )
}

pub fn cria_barra_ascii_sem_cor(taxa: f32, comprimento: u8) -> String
{
   let c = comprimento as usize;
   let p = comprimento as f32;
   let componente_a: &str = COMPONENTE;
   let componente_b: &str = VAZIO;
   let mut barra = String::with_capacity(c);
   let quantia = (taxa * p) as usize;
   let consumido = componente_a.repeat(quantia);
   let vazio = componente_b.repeat(c - quantia);

   barra.push_str(&consumido);
   barra.push_str(&vazio);
   barra
}

pub fn progresso_ascii_sem_cor(atual: usize, total: usize) -> String
{
   let percentagem = (atual as f32) / (total as f32);
   let barra = cria_barra_ascii_sem_cor(percentagem, CAPACIDADE);

   if percentagem > 1.0 || (atual > total)
      { panic!("os valores de atual supera o total!"); }
      
   /* Molde da string retornada representando por inteiro a barra de 
    * progresso. */
   format!(
      "{0:>espaco$} de {1} [{2}]{3:>5.1}%",
      atual, total, barra, percentagem * 100.0, 
      espaco = conta_algarismos(total)
   )
}
