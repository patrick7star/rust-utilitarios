
use crate::tela::mudanca::{CAPACIDADE};
// renomea tipo estático.
type Str = &'static str;

pub struct Pilha<A> {
/* Array que armazena os objetos Para tal, com inicialização, no
 * mínimo em vinte, mais é desnecessário */
   array: Vec<A>,
   tamanho: u8,
}

impl <A>Pilha<A> {
// método construtor.
   pub fn nova() -> Self {
      let capacidade: usize;
      capacidade = CAPACIDADE as usize;
      Pilha {
         array: Vec::with_capacity(capacidade),
         tamanho: 0,
      }
   }
}

impl <A>Pilha<A> {
// operações que a pilha realiza:
   // verifica se a pilha está vázia.
   pub fn vazia(&self) -> bool
      { self.tamanho == 0 }

   // empilha um item na pilha se não estiver
   // atigindo sua capacidade máxima.
   pub fn empilha(&mut self, item:A) -> Result<(), Str> {
      if self.tamanho == CAPACIDADE
         { return Err("só é possível empilhar isso aí."); }

      self.array.push(item);
      // contabiliza o item adicionado.
      self.tamanho += 1;
      return Ok(());
   }

   // remove um item do topo, se houver algum.
   pub fn desempilha(&mut self) -> Option<A> {
      if self.vazia()
         { return None; }
      //descontabiliza item retirado.
      self.tamanho -= 1;
      return self.array.pop();
   }
}
