// Biblioteca padrão do Rust:
use utilitarios::arvore::
   {arvore as arvore_simples, arvore_a as arvore_personalizada};
use std::path::{Path};
use std::collections::{VecDeque};
use std::path::{PathBuf};
use std::ffi::{c_void, c_int, c_char, CString, CStr};
use std::ptr::{null};
use std::mem::{size_of};
// Outros módulos do caixote.
use crate::written_numerals::{aloca_cstring_na_heap};

// Valor que desativa várias opções da 'árvore config', porque não há 
// enum options in C, então se usa constantes no lugar:
pub const MAX_DEPTH_OFF: c_int = -1;
pub const EXCLUSAO_OFF: *const *const i8 = null::<*const i8>();
// Apelido para booleano do C:
#[allow(non_camel_case_types)]
type c_bool = u8;

extern "C" {
   fn abort() -> c_void;
   fn perror(s: *const c_char) -> c_void;

   fn AmostraDeFrutas() -> *const *const c_char;
   fn AmostraDeFrutasSize() -> i32;
}

/* == == == == == == == == == == == == == == == == == == == == == == == == =
 *                   Funções de Geração Gráfica de Árvore
 * == == == == == == == == == == == == == == == == == == == == == == == = */
// Emite erro se o caminho não for válido. Portanto, interrompe o programa.
fn emite_erro_por_caminho_invalido(caminho: &Path) {
   let e_tudo_menos_diretorio = !caminho.is_dir();
   let caminho_nao_existe = !caminho.exists();

   /* Isso só será acionado, apenas se o caminho não for um diretório, ou
    * também ele não existir. */
   if e_tudo_menos_diretorio || caminho_nao_existe { unsafe {
      let msg = "um não diretório foi posto.";
      let ptr = msg.as_ptr() as *const c_char;

      perror(ptr);
      abort(); 
   }}
}

/* Transforma uma array crua de caractéres num caminho padrão do Rust. */
fn rawchar_to_path(caminho: *const c_char) -> PathBuf {
   let path_a  = unsafe { CStr::from_ptr(caminho) };
   let path_b  = path_a.to_str().unwrap();

   Path::new(path_b).to_path_buf()
}

#[no_mangle]
pub extern "C" fn gera_arvore(caminho: *const c_char, visivel: c_bool)
  -> *mut c_char
{
   let caminho = rawchar_to_path(caminho);

   // Para a aplicação se o caminho for inválido.
   // erro_por_caminho_nao_ser_dir(caminho);

   let show_files: bool = visivel != 0;
   let result = arvore_simples(&caminho, show_files);
   let result = CString::new(result).unwrap();

   aloca_cstring_na_heap(&result)
}

use crate::written_numerals::{strlen};
use std::alloc::{Layout, alloc};
use std::ptr::{copy_nonoverlapping};

fn strcpy(array: *const c_char) -> *mut c_char
{
   let t = strlen(array);
   let sz = size_of::<c_char>();
   let n = (t + 1) * sz;
   let modelo = Layout::array::<c_char>(t).unwrap(); 
   let bloco_de_bytes = unsafe { alloc(modelo) };
   let bloco_ptr = bloco_de_bytes as *mut c_char;

   unsafe { 
      copy_nonoverlapping(array, bloco_ptr, n); 
      bloco_de_bytes as *mut c_char
   }
}

fn transforma_multiarray_char_to_queue_cstring
  (multiarray: *const *const c_char, n: i32) -> VecDeque<CString>
{
/* Tentarei uma abordagem nova. Transformarei em bytes, pegarei cada 
 * caractére nulo, então copiarei os trechos entre bytes nulos. */
   let mut fila_de_cstring: VecDeque<CString>; 
   let mut pointer = multiarray;

   fila_de_cstring = VecDeque::with_capacity(n as usize);
   for p in 0..(n as usize) {
      unsafe { 
         let copia = strcpy(*pointer);
         let cstring = CString::from_raw(copia);
         fila_de_cstring.push_back(cstring); 
         pointer = multiarray.add(p);
      }
   }
   fila_de_cstring
}

#[no_mangle]
pub extern "C" fn gera_arvore_config(caminho: *const c_char, visivel: c_bool,
  profundidade: c_int, exclusao: *const *const c_char, n: c_int)
  -> *mut c_char
{
/* Pega todos parâmetros, tais que são compatíveis com C, então transforma
 * na forma Rusteana, faz a chamada usando tais, obtém o resultado string,
 * transforma tal para uma forma aceitável pro C, então retorna-o.
 * O resultado final é uma raw array de caractéres, alocadas na heap, 
 * portanto, fica a cargo do */
   let caminho = rawchar_to_path(caminho);
   let show_files = visivel != 0;
   let depth = {
      if profundidade < MAX_DEPTH_OFF
         { None }
      else if MAX_DEPTH_OFF == profundidade
         { None}
      else
         { Some(profundidade as usize) }
   };
   let exclusion_cstring = {
      if exclusao == EXCLUSAO_OFF { None }
      else {
         #[allow(non_snake_case)]
         let tMcTqC = transforma_multiarray_char_to_queue_cstring;
         Some(tMcTqC(exclusao, n))
      }
   };
   let exclusion_string = {
      if let Some(mut colecao) = exclusion_cstring {
         Some(
            colecao.drain(..)
            .map(|cstr| cstr.into_string().unwrap())
            .collect::<VecDeque<String>>()
         )
      } else { None}
   };
   let exclusion_str = {
      if let Some(ref colecao) = exclusion_string {
         Some(
            colecao.iter().map(|s| s.as_str())
            .collect::<VecDeque<&str>>()
          )
      } else { None }
   };
   let exclusion = exclusion_str;

   // Para a aplicação se o caminho for inválido.
   // erro_por_caminho_nao_ser_dir(caminho);

   let result = arvore_personalizada(&caminho, show_files, depth, exclusion);
   let result = CString::new(result).unwrap();

   aloca_cstring_na_heap(&result)
}

