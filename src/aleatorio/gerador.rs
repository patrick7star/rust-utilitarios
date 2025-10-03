/*   Método mais veloz de gerar números aleatórios. Este aqui, chama uma 
 * única vez um byte aleatório, ou as unidades de nanosegundos para
 * gerar um valor aleatório, os demais são obtidos usando embaralhamento
 * de bits de uma "semente" que pode ser dado pelo endereço aleatório de
 * alguma entidade na memória, ou o selo de tempo no momento da execução,
 * ou um byte aleatório. a escolha é livre.
 *
 *   Método de alimentação de semente, ao invés de pegar o termo que mais 
 * varia no selo de tempo do sistema, pegamos um valor inicial e ficamos
 * mudando seus bits internos, a cada nova chamada para gerar entropia. A
 * questão é: qual será o valor inicial? No Unix usanos o leitor de byte
 * do sistema -- neste caso a primeira leitura inicial seria bem lenta,
 * porém o restante iria seguir o algoritmo padrão, já que os bytes iniciais
 * já teriam sido lidos para gerar um valor inicial; ou mesmo qualquer 
 * endereço de memória na execução inicial do programa, seja de uma função 
 * ou variável global, já que ela muda à cada nova execução.
 * 
 *   Esta técnica é imensamente mais veloz que as demais já tentadas. E 
 * vamos realizar os benchmarks aqui. Porém, ela tem um problema padrão por 
 * não ser thread-safe, já que, dois processos forqueados computariam, o 
 * mesmo valor "aleatório", portanto não seria aleatório.
 */

use std::ops::RangeInclusive as Intervalo;
use std::fs::OpenOptions;
use std::io::Read;
use std::time::{SystemTime};


// Valor inicial do maior inteiro positivo.
static mut SEMENTE: usize = 1;
// Se for já computador um valor inicial...
static mut ACIONADO: bool = false;
const N: usize = std::mem::size_of::<usize>();

#[cfg(target_os="linux")]
fn pega_n_bytes(n: usize) -> Vec<u8> 
{
   // tal gera bytes aleatórios a todo instante.
   let mut arquivo_entropico = {
      OpenOptions::new()
      .read(true)
      .open("/dev/random")
      .unwrap()
   };
   /* buffer de 64-bits(8 bytes), porém 
    * se pode pega apenas o necessário, 
    * claro, sem extrapolar o limite(que será 
    * expandido futuramente). */
   let mut bytes: [u8; 8] = [0; 8];

   // tenta lê "device" ...
   match arquivo_entropico.read(&mut bytes) {
      Ok(_) => (),
      Err(_) =>
         { panic!("não possível lê '/dev/random'"); }
   };

   // array com tamanho demandado.
   return bytes[0..n].to_vec();
}

fn copia_bytes(mut vetor: Vec<u8>, array: &mut [u8; N])
{
   for (p, x) in vetor.drain(..).enumerate()
      { array[p] = x; }
}

#[cfg(target_os="windows")]
fn bytes_pseudorandomicos(bytes: &mut [u8]) {
/* O método usado prá encontrar bytes "randômicos" é, pega um 'timestamp',
 * computa os nanosegundos decorridos desde Unix Epoch, então converte
 * tal valor num 'double'; interpreta seus bytes de forma errada, e os 
 * copia. */
   const INICIO: SystemTime = SystemTime::UNIX_EPOCH;
   let agora = SystemTime::now();

   match agora.duration_since(INICIO) {
      Ok(decorrido) => {
         let tempo = decorrido.as_secs_f64();

         bytes.copy_from_slice(tempo.to_be_bytes().as_slice());
      } Err(_) => 
         { bytes.fill(0xFF); }
   };

   for X in bytes
      { *X = 1; }
}

fn computa_semente()
{
/* Tenta achar um valor randômico usando de vários meios, e também, bem
 * importante, a função computa apenas uma única vez em execução. */
   let nao_foi_acionado_ainda = unsafe { !ACIONADO };
   let mut buffer: [u8; N] = [0x00; N];
   
   if nao_foi_acionado_ainda
   {
      #[cfg(target_os="linux")]
      copia_bytes(pega_n_bytes(N), &mut buffer);
      #[cfg(target_os="windows")]
      bytes_pseudorandomicos(&mut buffer); 

      unsafe {
         SEMENTE = usize::from_le_bytes(buffer);
         ACIONADO = true;
      }
   }
}

fn gera_novo_usize() -> usize
{
   // Remexe bits do número para gerar novo número aleatório.
   computa_semente();

   // Move 32-bits à direita, e mais 32 bits à esquerda, não de uma vez
   // apenas, mas de forma alternada. No fim, retorna o novo resultado.
   unsafe { 
      SEMENTE ^= SEMENTE >> 15;
      SEMENTE ^= SEMENTE << 12;
      SEMENTE ^= SEMENTE >> 6;
      SEMENTE ^= SEMENTE << 11;
      SEMENTE ^= SEMENTE >> 9;
      SEMENTE ^= SEMENTE << 7;

      SEMENTE
   }
}

fn erro_mensagem_isize(a: isize, b: isize)
{
/* O valor inicial do intervalo tem que seguir a ordem matemática. */
   if a > b {
      let msg_i = "erro matemático no intervalo";
      let msg_ii = format!("{} < {}(Errado!)", a, b);
      let msg_iii = format!("o correto é {} > {}", a, b);
      let msg_iv = format!("logo {}..={} é o correto", b, a);

      panic!(
         "{}: {}, {}, {} a se tentar.", 
         msg_i, msg_ii, 
         msg_iii, msg_iv
      );
   }
}

pub mod sortear {
   use super::*;

