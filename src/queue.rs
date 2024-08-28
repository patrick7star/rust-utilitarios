/** Este módulo terá vários tipos de filas, array, circular, e ligada. Todas
 serão feitas com alocamento manual de memória.
 */

use std::ptr::{null_mut};
use std::ops::{Drop};

struct Nodulo<A> {
   // Dado encapsulado.
   pub dado: Option<A>,

   // Próximo nódulo que é apontado.
   pub seta: *mut Nodulo<A>
}

impl<A> Nodulo<A> {
   pub fn aloca(dado: A, endereco: *mut Nodulo<A>) -> Self
      { Self { dado: Some(dado), seta: endereco } }
   
   pub fn desaloca(pointer: *mut Nodulo<A>) 
      { drop(unsafe { Box::from_raw(pointer)}); }
}

pub struct FilaCircular<B> {
   // Marca o final da fila-circular.
   fim: *mut Nodulo<B>,

   // Total de itens que estão contido na 'fila'.
   quantia: usize,
}

impl<B> FilaCircular<B> {

   pub fn nova() -> Self 
      { Self { fim: null_mut(), quantia: 0 } }

   pub fn comprimento(&self) -> usize
      { self.quantia }
   
   pub fn vazia(&self) -> bool
      { self.quantia == 0 }
   
   pub fn adiciona(&mut self, dado: B)
   {
      /* Criando um 'nó', encapsulando-o numa 'Box' para alocar na 'heap', 
       * então obtendo o 'raw pointer' dele.
       */
      let no = Nodulo::aloca(dado, null_mut()) ;
      let caixa = Box::new(no);
      let pointer = Box::into_raw(caixa);

      unsafe {
         if self.vazia() {
            (*pointer).seta = pointer;
         } else {
            // Implementação do livro...
            (*pointer).seta = (*self.fim).seta;
            (*self.fim).seta = pointer;
         }
      }
      self.fim = pointer;
      self.quantia += 1;
   }

   pub fn remove(&mut self) -> Option<B> 
   {
      if self.vazia()
         { return None; }
      
      let salvo = unsafe { (*self.fim).seta };

      if self.quantia == 1
         { self.fim = null_mut(); }
      else
         { unsafe { (*self.fim).seta = (*salvo).seta; } }
      // Descontabilizando...
      self.quantia -= 1;

      unsafe { 
         let remocao = (*salvo).dado.take(); 
         Nodulo::desaloca(salvo);
         remocao
      }
   }

   /// Obtém atual item no fim da 'fila-circular', se houver algo, é claro.
   pub fn obtem(&self) -> Option<&B>
   {
      if self.vazia()
         { return None; }
      
      unsafe { (*self.fim).dado.as_ref() }
   }
}

impl<C> Drop for FilaCircular<C> 
{
   fn drop(&mut self)
   {
      let mut contagem = 0;

      if cfg!(debug_assertions)
         { print!("Começado desalocação da 'FC'..."); }

      while let Some(_) = self.remove() { contagem += 1; }
      if cfg!(debug_assertions)
         { print!("todos {} itens desalocados...", contagem); }
      assert!(self.vazia());

      if cfg!(debug_assertions)
         { println!("desalocada com sucesso a fila."); }
   }
}

use std::fmt::{Display, Result as ResultFMT, Formatter};

impl<F: Display> Display for FilaCircular<F>
{
   fn fmt(&self, output: &mut Formatter) -> ResultFMT
   {
      write!(output, "Fila-Circular({}): {{...", self.quantia).unwrap();
      let mut todos_itens = self.iter();

      while let Some(item) = todos_itens.next()
         { write!(output, "{}, ", item).unwrap(); }

      write!(output, "...}}")
   }
}

pub struct Iter<'a, D> where D: 'a 
{
   // Próximo nó a iterar.
   cursor: *mut Nodulo<D>,

   // Atual dado a ser iterado.
   dado: Option<&'a D>,