/* == == == == == == == == == == == == == == == == == == == == == == == == =
 *                            Testes Unitários
 * == == == == == == == == == == == == == == == == == == == == == == == = */
#[cfg(test)]
mod tests {
   use super::*;
   use std::alloc::{dealloc, Layout};
   use std::mem::{transmute};

   fn visualiza_raw_string(array: *const c_char) {
      let cstring = unsafe { CStr::from_ptr(array) };
      let string = cstring.to_string_lossy();
      println!("array: {}", string);
   }

   #[test]
   fn funcao_de_geracao_de_arvore_simples() {
      let caminho = Path::new(env!("C_CODES"));
      let caminho_c = caminho.to_str().unwrap().as_ptr() as *const i8;

      unsafe {
         let visivel = true;
         let output = gera_arvore(caminho_c, visivel as u8); 
         let fmt = CStr::from_ptr(output).to_str().unwrap();
         let n = fmt.len();
         let memoria = Layout::array::<c_char>(n);

         println!("{}", fmt);
         dealloc(output as *mut u8, memoria.unwrap());
      }
   }

   #[test]
   fn geracao_de_arvore_personalizada_maxima_profundidade() {
      let caminho_c = CString::new(env!("CCODES")).unwrap();
      let caminho_str = caminho_c.to_str().unwrap();
      let caminho = Path::new(caminho_str);
      let caminho_ptr = caminho_str.as_ptr() as *const i8;

      println!("caminho em Rust: {}\n", caminho.display());
      print!("caminho em C: "); visualiza_raw_string(caminho_ptr);
      print!("\n\n");

      unsafe {
         println!("Acionando limite de profundidade ...");
         let output = gera_arvore_config
            (caminho_ptr, true as u8, 2, EXCLUSAO_OFF, 0); 
         let s = CStr::from_ptr(output);
         let fmt = &s.to_str().unwrap();
         let n = fmt.len();
         let memoria = Layout::array::<c_char>(n);

         println!("{}", fmt);
         dealloc(output as *mut u8, memoria.unwrap());
      }
   }

   #[test]
   fn geracao_de_arvore_personalizada_exclusoes() {
      let caminho_c = CString::new(env!("CCODES")).unwrap();
      let caminho_str = caminho_c.to_str().unwrap();
      let caminho_ptr = caminho_str.as_ptr() as *const i8;

      let targets = [
         CString::new("bin").unwrap(),
         CString::new("build").unwrap(),
         CString::new("lib").unwrap(),
         CString::new("tests").unwrap()
      ];
      let minhas_exclusoes = [
         targets[0].as_ptr() as *const c_char, 
         targets[1].as_ptr() as *const c_char, 
         targets[2].as_ptr() as *const c_char,
         targets[3].as_ptr() as *const c_char, 
      ];
      let lista_ptr = minhas_exclusoes.as_ptr() as *const *const c_char;

      unsafe {
         println!("Acionando lista de exclusão ...");
         let output = gera_arvore_config
            (caminho_ptr, true as u8, MAX_DEPTH_OFF, lista_ptr, 5); 
         let s = CStr::from_ptr(output);
         let fmt = &s.to_str().unwrap();
         let n = fmt.len();
         let memoria = Layout::array::<c_char>(n);

         println!("{}", fmt);
         dealloc(output as *mut u8, memoria.unwrap());
      }

      /* Observação: com uma avaliação manual, o resultado foi que funcionou
       * perfeitamente em suprimir todos diretórios listados. */
   }

   #[test]
   fn varrendo_caracteres_de_cstring_de_arrays()
   {
      let data = [
         CString::new("processamento").unwrap(), 
         CString::new("vermelho").unwrap(),
         CString::new("queijo").unwrap(), 
         CString::new("trêm").unwrap(),
         CString::new("ármario").unwrap()
      ];
      let data_ptr = [
         data[0].as_ptr(), data[1].as_ptr(), data[2].as_ptr(), 
         data[3].as_ptr(), data[4].as_ptr()
      ];
      let lista: *const *const c_char;
      let mut ptr: *const *const c_char;

      unsafe { 
         lista = transmute::<&[*const c_char; 5], _>(&data_ptr); 
         ptr = lista;

         for _ in 1..=data_ptr.len() { 
            visualiza_raw_string(*ptr); 
            ptr = ptr.add(1);
         }
      }
   }

   fn visualiza_multiarray_char(lista: *const *const c_char, n: i32)
   {
      let mut pointer = lista;
      let sz = size_of::<*const c_char>() as i32;

      for p in 0..n { unsafe {
         visualiza_raw_string(*pointer as *const i8);
         pointer = lista.offset(p as isize);
      }}
   }

   #[test]
   fn convertendo_multiarray_char() {
      let entrada = unsafe {  super::AmostraDeFrutas() };
      let n: i32 = unsafe { super::AmostraDeFrutasSize() };

      println!("Dados de entrada(veio de um código C):");
      visualiza_multiarray_char(entrada, n);

      let transforma = transforma_multiarray_char_to_queue_cstring;
      let saida = transforma(entrada, n);

      println!("\nApós conversão:");
      for item in saida.iter() { println!("{item:?}"); }
      /*

      assert_eq!(saida, data); */
   }
}
