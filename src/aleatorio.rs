/*!  
 # Geração de números aleatórios
  Neste código vamos apresentar algumas funções
 que geram, números inteiros e flutuantes, 
 arrays inteiras e intervalos de números
 aleatórios.

  O algoritmo é simples, pega o clock ou 
 tempo no exato momento de execução, como geralmente
 é um número na casa das dezenas de milhares, 
 obtem seu último dígito, que vária muito, e
 soma com outros mais cem números obtidos pelo
 mesmo processo, e, novamente arranca-se o
 valor unitário do número, que sempre varia
 de zero à dez. Em cima desta função que dá
 valores de 0 à 9 de forma randomizada, se
 constrói todo o ferramental de aleatoriaedade.
 
  Tais funções não foram feito para serem gerados
 em grandes escalas randomizadas, apenas um 
 em escalas de milisegundos a geração massissa
 estocástica funciona bem, portanto, um algoritmo
 extremamente lento.
*/

// biblioteca Rust:
use std::time::SystemTime;

/* função efeitua um "lançamento de moeda" levando
 * em conta repetição de dezenas de vezes da medição
 * do relógio do sistema, isto após ter somado a 
 * medição várias vezes, após isso pega a unidade
 * do número medido porque é a parte mais volátil
 * do número, principalmente depois de somado 
 * várias vezes. */
fn lancamento() -> bool {
   // contador de tempo decorrido.
   let cronometro = SystemTime::now();
   let mut nanoseg:u64 = 0;

   /* soma todos tempos decorridos para que
    * unidades de tal valor, varie alucinadamente,
    * porque é com tal elemento que decidimos 
    * o valor estocástico gerado. */
   for _ in 1..=50 {
      let d = cronometro.elapsed().unwrap();
      nanoseg += d.as_nanos() as u64; 
   }

   // extraíndo unidade do número, pois é muito volátil.
   let unidade:char = {
      nanoseg
      .to_string()
      .pop()
      .unwrap()
   };
   /* metade dos "números" resultam em 'verdadeiro'
    * a outra o inverso. */
   match unidade {
      '0' => false,
      '1' => true,
      '2' => false,
      '3' => true,
      '4' => false,
      '5' => true,
      '6' => false,
      '7' => true,
      '8' => false,
      '9' => true,
      _ => 
         { panic!("não obtido um número!"); }
   }
}

/**
 compila todas funções em uma só, que poder emitir
 quaisquer tipos num só módulo, onde têm funções com
 nomes parecidos dos tipos, assim invokar tal função 
 parece o tipo acompanhado de "()". As funções no
 só chamam as funções acimas que já trabalham na parte
 de gerar tais coisas aleatórias, e algumas também
 implemenetam 'Range' que as funções acimas foram deixas
 de fora nisso. 
*/
pub mod sortear {
   use std::ops::RangeInclusive;

   /// gera um valor booleano randômico.
   pub fn bool() -> bool 
      { super::lancamento() }

   /// gera valor inteiro positivo de 8-bits(0 à 255).
   pub fn u8(intervalo:RangeInclusive<u8>) -> u8 {
      // apelidos para melhor legibilidade.
      let a = *intervalo.start();
      let b = *intervalo.end();

      /* se não estiver dentro do limite, sortear 
      * até que esteja. */
      let mut x:u8 = 0;
      // last-endian ...
      for e in 0u32..8u32 {
         // positivo como se fosse o um(1).
         if bool()   
            { x += 2u8.pow(e); }
      }

      // retorna número sorteado.
      if a > 0 && b <= u8::MAX
         { (x % ((b - a) + 1)) + a }
      else if a == 0 && b < u8::MAX
         { x % (b + 1) }
      else { x }
   }