   // Onde no contínuo de itens está a iteração.
   posicao: usize,

   // Total de itens na criação do iterador.
   total: usize
}

impl<'a, D> FilaCircular<D> 
{
   pub fn iter(&'a self) -> Iter<'a, D>
   {
      Iter { 
         dado: unsafe { (*self.fim).dado.as_ref() },
         cursor: self.fim, 
         posicao: 0, 
         total: self.quantia 
      }
   }
}

impl<'e, E> Iterator for Iter<'e, E>
{
   type Item = &'e E;

   fn next(&mut self) -> Option<Self::Item>
   {

      if self.posicao == self.total
         { return None; }
         
      let atual_no = self.cursor ;
      let atual_dt = self.dado;
      
      // Avançando para o próximo, e já capturando o dado também...
      unsafe {
         self.cursor = (*atual_no).seta;
         self.dado = (*self.cursor).dado.as_ref();
      }
      self.posicao += 1;

      // Retornando atual item...
      return atual_dt;
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use std::fmt::{Display, Debug};
   use std::thread;
   use std::time::{Duration};

   fn visualizacao_do_fc<C>(queue: &mut FilaCircular<C>)
     where C: Display + Debug 
   {
      let mut current: *mut Nodulo<C> = queue.fim;
      let mut total = queue.quantia;

      while total > 0
      {
         let obj = unsafe { (*current).dado.as_ref() };
         println!(" >>> {}", obj.unwrap());
         current = unsafe { (*current).seta };
         total -= 1;
      }
   }

   #[test]
   fn apenas_instanciacao_da_fila_circular() 
   {
      let mut queue = FilaCircular::<i8>::nova();

      assert!(queue.vazia());
      queue.adiciona(-3);
      queue.adiciona(-17);
      queue.adiciona(-2);
      queue.adiciona(11);
      queue.adiciona(5);
      assert_eq!(queue.comprimento(), 5);

      visualizacao_do_fc(&mut queue);

      println!("Removendo todos eles...");
      while let Some(deletado) = queue.remove()
         { println!("\t---> {:?}", deletado); }
   }

   #[test]
   fn verificacao_de_memory_leak() 
   {
      let mut queue = FilaCircular::<char>::nova();

      println!("Inserção para consumir memória...");
      for _ in 1..=80000
         { queue.adiciona('M'); }
      thread::sleep(Duration::from_secs(5));

      println!("Remoção de todas adições...");
      while let Some(_) = queue.remove() {}
      thread::sleep(Duration::from_secs(5));

      println!("Reinserindo para ver se o descontrutor não vaza também...");
      for _ in 1..=80000
         { queue.adiciona('B'); }
      thread::sleep(Duration::from_secs(5));
      drop(queue);
      println!("Destruída! Favor, checar no monitor do sistema.");
      thread::sleep(Duration::from_secs(5));
   }

   #[test]
   fn verificacao_da_iteracao_da_estrutura()
   {
      let mut queue = FilaCircular::<char>::nova();

      for letra in 65..(65 + 26)
         { queue.adiciona(char::from_u32(letra).unwrap()); }

      let mut iterador = queue.iter();

      println!("Produto da iteração:");
      while let Some(letter) = iterador.next()
         { println!("\t>> {}", letter); }
      
      drop(queue);
      println!("O fim foi alcançado.");
   }

   #[test]
   fn metodo_de_obtencao_da_ponta()
   {
      let mut queue = FilaCircular::<char>::nova();

      assert_eq!(queue.obtem(), None);
      queue.adiciona(char::from_u32(97).unwrap());

      println!("{}", queue);
      println!("Único dado: {:?}", queue.obtem());

      for letra in 97..(97 + 26) { 
         queue.adiciona(char::from_u32(letra).unwrap()); 
         println!("Obtido:{:?}\n{}\n", queue.obtem(), queue);
      }

   }
}