// Biblioteca padrão do Rust:
use utilitarios::arvore::
   {arvore as arvore_simples, arvore_a as arvore_personalizada};
use std::path::{Path};
use std::collections::{VecDeque};
use std::path::{PathBuf};
use std::ffi::{c_void, c_int, c_char, CString, CStr};
use std::ptr::{null};
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
   let exclusion = unsafe {
      if exclusao == EXCLUSAO_OFF { None }
      else {
         let mut fila = VecDeque::new();

         // Transformando 'raw pointers of char' para strings.
         for _ in 1..=n {
            let ptr =  *exclusao;
            let cstr = CStr::from_ptr(ptr); 
            let str = cstr.to_str().unwrap();
            fila.push_back(str);
         }
         Some(fila)
      }
   };

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
         CString::new("utilitarios-em-c").unwrap(),
         CString::new("build").unwrap(),
         CString::new("bin").unwrap(),
         CString::new("praticas").unwrap(),
         CString::new("lib").unwrap()
      ];
      let minhas_exclusoes = [
         targets[0].as_ptr() as *const c_char, 
         targets[1].as_ptr() as *const c_char, 
         targets[2].as_ptr() as *const c_char,
         targets[3].as_ptr() as *const c_char, 
         targets[4].as_ptr() as *const c_char
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
}
