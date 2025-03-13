/*! 
 # Grandezas mais legíveis 
  Faz conversões de grandezas referentes a dados utilizados em computação, 
  ou outros campos. O negócio é deixa-lo mais legiveis, agrupados na forma
  númerica de vários tipos de submultiplos e múltiplos de potências de dez.
  O tempo também é mostrado de forma mais humana, mas considerando a 
  organização fundamental de tempo, que por mais que seja ainda S.I., é algo
  bem diferente -- de sessenta em sessenta, então passa para de mil em mil 
  quando levado em conta valores menores que um segundo, é uma bagunça.
*/

// extensão do módulo:
mod fracao_seg;
mod aproxima;
mod generico;
// Biblioteca padrão do Rust(apenas o necessário).
use std::{
   iter::{FromIterator}, collections::{HashMap}, time::{Duration},
};
// reexportando certas funções.
pub use aproxima::tempo_detalhado;
pub use generico::tempo_humano;
pub use fracao_seg::tempo_fracao;

// Apelidos pra uma 'lookup table'.
type PesosTempo<'a> = HashMap<&'a str, f32>;

// múltiplos de tempo(equivalente em seg).
const MINUTO:  f32 = 60.0;             // segundos por minuto.
const HORA:    f32 = MINUTO * MINUTO;  // segundos por hora.
const DIA:     f32 = 24.0 * HORA;      // segundos por dia.
const MES:     f32 = 30.0 * DIA;       // segundos/mês.
const ANO:     f32 = 365.0 * DIA;      // segundos/ano.
const DECADA:  f32 = 10.0 * ANO;       // segundos/década.
const SECULO:  f32 = 10.0 * DECADA;    // segundos por século.
const MILENIO: f32 = 10.0 * SECULO;    // seg/milênio.

// múltiplos de tamanho(equivalente em bytes).
const KILO: u64 = 2_u64.pow(10);  // bytes por kB.
const MEGA: u64 = 2_u64.pow(20);  // bytes por MB.
const GIGA: u64 = 2_u64.pow(30);  // bytes por GB.
const TERA: u64 = 2_u64.pow(40);  // bytes por TB.
const PETA: u64 = 2_u64.pow(50);  // bytes por PB.


/** Retorna uma string contendo o valor legendado porém numa faixa mais 
 legível. */
pub fn tempo(segundos:u64, contracao:bool) -> String {
   // renomeação da variável a comparar e computar.
   let t: f32 = segundos as f32;
   let calculo: f32;
   let sigla: &str;

   if (MINUTO..HORA).contains(&t) {
      sigla = if contracao {"min"} else {"minutos" };
      calculo = t / MINUTO;
   }
   else if t >= HORA && t < DIA {
      sigla = if contracao{"h"} else{"horas"};
      calculo = t / HORA;
   }
   else if t >= DIA && t < MES {
      sigla = "dias";
      calculo = t / DIA;
   }
   else if t >= MES && t < ANO {
      sigla = if contracao{"mês"} else{"meses"};
      calculo = t / MES;
   }
   else if t >= ANO && t < DECADA {
      sigla = "anos";
      calculo = t / ANO;
   }
   else if t >= DECADA && t < SECULO {
      sigla = if contracao{"dec"} else{"décadas"};
      calculo = t / DECADA;
   }
   else if t >= SECULO && t < MILENIO {
      sigla = if contracao{"sec"} else{"séculos"};
      calculo = t / SECULO;
   }
   else if t >= MILENIO && t < 10_f32*MILENIO {
      sigla = "milênios";
      calculo = t / MILENIO;
   }
   else {
      sigla = if contracao{"seg"} else{"segundos"};
      calculo = t;
   }
   format!("{:0.1} {}", calculo, sigla)
} 

/** Retorna uma string contendo o tamanho legendado com um múltiplo, porém 
 * de forma mais legível. */
