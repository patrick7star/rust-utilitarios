/*! 
 # Dimensao do terminal(tela)
    Tenta obter as dimensões do terminal, pois tal biblioteca é muito 
 extensa, e utiliza de tal funções bastantes, e nisso recorre a bibliotecas
 externas(aliás está usando uma agora). Por isso vamos tentar criar tais 
 funções manualmente para desutilizar bibliotecas externas que são 
 utilizadas junto com esta, com as usadas internamente aqui para uma função
 tão simples como tal. 

    O que está sendo feito aqui é na verdade uma cópia da atual biblioteca 
 "terminal_size", só que feita manualmente, pelos meus meios encontrados. Não
 vi o código original como é feito. A estrutura deste será igual a dele, 
 dentro de uma estrutura e tal. Porém sem a computabilidade de pixels.

    Claro que têm o fato de tal implementação ser intensamente mais lenta do
 que as geralmente utilizadas.
*/

// biblioteca padrão do Rust.
use std::process::Command;
use std::str::FromStr;
use std::io::{self, Write, stdin, stdout};
#[cfg(target_os="windows")]
use std::os::windows::raw::HANDLE;
#[cfg(target_os="windows")]
use std::mem::{transmute, zeroed};

/** Estrutura que "embrulha" um inteiro positivo de 16-bits, este 
 * significando a ***largura*** do terminal. */
pub struct Largura(pub u16);
/** Estrutura que "embrulha" um inteiro positivo de 16-bits, este 
 * significando a **altura** do terminal. */
pub struct Altura(pub u16);

// apelido para melhorar legibilidade.
type TerminalDimensao = Option<(Largura, Altura)>;
type TermLargura      = Result<Largura, &'static str>;
type TermAltura       = Result<Altura, &'static str>;
type Bytes            = Vec<u8>;
// Apelidos tornados públicos:
pub type Dimensao = (u16, u16);


/**   Tal Função, retorna tupla com dimensão, porém implementação
 * distinta da anterior, encapsulando valores com "structs" ao invés de 
 * Enum's. No fim, ela é apenas um wrap para a função 
 * 'obtem_terminal_dimensão', esta que tem um sufixo dependendo da 
 * plataforma. Com a nova refatoração, o código interno pra fazer isso,
 * ficou fantástico de simples.
 */
pub fn dimensao() -> TerminalDimensao {
   let (altura, largura) = obtem_dimensao_do_terminal_no_linux();

   // Retorno, porém removendo valores dos enum's.
   Some((Largura(largura), Altura(altura)))
}

/** A mesma coisa que dimesão, porém sem tolerância a erros. */
pub fn terminal_dimensao() -> Dimensao { 
   #[cfg(target_os="linux")]
   return obtem_dimensao_do_terminal_no_linux();
   #[cfg(target_os="windows")]
   return obtem_terminal_dimensao_no_windows();
}

/* Retorno mais rápido da `Dimensão` pois usa o API do Windows. Também
 * o tipo de retorno é mais simples, só uma tupla com dois inteiros 
 * positivos de 16-bits, em que, o primeiro é referente as linhas do 
 * terminal, e o segundo as colunas. */
