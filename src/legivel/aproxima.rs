
// biblioteca padrão do Rust.
use std::str::FromStr;
// próprio módulo.
use super::{
   SECULO, DECADA, MILENIO, MES,
   DIA, HORA, MINUTO, ANO
};

/* arredonda o valor decimal da string
 * que representa tempo, segue as mesmas
 * regras de arredondamento de um valor comum. */
fn arredondando_str<S: AsRef<str>>(s: &S) -> String {
   let mut partes = s.as_ref().split_whitespace();
   let arredonda = { |x: f32| {
      let inteiro: f32 = (x as u8) as f32;
      let fracao: f32 = (inteiro - x).abs();
      if fracao < 0.5 
         { inteiro as u8 }
      else 
         { (inteiro + 1.0) as u8 }
   }};
   let valor = f32::from_str(partes.next().unwrap()).unwrap();
   let peso = partes.next().unwrap();
   format!("{} {}", arredonda(valor), peso)
}

/** rescreve de forma mais legível a transformação
 convertendo a parte decimal em inteira rotuláda
 com o múltiplo/submultiplo adequado.
 Apenas funciona com valores decimais com dois
 dígitos significativos:

 # Exemplo:
```
use utilitarios::legivel::tempo_detalhado;

assert_eq!(
    tempo_detalhado("3.5 horas"),
   Some(String::from("3 horas 30 minutos"))
);
assert_eq!(
   tempo_detalhado("8.1 minutos"),
   Some("8 minutos 6 segundos".to_string())
);
assert_eq!(
   tempo_detalhado("5.4 meses"),
   Some("5 meses 12 dias".to_string())
);
assert_eq!(tempo_detalhado("12 min"), None);
``` */
pub fn tempo_detalhado<'a, S>(tempo_str:&'a S) 
  -> Option<String> 
  where S: AsRef<str> + ?Sized
{
   let mut partes = tempo_str.as_ref().split_whitespace();
   let valor = partes.next().unwrap();
   let peso = partes.next().unwrap();
   
   let (parte_inteira, parte_fracionaria) = {
      match valor.split_once(".") {
         Some((int, fraction)) => (int, fraction),
         // se não têm fração, não tem porque processar...
         None => { return None; }
      }
   };

   // partes inteira e fracionária em valores numéricos.
   let mut aux = String::from("0.");
   aux += parte_fracionaria;
   let f = f32::from_str(&aux).unwrap_or(0.0);

   // converte no peso antecessor do dado.
   let calc:u64;
   if peso.contains("min") 
      { calc = (MINUTO * f) as u64; }
   else if peso.contains("hora")
      { calc = (HORA * f) as u64; }
   else if peso.contains("dia")
      { calc = (DIA * f) as u64 }
   else if peso.contains("mes") 
      { calc = (MES * f) as u64; }
   else if peso.contains("ano")
      { calc = (ANO * f) as u64; }
   else if peso.contains("dec") || peso.contains("déc")
      { calc = (DECADA * f) as u64; }
   else if peso.contains("sec") || peso.contains("séc")
      { calc = (SECULO * f) as u64; }
   else if peso.contains("mil")
      { calc = (MILENIO * f) as u64; }
   else 
      // sem peso, o argumento fica inválido.
      { return None; }

   // converte fração.
   let conversao = super::tempo(calc, false);
   /* arredonda se está quebrada, pois a recursão 
    * do "tempo_detalhado" ainda não foi implementada.  */
   let conversao = arredondando_str(&conversao);

   // concatenando as partes:
   let mut s = String::new();
   s += parte_inteira;
   s += " ";
   s += peso;
   s += " ";
   s += conversao.as_str();

   // aparando desnecessário.
   s = s.replace(".0 ", " ");

   // retorna resultado encapsulado.
   return Some(s);
}