pub fn tamanho(qtd:u64, contracao:bool) -> String { 
   if qtd >= KILO && qtd < MEGA {
      let sigla = if contracao{"KiB"} else{"kilobytes"};
      format!("{:.1} {}", (qtd as f32 / KILO as f32), sigla)
   }
   else if qtd >= MEGA && qtd < GIGA {
      let sigla = if contracao{"MiB"} else{"megabytes"};
      format!("{:.1} {}", (qtd as f32 /MEGA as f32), sigla)
   }
   else if qtd >= GIGA && qtd < TERA {
      let sigla = if contracao{"GiB"} else{"gigabytes"};
      format!("{:.1} {}", (qtd/GIGA) as f32, sigla)
   }
   else if qtd >= TERA && qtd < PETA {
      let sigla = if contracao{"TiB"} else{"terabytes"};
      format!("{:.1} {}", (qtd/TERA) as f32, sigla)
   }
   else {
      let sigla = if contracao{"B's"} else {"bytes"};
      format!("{:.1} {}", qtd, sigla)
   }
}

/** Pega valores muito grande, maiores que mil, e coloca eles de forma
 mais legivel, com três algarismos significativos no máximo, e o peso de
 em casas decimais. */
pub fn valor_legivel(qtd: usize) -> String {
   let mantisa: f64;
   let peso: &str;

   // só converte até a faixa dos quatrilhões.
   if qtd > 1_000_000_000_000_000 {
      mantisa = qtd as f64 / 1_000_000_000_000_000.0;
      peso = " quadrilhões";
   } else if qtd > 1_000_000_000_000 {
      mantisa = qtd as f64 / 1_000_000_000_000.0;
      peso = "tri";
   } else if qtd > 1_000_000_000 {
      mantisa = qtd as f64 / 1_000_000_000.0;
      peso = "bi";
   } else if qtd > 1_000_000 {
      mantisa = qtd as f64 / 1_000_000.0;
      peso = "mi";
   } else if qtd > 1000 {
      mantisa = qtd as f64 / 1000.0;
      peso = "mil";
   } else {
      mantisa = qtd as f64;
      peso = "";
   } 

   // convertendo para string, então adicionando ...
   format!("{:0.1}{}", mantisa, peso)
}


/** Está função processa string com representação de tempo, seja qual for 
 * o múltiplo represetando-a. A formatação básica do input pra funcionar é,
 * valor númerico(seja decimal ou apenas um inteiro), e o pesso representando
 * tal, se não houver um, a primeira parte, sendo esta a numerica, apenas é
 * considerada como segundos. */
pub fn interpleta_string_de_tempo(input: &str) -> Option<Duration> {
   let mut valor = String::new();
   let mut peso = String::new();
   #[allow(non_snake_case)]
   let LOOKUP_TABLE: PesosTempo = HashMap::from_iter([
      ("segundo", 1.0), ("minuto", MINUTO), ("hora", HORA), ("dia", DIA),
      ("mes", MES), ("ano", ANO), 
      // Versão mais curta das formas acima.
      ("seg", 1.0), ("min", MINUTO), ("h", HORA), ("d", DIA),
      ("m", MES), ("a", ANO), 
      // Versão Plural dos pesos acimas.
      ("segundos", 1.0), ("minutos", MINUTO), ("horas", HORA), 
      ("dias", DIA), ("meses", MES), ("anos", ANO), 
      // Versão acentuada.
      ("mês", MES)
   ]);

   // Separa à parte númerica da parte textual.
   for simbolo in input.chars() 
   {
      if simbolo.is_ascii_digit() || simbolo == '.'
         { valor.push(simbolo); }
      else if simbolo.is_alphabetic()
         { peso.push(simbolo); }
   }

   if let Ok(valor_real) = valor.parse::<f32>() 
   {
      if peso.is_empty()
         { return Some(Duration::from_secs_f32(valor_real)); }

      if let Some(peso_real) = LOOKUP_TABLE.get(peso.as_str()) 
         { return Some(Duration::from_secs_f32(valor_real * peso_real)); }
   }
   None
}