#[cfg(target_os="windows")]
fn obtem_terminal_dimensao_no_windows() -> Dimensao {
/* Diferente das demais, esta aqui, já usa a API padrão do Windows para
 * obter tal valor relativo. Também o retorno é bem simplificado, 
 * quando comparado aos demais, aqui a tupla significa, diretamente,
 * nesta ordem, respectivamente linhas e colunas. */
   #[repr(C)]
   struct COOD { X: i16, Y: i16}

   #[repr(C)]
   struct SMALL_RECT { Left: i16, Top: i16, Right: i16, Bottom: i16 }

   #[repr(C)]
   struct CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: COOD,
        dwCursorPostion: COOD,
        wAttributes: i16,
        srWindow: SMALL_RECT,
        dwMaximumWindowSize: COOD
    }

   const STD_OUT_HANDLE: u32 = u32::MAX - 11;

    /* Depois de ter definido todas estruturas necessárias acima, que
     * são nativa do Windows, e que são usada nas declarações e 
     * chamadas abaixo, então "declaramos" as funções em si. */
    extern "C" {
        fn GetStdHandle(_: u32) -> HANDLE;
        fn GetConsoleScreenBufferInfo
          (_: HANDLE, _: *mut CONSOLE_SCREEN_BUFFER_INFO) -> bool;
    }

    let mut info: CONSOLE_SCREEN_BUFFER_INFO;
    let console: HANDLE; 
    let ptr_info: *mut CONSOLE_SCREEN_BUFFER_INFO;

    // Apelido para encurtar tipo de variável.
    type CSBI = CONSOLE_SCREEN_BUFFER_INFO;

    unsafe { 
        console = GetStdHandle(STD_OUT_HANDLE);
        info = zeroed();
        ptr_info = transmute::<&mut CSBI, *mut CSBI>(&mut info);
         GetConsoleScreenBufferInfo(console, ptr_info);
    };

    (info.dwSize.Y as u16, info.dwSize.X as u16)
}

/** De forma direta, retorna o Enum contendo apenas a largura do 
 * terminal. 
 *
 * *NOTA:* Apenas um 'wrapping' da função 'dimensão'. Isso faz uma redução
 * de código brutal. O futuro é descontinuar tais funções o quão rápido 
 * possível. O modo de ele retornar um erro, por chamar tal função do 
 * sistema está sendo descontinuado. Mais por motivos de compatibilidade,
 * ainda deixarei tais tipos de funções aqui. Que sem falar, são bastante
 * indiretas no retorno também. Que palhaçada é essa de Largura/Altura 
 * 'enum'. Nem lembro por que fiz assim. 
 */
pub fn terminal_largura() -> TermLargura 
   { Ok(dimensao().unwrap().0) }

/** Diretamente, retorna o Enum apenas com um inteiro de 16-bits 
 * *encapsulado* como dado dentro dele. */
pub fn terminal_altura() -> TermAltura 
    { Ok(dimensao().unwrap().1) }

/** Como sem um nome de módulo no momento, vamos colocar aqui a 
 * implementação de um prompt genérico. 
 */
pub fn lanca_prompt(dica:&str) -> String {
   // formantando dica.
   let dica = format!("{}: ", dica);
   let mut saida = stdout();
   let entrada = stdin();
   // buffer.
   let mut conteudo = String::new();

   // escreve o prompt.
   saida.write(dica.as_bytes()).unwrap();
   // sem guardar no buffer para ir antes do 'stdin'.
   saida.flush().unwrap();

   // retornando conteudo digitado.
   entrada.read_line(&mut conteudo).unwrap();
   // removendo quebra-de-linha.
   conteudo.pop().unwrap();
   return conteudo;
}


/* Faz, por enquanto, a função de obter e processar os dados, referentes as
 * dimensões do terminal em questão, pro Linux especificamente. Futuramente,
 * posso acoplar a função específica de outras plataformas, e tornar 
 * tal função com um nome mais genérico. */
#[cfg(target_os="linux")]
fn obtem_dimensao_do_terminal_no_linux() -> Dimensao {
   let nomes_das_funcoes = [
      stringify! (roda_comando_que_informa_dimensao_do_terminal_no_linux),
      stringify! (separa_bytes_referentes_a_cada_comprimento)
   ];
   let nome_a = nomes_das_funcoes[0];
   let nome_b = nomes_das_funcoes[0];
   // Melhores nomes prás funções utilizadas:
   let particionamento = separa_bytes_referentes_a_cada_comprimento;
   let conversao_em_str = converte_bytes_em_seus_respectivos_valores;

   match roda_comando_que_informa_dimensao_do_terminal_no_linux()
   {
      Ok(array) => 
      {
         match particionamento(array)
         {
            Some(tupla_in) => {
               let (altura, largura) = conversao_em_str(tupla_in);

               (altura, largura)

            } None =>
               { panic!("Erro ao chamar função '{}'.", nome_b); }
         }
      } Err(_) => 
         { panic!("Erro ao chamar a função '{}'", nome_a); }
   }
}

