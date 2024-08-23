/* Não existe uma pilha official na coleção do Rust, então aqui será 
 * implementado uma. Esta versão aqui é estática(usando array).
 * Poderia implementar um "embrulho" da deque ou vetor, entretanto, farei
 * usando "raw pointer", mesmo que em C.
 */

use std::alloc::{Layout, alloc};
use std::mem::{transmute};
#[allow(unused_imports)]
use std::ptr::{null_mut, replace};
use std::fmt::{Display, Result as Resultfmt, Formatter};

// Valor padrão para alocação.
const CAPACIDADE: usize = 50;
// Apelido da estrutura em Inglês:
pub type StackArray<M> = PilhaArray<M>;

pub struct PilhaArray<T> {
   // Todos itens colocados lado-a-lado numa array. O topo dela é a ponta
   // móvel direita.
   array: *mut T,

   // Total de itens que ela pode armazenar inicialmente.
   capacidade: usize,

   // Total de itens. Também serve como posição do 'topo' da 'pilha'.
   quantia: usize
}

impl<T> PilhaArray<T> {
/* A pilha será aplicável para cada tipo de dado, como o mínimo de 
 * requisitos possíveis. Pensei em colocar como 'trait bound' a visualização
 * do dado, mas não neste lote de implementação. */
   pub fn nova() -> Self {
      let modo = Layout::array::<T>(CAPACIDADE);
      let raw_array = unsafe { alloc(modo.unwrap()) };

      Self {
         array: raw_array as *mut T,
         capacidade: CAPACIDADE,
         quantia: 0
      }
   }

   pub fn comprimento(&self) -> usize
      { self.quantia }

   pub fn vazia(&self) -> bool
      { self.quantia == 0 }

   pub fn coloca(&mut self, dado: T) -> bool {
      let indice = self.quantia;

      if indice >= self.capacidade
      /* Por enquanto, apenas nega a inserção na pilha. */
         { return false; }

      unsafe {
         let ptr = self.array.add(indice);
         std::ptr::write(ptr, dado);
      }
      self.quantia += 1;

      // Sem qualquer problema, apenas confirma inserção.
      true
   }

   pub fn retira(&mut self) -> Option<T> {
      if self.vazia()
         { return None; }

      // Posição do topo:
      let tp = self.quantia - 1;
      let dado: T;

      unsafe {
         let ptr = self.array.add(tp);
         dado = ptr.read();
      } 

      // Desconta remoção, e então envia dado encapsulado.
      self.quantia -= 1;
      Some(dado)
   }

   pub fn topo(&self) -> Option<&T> {
      if self.vazia()
      // Nada será retornado se estiver vázio.
         { return None; }

      let indice = self.quantia - 1;
      let top: &T;

      unsafe {
         let ptr = self.array.add(indice);
         top = transmute::<*mut T, &T>(ptr);         
      }
      Some(top)
   }
}

impl<T: Display> Display for PilhaArray<T> {
   fn fmt(&self, output: &mut Formatter<'_>) -> Resultfmt   
   {
      const LIMITE_DE_IMPRESSAO: usize = 5000;
      let total = self.quantia;

      if self.vazia()
         { return write!(output, "Pilha: {{}}"); }
      else if total > LIMITE_DE_IMPRESSAO
         { panic!("Mais de {} itens, impossível imprimir!", total); }

      let itemptr: *mut T;
      let dadofmt: &T;
      let mut i = 1;

      unsafe {
         itemptr = self.array.add(total - i);
         dadofmt = transmute::<*mut T, &T>(itemptr);
      }
      // O primeiro item já foi iterado.
      i += 1;

      write!(output, "Pilha({}): [{}]-{{", self.quantia, dadofmt).unwrap();

      while i <= total {
         let itemptr: *mut T;
         let dadofmt: &T;

         unsafe {
            itemptr = self.array.add(total - i);
            dadofmt = transmute::<*mut T, &T>(itemptr);
         }

         if i == total
            { write!(output , "{}", dadofmt).unwrap(); }
         else
            { write!(output , "{}, ", dadofmt).unwrap(); }
         i += 1;
      }
      write!(output, "}}\n")
   }
}

impl<T> PilhaArray<T> {
/* Interface dos métodos em Inglês. A maioria dos métodos chamado acima
 * são mais conhecidos por seus nomes na língua anglofona. Para não confudir
 * outros que usam isso, vamos embrulhar tais chamadas com tais nomes. */
   pub fn new() -> Self { PilhaArray::nova() }

   pub fn empty(&self) -> bool { self.vazia() }

   pub fn push(&mut self, dado: T) -> bool { self.coloca(dado) }

   pub fn pop(&mut self) -> Option<T> { self.retira() }

   pub fn top(&self) -> Option<&T> { self.topo() }
}

#[cfg(test)]
mod tests {
   use super::*;

   fn visualiza_raw_array<U: Display>(inicio: *mut U, q: usize)
   {
      println!("Visualizando array interna:");

      for k in 0..q 
      {
         let a: &U;
         unsafe {
            a = std::mem::transmute::<*mut U, &U>(inicio.add(k));
         }
         println!(" > {}", a);
      }
   }

   #[test]
   fn simples_teste() {
      println!("Alocando uma nova pilha-array...");

      let mut stack: PilhaArray<char> = PilhaArray::nova();

      assert!(stack.vazia());
      println!("Adição de alguns itens:");
      stack.coloca('a');
      stack.coloca('A');
      assert_eq!(stack.comprimento(), 2);

      visualiza_raw_array(stack.array, 2);

      let topo = stack.topo();
      assert_eq!(topo, Some(&'A'));
      println!("topo: {}", topo.unwrap());

      for _char in ['b', 'B', 'C', 'c', 'd', 'D'] { 
         stack.coloca(_char); 
         println!("topo: {}", stack.topo().unwrap());
      }

      println!("\nEsvaziando a pilha:");
      println!("{}", stack);

      while let Some(dt) = stack.retira()
         { println!("\n\tRemovido:{}\n{}", dt, stack); }
   }
}
   