#[cfg(test)]
mod tests {
   use crate::legivel::*;

   #[test]   
   fn testa_tamanho_legibilidade() {
      let mut x = 3419; 
      println!("{} ==> {}",x, tamanho(x, false));
      x = 10293; 
      println!("{} ==> {}",x, tamanho(x, false));
      x = 1982419; 
      println!("{} ==> {}",x, tamanho(x, true));
      x = 123048190; 
      println!("{} ==> {}",x, tamanho(x, true));
      x = 1000348293192; 
      println!("{} ==> {}",x, tamanho(x,false));
      x = 193843092384101; 
      println!("{} ==> {}",x, tamanho(x, true));
      assert!(true);
   }

   #[test]
   fn tempo_legibilidade() {
      let mut t:u64 = 36;
      println!("{} ==> {}", t, tempo(t, false));
      t = 152;
      println!("{} ==> {}", t, tempo(t, true));
      t = 552;
      println!("{} ==> {}", t, tempo(t, false));
      t = 9000;
      println!("{} ==> {}", t, tempo(t, false));
      t = 38910;
      println!("{} ==> {}", t, tempo(t, true));
      t = 1039842;
      println!("{} ==> {}", t, tempo(t, false));
      t = 30489123918;
      println!("{} ==> {}", t, tempo(t, true));
      t = 99990192152;
      println!("{} ==> {}", t, tempo(t, true));
      t = 1110238951152;
      println!("{} ==> {}", t, tempo(t, false));
      assert!(true);
   }

   #[test]
   fn testa_tempo_detalhado() {
      assert_eq!(
         super::tempo_detalhado("3.5 horas"), 
         Some(String::from("3 horas 30 minutos"))
      );

      assert_eq!(
         super::tempo_detalhado("4.2 dias"),
         Some(String::from("4 dias 5 horas"))
      );

      assert_eq!(
         super::tempo_detalhado("3.5 meses"), 
         Some(String::from("3 meses 15 dias"))
      );

      assert_eq!(
         super::tempo_detalhado("4.2 anos"),
         Some(String::from("4 anos 2 meses"))
      );


      assert_eq!(
         super::tempo_detalhado("7.9 décadas"),
         Some(String::from("7 décadas 9 anos"))
      );

      assert_eq!(
         super::tempo_detalhado("8.5 séculos"),
         Some(String::from("8 séculos 5 décadas"))
      );

      assert_eq!(
         super::tempo_detalhado("1.5 milênios"),
         Some(String::from("1 milênios 5 séculos"))
      );
   } 

   #[test]
   #[allow(non_snake_case)]
   fn aplicacao_da_interpletacao_de_strings() {
      let inputs = [
         "12.3 minutos", "30 horas", "53 segundos", "12 meses", 
         "1 mês", 
      ];
      let inputs_a = [
         "8.7 min", "8.7min", "123seg", "123 seg", "8a", "8 a", "25.4 m",
         "25.4m", "87.3d", "87.3 d"
      ];
      let inputs_b = [
         "24 minutos", "78.1segundos", "10 anos", "23meses", "14dias", 
         "31 horas"
      ];

      println!("\nExemplos de amostras bem formais ...");
      for In in inputs { 
         let Out = interpleta_string_de_tempo(In);
         println!("\t- {} ===> {:?}", In, Out); 
      }

      println!("\nExemplos de amostras contracionadas e espaçadas ...");
      for In in inputs_a { 
         let Out = interpleta_string_de_tempo(In);
         println!("\t- {} ===> {:?}", In, Out); 
      }

      println!("\nExemplos de amostras com pesos completos no plural ...");
      for In in inputs_b { 
         let Out = interpleta_string_de_tempo(In);
         println!("\t- {} ===> {:?}", In, Out); 
      }
   }
}