/* Roda o programa que informa as dimensões do shell em que foi chamada, 
 * e tenta extrair os bytes da string. Isso para que possa passar por uma
 * 'pipeline' de processamento posterior. 
 */
fn roda_comando_que_informa_dimensao_do_terminal_no_linux() 
 -> io::Result<Bytes> 
{
    match Command::new("tput").arg("lines").arg("cols").output() {
       Ok(saida) => Ok(saida.stdout),
       Err(erro) => 
          { Err(erro) }
   }
}

/* Pega os bytes que foram extraidos do comando rodado, então tenta 
 * separa-lo de acordo, e fazer um miniprocessamento(que seria algo como,
 * retirar byte de quebra-de-linha, provavelmente). Em casso de erro, uma
 * entrada inválida, ele não retorna a tupla com os bytes particionados. Ou
 * impossível alguma array dinâmica vázia, não sei ainda.
 */
fn separa_bytes_referentes_a_cada_comprimento
  (input: Bytes) -> Option<(Bytes, Bytes)>
{
   /* Reparte a array de bytes em duas: bytes da linha e bytes das 
    * coluna. */
   let mut caracteres = input.split_inclusive(|x| *x == 10);
   let (mut linhas, mut colunas): (Bytes, Bytes);

   linhas = match caracteres.next() {
      Some(bytes) => bytes.to_vec(),
      None => { return None; }
   };
   colunas = match caracteres.next() {
      Some(bytes) => bytes.to_vec(),
      None => { return None; }
   };

   // Retirando o último byte de cada, que representa quebra-de-linha.
   match linhas.pop() { None => {return None; } Some(_) => () }
   match colunas.pop() { None => {return None; } Some(_) => () }

   Some((linhas, colunas))
}

fn converte_bytes_em_seus_respectivos_valores
  (input: (Bytes, Bytes)) -> Dimensao 
{
   let linhas = String::from_utf8_lossy(&input.0[..]);
   let colunas = String::from_utf8_lossy(&input.1[..]);
   // Valores padrões caso a conversão dê errado.
   const LIN: u16 = 25;
   const COL: u16 = 80;

   (
      u16::from_str(&linhas).unwrap_or(LIN), 
      u16::from_str(&colunas).unwrap_or(COL)
   )
}