   /// Gera um intero de(quantia variada) bytes, randomicamente.
   pub fn usize(i: Intervalo<usize>) -> usize 
   {
      let (a, b) = (*i.start(), *i.end());
      let n = gera_novo_usize();

      // Corrigindo nos intervalos.
      if a > 0 && b <= usize::MAX
         { n % ((b - a) + 1) + a }
      else if a == 0 && b < usize::MAX
         { n % (b + 1) }
      else { n }
   }

   /// Gera maior inteiro com sinal que a máquina permite.
   pub fn isize(i: Intervalo<isize>) -> isize
   {
      let (a, b) = (*i.start(), *i.end());

      if a >= 0 && b <= isize::MAX { 
      /* Se cair neste caso, um valor de 0 ao máximo será gerado, inclusive
       * o extremo direito, entretanto, apenas positivos. */
         let c = a as usize;
         let d = b as usize;
         self::usize(c..=d) as isize

      } else if (a < 0 && b <= -1) && a >= isize::MIN  { 
      /* Caso no espectro negativo, o mesmo que acima, porém com sinal. E 
       * essa mesma a jogada, tira módulo do intervalo, inverte, e negativa
       * o resultado. */
         erro_mensagem_isize(a, b);

         let c = b.abs();
         let d = a.abs();

         if a == isize::MIN
            // { (-1) * self::isize(b.abs()..=(a+1).abs()) -1 }
            { (-1) * self::isize(c..=(d + 1)) - 1 }
         else
            // { (-1) * self::isize(b.abs()..=a.abs()) }
            { (-1) * self::isize(c..=d) }

      } else if a < 0 && b >= 0 {
      /* Caso que o extremo esquerdo é negativo, e o oposto positivo. A 
       * distância dentre eles será a soma das distâncias até o ponto 
       * central(zero). 
       */
         let max = usize::MAX;
         #[allow(non_snake_case)]
         let S = self::usize(0..=max) as isize;
         /* Computando comprimento levando em conta os sinais. */
         let t =  a.abs() + b;

         /* Escolher, usando de divisão euclidiana, um valor entre tal
          * comprimento, posteriormente deslocar ele à partir do valor 
          * inicial demanadado. */
         a + S.rem_euclid(t)
      } else { 
         if !self::bool()
            { self::isize(-1..=isize::MIN) }
         else
            { self::isize(0..=isize::MAX) }
      }
   }

   /// Gera um valor lógico, `true` ou `false`.
   pub fn bool() -> bool 
   /* Se um valor randômico é maior o igual a cinco, o lançamento se torna
    * verdadeiro, caso contrário, falso. */
      { usize(0..=9) >= 5 }

   /// Gera valor inteiro positivo de 8-bits(0 à 255).
   pub fn u8(i: Intervalo<u8>) -> u8 
   { 
      let a = *i.start() as usize;
      let b = *i.end() as usize;

      usize(a..=b) as u8
   }

   /// Gera um inteiro de 8-bits de modo randômico.
   pub fn i8(i: Intervalo<i8>) -> i8 
   {
      let a = *i.start() as isize;
      let b = *i.end() as isize;

      isize(a..=b) as i8
   }

   /// Gera um inteiro positivo de 16-bits randômico.
   pub fn u16(i: Intervalo<u16>) -> u16 {
      let a = *i.start() as usize;
      let b = *i.end() as usize;

      usize(a..=b) as u16
   }

   /// Gera um inteiro de 16-bits de modo randômico.
   pub fn i16(i: Intervalo<i16>) -> i16 
   {
      let a = *i.start() as isize;
      let b = *i.end() as isize;

      isize(a..=b) as i16
   }

   /// Gera um inteiro positivo de 32-bits randômico.
   pub fn u32(i: Intervalo<u32>) -> u32 
   {
      let a = *i.start() as usize;
      let b = *i.end() as usize;

      usize(a..=b) as u32
   }

   /// Gera um inteiro de 32-bits(4 bytes) de modo randômico.
   pub fn i32(i: Intervalo<i32>) -> i32 
   { 
      let a = *i.start() as isize;
      let b = *i.end() as isize;

      isize(a..=b) as i32
   }
   
   /// Gera um inteiro positivo de 64-bits(8 bytes) randômico.
   pub fn u64(i: Intervalo<u64>) -> u64 
   { 
      let a = *i.start() as usize;
      let b = *i.end() as usize;

      usize(a..=b) as u64
   }

   /// Gera um inteiro de 64-bits(8 bytes) de modo randômico.
   pub fn i64(i: Intervalo<i64>) -> i64 
   {
      let a = *i.start() as isize;
      let b = *i.end() as isize;

      isize(a..=b) as i64
   }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

   #[test]
   fn simples_sorteamento_do_isize()
   {
      println!("Tem que está entre(incluso) -100 à 50:");

      for _ in 1..=30 
      { 
         let range = -100..=50isize;
         let X = super::sortear::isize(range);
         print!(" {} ", X); 
      }
      print!("\n");
   }

   #[test]
   fn simples_sorteamento_do_usize()
   {
      println!("Tem que está entre(incluso) 9 à 38:");

      for _ in 1..=30 
         { print!("{} ", super::sortear::usize(9..=38)); }
      print!("\n");
   }

   #[test]
   fn distribuicao_do_isize_entre_negativos_e_positivos()
   {
      let mut negativos: f64 = 0.0;
      const TOTAL: usize = 90_000;
      
      for _ in 1..=TOTAL
      {
         let X = super::sortear::isize(-100..=100);

         if X < 0 
            { negativos += 1.0; }
      }
      let percentual = negativos / TOTAL as f64;

      println!(
         "Positivos: {:0.2}%\tnegativos: {:0.2}%",
         percentual * 100.0, (1.0 - percentual) * 100.0
      );
      assert!(percentual - 0.50 < 0.05);
   }
}
