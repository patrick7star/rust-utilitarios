extern crate utilitarios;
use utilitarios::porextenso::{escreve_por_extenso};
// use porextenso::{escreve_por_extenso};
use std::ffi::{c_ulonglong, c_char, CString, CStr};
use std::mem::{size_of};
use std::alloc::{alloc, Layout};
use std::ptr::{copy_nonoverlapping};

const STRING_NULA: &'static str = "\0\0\0";


/* == == == == == == == == == == == == == == == == == == == == == == == == =
 *                         Utilitários Genéricos
 * == == == == == == == == == == == == == == == == == == == == == == == = */
pub fn strlen(mut pointer: *const c_char) -> usize {
/* Igual o 'strlen' do C, retorna o número de bytes, sem contar o caractére
 * nulo. */
   let mut contador = 0;
   let caractere_nulo: i8 = 0x0;
   let sz = size_of::<i8>() as isize;

   unsafe {
      while *pointer != caractere_nulo
      {
         pointer = pointer.offset(sz);
         contador += 1;
      }
   }
   contador
}

fn strlen_cstr(str: &CStr) -> usize 
// O mesmo que acima, porém para CStr do Rust.
   { strlen(str.as_ptr()) }

pub fn aloca_cstring_na_heap(string: &CStr) -> *mut c_char
{
   let t = strlen_cstr(string);
   let src = string.as_ptr();
   let tipo = Layout::array::<c_char>(t).unwrap();
   let array: *mut i8;
   let sz = size_of::<u8>();

   unsafe {
      let bloco = alloc(tipo);
      array = bloco as *mut i8;
      copy_nonoverlapping(src, array, t*sz);
   }
   // Retornando array alocada e copiada.
   array
}

/* == == == == == == == == == == == == == == == == == == == == == == == == =
 *                   Geração de Número por Extenso 
 * == == == == == == == == == == == == == == == == == == == == == == == = */
fn escreve_por_extenso_numa_cstring(numero: c_ulonglong) -> CString
{
/* Transforma string normalmente, em caso de erro, apenas retorna uma 
 * CString contendo apenas caractéres núlos, ou seja, retorna NULL. */
   match escreve_por_extenso(numero) {
      Ok(texto) => {
         CString::new(texto).unwrap()
      } Err(_) => {
         CString::new(STRING_NULA).unwrap()
      }
   }
}

#[no_mangle]
pub extern "C" fn escrita_por_extenso(numero: c_ulonglong) -> *mut c_char
{ 
   let string = escreve_por_extenso_numa_cstring(numero);
   
   /* Copia os bytes da CString. Converte numa array de caractéres, com o
    * caractére nulo, do tipo C. A põe na heap, então a retorna.*/
   aloca_cstring_na_heap(&string) 
}


#[cfg(test)]
mod tests {
   use std::alloc::{dealloc};
   use super::*;

   fn visualiza_raw_string(array: *const c_char) {
      let cstring = unsafe { CStr::from_ptr(array) };
      let string = cstring.to_string_lossy();
      println!("array: {}", string);
   }

   #[test]
   fn funcao_de_escrita_por_extenso() {
      let num = 653;
      let out = escrita_por_extenso(num);
      let n = strlen(out);
      let tipo = Layout::array::<c_char>(n).unwrap();
      visualiza_raw_string(out);
      unsafe { dealloc(out as *mut u8, tipo) };
   }
}