#[cfg(target_os="linux")]
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;

   #[test]
   fn conversao_em_numeros_inteiros_dos_bytes_extraidos() {
      let nome_a = stringify!
         (roda_comando_que_informa_dimensao_do_terminal_no_linux);
      let nome_b = stringify!
         (separa_bytes_referentes_a_cada_comprimento);

      match roda_comando_que_informa_dimensao_do_terminal_no_linux()
      {
         Ok(In) => 
         {
            match separa_bytes_referentes_a_cada_comprimento(In)
            {
               Some((lin_bytes, col_bytes)) => {
                  println!(
                     "Parte(A): {:?}\nParte(B): {:?}", 
                     lin_bytes, col_bytes
                  );
                  
                  let tupla_in = (lin_bytes, col_bytes);
                  let rotina = converte_bytes_em_seus_respectivos_valores;
                  let tupla_out = rotina(tupla_in);

                  println!(
                     "Linhas: {}  Colunas: {}", 
                     tupla_out.0, tupla_out.1
                  );
               } None =>
                  { assert!(false, "Erro ao chamar função '{}'.", nome_b); }
            }
         } Err(_) => 
            { assert!(false, "Erro ao chamar a função '{}'", nome_a); }
      }
   }

   #[test]
   fn dividindo_bytes_de_cada_comprimento_devidamente() {
      let nome = stringify!
         (roda_comando_que_informa_dimensao_do_terminal_no_linux);
      match roda_comando_que_informa_dimensao_do_terminal_no_linux()
      {
         Ok(In) => 
         {
            match separa_bytes_referentes_a_cada_comprimento(In)
            {
               Some((lin_bytes, col_bytes)) => {
                  println!(
                     "Parte(A): {:?}\nParte(B): {:?}", 
                     lin_bytes, col_bytes
                  );
               } None =>
                  { assert!(false, "Erro ao chamar função 'separa'."); }
            }
         } Err(_) => 
            { assert!(false, "Erro ao chamar a função '{}'", nome); }
      }
   }

   #[test]
   fn fonte_primaria_dos_dados_obtidos() {
      let rotina = roda_comando_que_informa_dimensao_do_terminal_no_linux;
      let output_a = rotina().unwrap();
      let output_b = &output_a.as_slice();
      let output_c = String::from_utf8_lossy(output_b);

      println!("Bytes: {:?}", output_a);
      println!("String formatada: \"{}\"", output_c);
   }

   #[test]
   fn testa_dimensao() {
      let (largura, altura):(u16, u16);
      if let Some((Largura(l), Altura(h))) = dimensao()
         { largura = l; altura = h; }
      else 
         { largura = u16::MIN; altura = u16::MAX; }
      assert!(dbg!(largura) > dbg!(altura));
   }

   #[test]
   fn funcaoTL() {
      match terminal_largura() {
         Ok(Largura(l)) => 
            { assert_eq!(l, 85); }
         Err(erro) => { 
            println!("{}", erro); 
            assert!(false);
         }
      };
   }

   #[test]
   fn funcaoTA() {
      match terminal_altura() {
         Ok(Altura(h)) => 
            { assert_eq!(h, 28); }
         Err(erro) => { 
            println!("{}", erro); 
            assert!(false);
         }
      };
   }

   #[test]
   fn testa_lanca_prompt() {
      let msg:&str = "digite mensagem-chave('chocolate')";
      let conteudo = super::lanca_prompt(msg);
      if cfg!(windows) { 
         // desconsiderando o recuo.
         let fim = conteudo.len() - 1;
         let chocolate = conteudo.get(0..fim).unwrap();
         assert_eq!(chocolate, "chocolate");
      } else 
        { assert_eq!(conteudo, "chocolate"); }
   }
}

#[cfg(test)]
#[cfg(target_os="windows")]
#[allow(non_snake_case)]
mod tests {
/* Bloco de teste que apenas é compilado para plataformas Windows.*/
   use super::*;
   use std::time::{Instant, Duration};

   #[test]
   fn funcaoNativaDoOS() {
      let clock = Instant::now();
      let dim = terminal_dimensao();
      let ta = clock.elapsed();
      let (Largura(l), Altura(h)) = dimensao().unwrap();
      let tb = clock.elapsed() - ta;
      let razao = tb.as_secs_f32() / ta.as_secs_f32();

      println!("Dimensão antiga: {}x{}", h, l); 
      assert_eq!(dim.0, h);
      assert_eq!(dim.1, l);
      println!("Dimensão do API: {:?}", dim);
      println!("Tempo do novo: {:#?}\nTempo do antigo: {:#?}", ta, tb);
      println!("Ele é quase {} mais rápido!", razao);
      assert!(ta < tb);
    }

   #[test]
   #[ignore="Preciso de iteração com o GUI"]
   fn valorRecuperadoEmTempoDeExecucao() {
       println!("Linhas x Colunas:");
        for _ in 1..=27 {
            let dim = terminal_dimensao();
            println!("\t{} x {}", dim.0, dim.1);
            std::thread::sleep(Duration::from_millis(800));
        }
    }
}
