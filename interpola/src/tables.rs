// Biblioteca padrão do Rust:
use std::ffi::{c_void, c_char, CString};
use std::ptr::{self, null_mut};
use std::collections::{VecDeque};
use std::mem::{transmute};
// Outros módulos do caixote.
use utilitarios::tabelas::{Coluna as ColunaOriginal, Tabela as TabelaRust};
use crate::tree::{transforma_multiarray_char_to_queue_cstring, strcpy};

// Apelidos de lista genérica.
#[allow(unused)]
type RolGenericoArray = *const *const c_void;
#[allow(unused)]
type RolTArray = RolGenericoArray;
type RolCharArray = *const *const c_char;
type PtrChar = *const c_char;

/* == == == == == == == == == == == == == == == == == == == == == == == == =
 *                      Coluna e seus Métodos
 * == == == == == == == == == == == == == == == == == == == == == == == = */
#[repr(C)]
pub struct Coluna 
{
   /* Instância original do Rust, que se pode realmente aplicar nas funções 
    * internas do embrulho. Abaixo um valor lógico dizendo se tal instância
    * já foi liberada ou não.
    */
   instancia: *mut ColunaOriginal<String>,
   pub liberada: bool,


   // Campos que serão visíveis ao C:
   rotulo: PtrChar,
   rol: RolCharArray,
   length: i32,
   largura: usize
}

fn fila_cstring_to_vec_string(mut input: VecDeque<CString>) -> Vec<String>
{
   let n = input.len();
   let mut output = Vec::with_capacity(n);

   while !input.is_empty() {
      if let Some(data) = input.pop_front()
         { output.push(data.into_string().unwrap()); }
   }
   output
}

fn char_array_to_string(input: PtrChar) -> String
{
   let output_a = strcpy(input);
   let output_b = unsafe { CString::from_raw(output_a) };

   output_b.into_string().unwrap()
}

#[no_mangle]
pub extern "C" fn nova_col(rotulo: PtrChar, dados: RolCharArray, n: i32) 
  -> Coluna
{
   let conversao = transforma_multiarray_char_to_queue_cstring;
   let rol_cstring = conversao(dados, n);
   let rotulo_str = char_array_to_string(rotulo);
   let rol_string = fila_cstring_to_vec_string(rol_cstring);
   let coluna_rust = ColunaOriginal::nova(&rotulo_str, rol_string); 
   let largura = coluna_rust.largura();
   let instancia = Box::into_raw(Box::new(coluna_rust));

   Coluna { instancia,  liberada: false, rotulo, rol: dados, length: n, largura}
}

#[no_mangle]
pub extern "C" fn linhas_col(obj: Coluna) -> usize
   { unsafe { (*obj.instancia).linhas() } }

#[no_mangle]
pub extern "C" fn largura_col(obj: Coluna) -> usize
   { unsafe { (*obj.instancia).largura() } }

#[no_mangle]
pub extern "C" fn free_col(obj:*mut Coluna) {
   unsafe {
      // Se já foi liberada posteriormente, apenas abandona a função daqui.
      if (*obj).liberada { return (); }

      let valor = (*obj).instancia;
      let _self = Box::from_raw(valor);

      // Apenas anulando a referência.
      (*obj).instancia = null_mut::<ColunaOriginal<String>>();
      // O box será liberado aqui no final do escopo.
      // drop(_self)
      (*obj).liberada = true;
   }
}

#[no_mangle]
pub extern "C" fn debug_col(obj: Coluna) 
  { println!("{:#?}", unsafe {(*obj.instancia).clone()}); }

#[no_mangle]
pub extern "C" fn print_col(obj: Coluna) 
  { println!("{}", unsafe {(*obj.instancia).clone()}); }

/* == == == == == == == == == == == == == == == == == == == == == == == == =
 *                      Tabela e seus Métodos
 *
 * Obs.: Eu apenas coloquei um contador de instâncias na estrutura, porque a 
 * parece que a interpolação não aceita 'opaque structs', então apenas para
 * não deixar um 'ponteiro crú' lá na declaração em C, coloquei uma 
 * referência constante do contador de instâncias.
 * == == == == == == == == == == == == == == == == == == == == == == == = */
#[repr(C)]
struct Tabela { instancia: *mut TabelaRust, contador: *const u32 }
// Contador da quantidade de instâncias de tabelas iniciadas neste processo.
static mut CONTAGEM_TABELAS: u32 = 0;

#[no_mangle]
pub extern "C" fn nova_table(maximo_de_tela: bool) -> Tabela {
   let ativado = maximo_de_tela;
   let tabela = TabelaRust::nova(ativado);
   // Alocando ele na heap ...
   let objeto = Box::new(tabela);

   unsafe {
      let count = CONTAGEM_TABELAS;
      let referencia = &CONTAGEM_TABELAS;

      // Incrementando nova instância da tabela iniciadas.
      CONTAGEM_TABELAS += 1; 

      Tabela { 
         instancia: Box::into_raw(objeto), 
         contador: transmute::<&u32, *const u32>(referencia)
      }
   }
}

#[no_mangle]
pub extern "C" fn adiciona_table(obj: *mut Tabela, item: Coluna) 
   { unsafe { (*(*obj).instancia) += (*item.instancia).clone(); } }

#[no_mangle]
pub extern "C" fn free_table(obj: *mut Tabela) { 
   unsafe { let _= Box::from_raw((*obj).instancia); } 
}

#[no_mangle]
pub extern "C" fn print_table(obj: *mut Tabela) { 
   let table: Tabela;
   let selfe: TabelaRust;

   unsafe {
      table = ptr::read(obj);
      selfe = ptr::read(table.instancia);

      println!("{}", selfe);
   }
}