   /* Agora para inteiros de 8-bits que permitem
   * números negativos. Mesmo esquema(retrocompatibilidade)
   * e implementação do 'Range' para delimitar
   * o sorteio. */
   pub fn i8(intervalo:RangeInclusive<i8>) -> i8 {
      // alias do interválo para propósito de codificação.
      let a = intervalo.start(); 
      let b = intervalo.end();

      // se é uma faixa positiva.
      if *a >= 0 && *b > 0 { 
         let a = *a as u8;
         let b = *b as u8;
         u8(a..=b) as i8 
      }
      // leva em conta a proporção de negativos/positivos.
      else if *a < 0 && *b >= 0 {
         // parte negativa.
         let _a:i8 = {
            if *a == -128
               { 127_i8 }
            else
               { a.abs() }
         };
         // porcentagem dos positivos.
         let p = (*b as f32) / (_a + *b) as f32;
         /* divisão dos ligados aos positivos e 
          * negativos, usando como bussola os 
          * duzentos e cinquenta seis valores do u8. */
         let meio:u8 = (p * 255.0) as u8;

         /* Se for do 'meio' computado para cima, então 
          * serão sorteados inteiros positivos. */
         if dbg!(u8(0..=255)) > dbg!(meio) 
            { i8(0..=*b) }
         // caso contrário, negativos apenas.
         else
            { (-1) * i8(0..=_a) -1 }
      } 
      /* intervalo negativo, então só negativos. Como
       * é basicamento o primeiro caso com valores
       * negativos, chama a função novamente, e negativa
       * o resultado. */
      else {
         (-1i8) * i8(b.abs()..=a.abs())
      }
   }

   /// gera um inteiro positivo de 16-bits randômico.
   pub fn u16(intervalo:RangeInclusive<u16>) -> u16 { 
      // acumulador de potências.
      let mut soma:u16 = 0;
      // quantia de bits.
      let mut qtd:u32 = 16;
      // apelidos:
      let a = *intervalo.start();
      let b = *intervalo.end();

      // supondo formatação big-endian.
      while qtd > 0 {
         // positivo como se fosse o um(1).
         if bool()   
            {soma += 2u16.pow(qtd-1); }
         // somar zero é irrelevante, mas fica aqui
         // para manter a lógica.
         else {}
         qtd -= 1;
      }

      // corrigindo nos intervalos.
      if a > 0 && b <= u16::MAX
         { (soma % ((b - a) + 1)) + a }
      else { soma }
   }

   /// gera um inteiro positivo de 32-bits randômico.
   pub fn u32(intervalo:RangeInclusive<u32>) -> u32 {
      // acumulador de potências.
      let mut soma:u32 = 0;
      // quantia de bits.
      let mut qtd:u32 = 32;
      // apelidos:
      let a = *intervalo.start();
      let b = *intervalo.end();
      // supondo formatação big-endian.
      while qtd > 0 {
         // positivo como se fosse o um(1).
         if bool()   
            {soma += 2_u32.pow(qtd-1); }
         // somar zero é irrelevante, mas fica aqui
         // para manter a lógica.
         else {}
         qtd -= 1;
      }

      // corrigindo nos intervalos.
      if a > 0 && b <= u32::MAX
         { (soma % ((b - a) + 1)) + a }
      else { soma }
   }
   
   /// gera um inteiro positivo de 64-bits randômico.
   pub fn u64(intervalo:RangeInclusive<u64>) -> u64 {
      // acumulador de potências.
      let mut soma:u64 = 0;
      // quantia de bits.
      let mut qtd:u32 = 64;
      // apelidos:
      let a = *intervalo.start();
      let b = *intervalo.end();
      // supondo formatação big-endian.
      while qtd > 0 {
         // positivo como se fosse o um(1).
         if bool()   
            {soma += 2u64.pow(qtd-1); }
         qtd -= 1;
      }
      // corrigindo nos intervalos.
      if a > 0 && b <= u64::MAX
         { (soma % ((b - a) + 1)) + a }
      else { soma }
   }
}

