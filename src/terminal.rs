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
use std::io::{Write, stdin, stdout};
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
#[cfg(target_os="windows")]
type Dimensao         = (u16, u16);


/** De forma direta, retorna o Enum contendo apenas a largura do 
 * terminal. */
pub fn terminal_largura() -> TermLargura {
   // executa comando para obter largura primeiramente ...
   let mut resultado:Vec<u8> = {
      if cfg!(linux) || cfg!(unix) {
          match Command::new("tput").arg("cols").output() {
             // retorna array de bytes que é o resultado.
             Ok(r) => r.stdout,
             Err(_) => 
                { return Err("não foi possível obter 'Largura'"); }
          }
      } else if cfg!(windows) {
          let mut comando = Command::new("powershell");
          comando.arg("-Command");
          comando.arg("Write-Host");
          comando.arg("$Host.UI.RawUI.WindowSize.Width");
          comando.arg("|");
          comando.arg("Out-String");
          match comando.output() {
             // retorna array de bytes que é o resultado.
             Ok(r) => r.stdout,
             Err(_) => 
                { return Err("não foi possível obter 'Largura'"); }
          }
      } else {
         println!(
            "o que está considerando:
            \r\tlinux: {}
            \r\twindows: {}
            \r\tunix: {}",
            cfg!(linux), cfg!(windows), cfg!(unix)
         );
         panic!("ainda não implementado para tal sistema."); 
      }
   };

   // removendo quebra de linha.
   if cfg!(windows) {
      // removendo espaço em branco e recuo '\n\r'.
      resultado.pop();
      resultado.pop();
      resultado.pop();
   } else if cfg!(linux) || cfg!(unix) 
       { resultado.pop(); }

   // transformando em número.
   let num_str = String::from_utf8_lossy(&resultado[..]);
   /* converte para um inteiro positivo, e 
    * e registra valor para retorno, posteriormente. */
   let largura = u16::from_str(&num_str).unwrap();

   // retornando encapsulado para possível erro.
   Ok(Largura(largura))
}

/** Diretamente, retorna o Enum apenas com um inteiro de 16-bits 
 * *encapsulado* como dado dentro dele. */
pub fn terminal_altura() -> TermAltura {
   // executa comando para obter largura primeiramente ...
   let mut resultado: Bytes = {
      if cfg!(unix) || cfg!(linux) {
          match Command::new("tput").arg("lines").output() {
             // retorna array de bytes que é o resultado.
             Ok(r) => dbg!(r.stdout),
             Err(_) => 
                { return Err("não foi possível obter 'Largura'"); }
          }
      } else if cfg!(windows) {
          let mut comando = Command::new("powershell");
          comando.arg("-Command");
          comando.arg("Write-Host");
          comando.arg("$Host.UI.RawUI.WindowSize.Height");
          comando.arg("|");
          comando.arg("Out-String");
          match comando.output() {
             // retorna array de bytes que é o resultado.
             Ok(r) => dbg!(r.stdout),
             Err(_) => 
                { return Err("não foi possível obter 'Altura'"); }
          }
      } else { 
         println!(
            "o que está considerando:
            \r\tlinux: {}
            \r\twindows: {}
            \r\tunix: {}",
            cfg!(linux), cfg!(windows), cfg!(unix)
         );
         panic!("ainda não implementado para tal sistema."); }
   };

   // removendo quebra de linha.
   if cfg!(windows) {
      // removendo espaço em branco e recuo '\n\r'.
      resultado.pop();
      resultado.pop();
      resultado.pop();
   } else if cfg!(linux) || cfg!(unix) 
       { resultado.pop(); }

   // transformando em número.
   let num_str = String::from_utf8_lossy(&resultado);
   /* converte para um inteiro positivo, e 
    * e registra valor para retorno, posteriormente. */
   let altura = u16::from_str(&num_str).unwrap();

   // retornando encapsulado para possível erro.
   Ok(Altura(altura))
}

/** Tal Função, retorna tupla com dimensão, porém implementação
 distinta da anterior, encapsulando valores com "structs" ao invés de 
 Enum's. */
pub fn dimensao() -> TerminalDimensao {
   /* usando construto acima de auxilio, para 
    * não ter que fazer a mesma coisa de novo.
    */
   let altura:u16 = match terminal_altura() {
      Ok(Altura(h)) => h,
      Err(_) => {return None; }
   };
   let largura:u16 = match terminal_largura() {
      Ok(Largura(l)) => l,
      Err(_) => { return None; },
   };

   // retorno, porém removendo valores dos enum's.
   Some((Largura(largura), Altura(altura)))
}

/** Retorno mais rápido da `Dimensão` pois usa o API do Windows. Também
 * o tipo de retorno é mais simples, só uma tupla com dois inteiros 
 * positivos de 16-bits, em que, o primeiro é referente as linhas do 
 * terminal, e o segundo as colunas. */
#[allow(non_snake_case)]
#[cfg(target_os="windows")]
pub fn terminal_dimensao() -> Dimensao {
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
   drop(conteudo.pop().unwrap());
   return conteudo;
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;

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
   #[cfg(target_os="windows")]
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
   #[cfg(target_os="windows")]
   fn valorRecuperadoEmTempoDeExecucao() {
       println!("Linhas x Colunas:");
        for _ in 1..=27 {
            let dim = terminal_dimensao();
            println!("\t{} x {}", dim.0, dim.1);
            std::thread::sleep(Duration::from_millis(800));
        }
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
