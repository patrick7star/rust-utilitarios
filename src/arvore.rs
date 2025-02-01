/*! 
 # Desenho dos diretórios em árvore
   Fazer uma árvore para visualização de forma organizada, ramificando seus 
 arquivos e sub-diretórios. Ao acionar a função pode-se escolher tanto se 
 quer apenas a visualiazão dos sub-diretórios, como também dos seus arquivos.
*/
// biblioteca padrão do Rust.
use std::fs::read_dir;
use std::path::Path;
use std::io::{Result};
use std::collections::{VecDeque};

// do próprio caixote:
use constroi_simbolos::{
   matriciar_string, 
   matriz_para_string
};
// partes "externas" deste módulo:
mod constroi_simbolos;
mod utilidades;
use crate::terminal_dimensao::{dimensao, Largura};
pub use utilidades::{listagem, ramifica_caminho};


// tipos de galhos:
/// galho do tipo horizontal.
const GALHO_H:char = '\u{2500}';
/// galho vertical.
const GALHO_V:char = '\u{2502}';
/// galho conector vertical e horizontal.
const GALHO_VH:char = '\u{2570}';
/// conector entre dois verticais e um horizontal.
const GALHO_VHV:char = '\u{251c}';
/* Caso uma string exceda a tela do terminal a função vai reduzi-lá e 
 * implicitar que tal string é mais extensa, continua... */
static mut JA_COMPUTADO: bool = false;
static mut LARGURA: usize = u16::MAX as usize;
// Apelidos para estruturas abaixo:
type Matriz = Vec<Vec<char>>;
// Espaço fixo do recuo.
const RECUO: usize = 3;


/* Escreva toda uma trilha, com ramificações de sub-diretórios e arquivos, 
 * dado uma raiz principal. */
#[allow(dead_code)]
fn desenha_trilha(esboco:&mut String, caminho:&Path , pfd:&mut u8) {
   // navegando arquivos e diretórios.
   let lista_no_dir = match read_dir(caminho) {
       Ok(iterador) => iterador,
       Err(_) => panic!("diretório erro:\"{:#?}\"", caminho),
    };
   
   // navegando em seu conteúdo...
   for item in lista_no_dir {
      let item = item.unwrap();

      // possível link-símbolico.
      match item.path().as_path().read_link() {
          //se for link-simbólico, passar ele...
          Ok(sl) => {
            println!("é link-simbólico, burlando...{:#?}",sl); 
           continue;
          },
          Err(_) => (), //apenas ignorando...
      };

      // string do caminho.
      let pth_str = {
         item.path()
         .into_os_string()
         .into_string()
         .expect("falha ao obter caminho no formato de string!")
      };
      // nome do arquivo/diretório do caminho.
      let nome_pth = item.file_name().into_string().unwrap();

      // se for um diretório usar de recursividade.
      let espacamento = " ".repeat((*pfd) as usize);
      let pth = Path::new(pth_str.as_str());
      if pth.is_dir() {
          // molde de diretório(dois pontos).
          let mut str_aux = format!("{1}{2}{3}{4} {0}:\n",
                                    nome_pth,espacamento,
                                    GALHO_VH,GALHO_H,GALHO_H);
          // ajusta a string na tela.
          ajusta_string(&mut str_aux, true);
          esboco.push_str(str_aux.as_str());
          let novo_path = Path::new(pth_str.as_str());
          // cada chamada recursiva, aumenta a profundidade.
          (*pfd) += 3; 
          desenha_trilha(esboco, novo_path, pfd);
          // "volta" um diretório...
          (*pfd) -= 3; 
      }
      // se for apenas um arquivo, só registra.
      else {
         // molde diferente para arquivos:
         let mut straux = format!(
            "{1}{2}{3}{4} \"{0}\"\n",
            nome_pth, espacamento,
            GALHO_VH,GALHO_H,GALHO_H
         );
         // ajusta a string na tela.
         ajusta_string(&mut straux, false);     
         esboco.push_str(straux.as_str());
      }
   }
}

fn ajusta_string(s:&mut String, e_diretorio:bool) {
   let largura = {
      unsafe {
         if !JA_COMPUTADO {
            // Registra largura,então informa que já foi computado.
            if let Some((Largura(l), _)) = dimensao()
               { LARGURA = l as usize; }
            JA_COMPUTADO = true;
         } 
         LARGURA
      }
   };

   // Se comprimento da string exceder o da tela.
   if s.len() > largura {
      if e_diretorio 
         { s.replace_range((largura - 8)..,"(...):\n"); }
      else 
         { s.replace_range((largura - 2)..,"...\n");}
   }
}
   
/* escreva trilha, porém só registros diretórios...
 * arquivos não será ramificados na árvore. */
#[allow(dead_code)]
fn desenha_trilha_dirs(esboco:&mut String, caminho:&Path , pfd:&mut u8) {
   // navegando arquivos e diretórios.
   let lista_no_dir = match read_dir(caminho) {
       Ok(iterador) => iterador,
       Err(_) => panic!("diretório erro:\"{:#?}\"", caminho),
    };
   
   // navegando em seu conteúdo...
   for item in lista_no_dir {
      let item = item.unwrap();

      // possível link-símbolico.
      match item.path().as_path().read_link() {
         //se for link-simbólico, passar ele...
         Ok(sl) => {
         println!("é link-simbólico, burlando...{:#?}",sl); 
         continue;
         },
         Err(_) => () //apenas ignorando...
      };

      // string do caminho.
      let pth_str = {
         item
         .path()
         .into_os_string()
         .into_string()
         .expect("falha ao obter caminho no formato de string!")
      };
      // nome do diretório do caminho.
      let nome_pth = item.file_name().into_string().unwrap();

      // se for um diretório usar de recursividade.
      let espacamento = " ".repeat((*pfd) as usize);
      let pth = Path::new(pth_str.as_str());
      if pth.is_dir() {
          // molde de diretório(dois pontos).
          let mut str_aux = format!(
             "{1}{2}{3}{4} {0}:\n",
             nome_pth,espacamento,
             GALHO_VH,GALHO_H,GALHO_H
         );
         // ajusta a string na tela.
         ajusta_string(&mut str_aux, true);
         esboco.push_str(str_aux.as_str());
         let novo_path = Path::new(pth_str.as_str());
         (*pfd) += 3; // cada chamada recursiva, aumenta a profundidade.
         desenha_trilha_dirs(esboco, novo_path, pfd);
         (*pfd) -= 3; // "volta" um diretório...
      }
   }
}

/* acha o índice da array que representa 
 * abstratamente o galho das ramificações.
 * Caso não haja galho em tal array, o 'Option'
 * com o valor do índice será 'None'. */
fn acha_galho_dobrado(linha:&Vec<char>) -> Option<usize> {
   for indice in 0..linha.len() {
      if linha[indice] == GALHO_VH 
         { return Some(indice); }
   }
   /* nenhum galho a modificar a aparência, então
    * "nada" a retornar. */
   return None;
}

fn preenchendo_galhos(arvore:&mut Matriz) {
   // dimensão da matriz:
   let max_y = arvore.len();
   // variável mutável para posição móvel da última linha.
   let mut l1;
   
   for l in 0..(max_y-1) {
      // última linha da matriz:
      l1 = (max_y-1)-l;
      // coluna com galho de dobra:
      let c = match acha_galho_dobrado(&arvore[l1]) {
         Some(valor) => valor,
         None => { continue; }
      };

      /* Subindo e trocando espaços até encontrar outro galho de dobra.
       * Proposições:   */
      let mut p1 = arvore[l1-1][c] != GALHO_VH; 
      let mut p2 = arvore[l1-1][c].is_whitespace();
      let mut p3 = !(
         arvore[l1-1][c+1].is_ascii_alphanumeric()
         || arvore[l1-1][c+1] == '.' 
         || arvore[l1-1][c+1] == '_'
      );

      while p1 && p2 && p3 {
         // troca vácuo por galho vertical.
         arvore[l1-1][c] = GALHO_V;

         l1 -= 1; 
         // atualizando premissas
         p1 = arvore[l1-1][c] != GALHO_VH; 
         p2 = arvore[l1-1][c].is_whitespace();
         p3 = !(arvore[l1-1][c+1].is_ascii_alphanumeric()
               || arvore[l1-1][c+1] == '.');
      }
   }
}

fn troca_galhos_adequadamente(arvore:&mut Matriz) {
   // dimensão da matriz:
   let max_y = arvore.len();
   let max_x = arvore[0].len();
   
   for l in 0..max_y-1 {
      for c in 0..max_x-1 {
         /* se o galho for dobrado(formato de L) 
          * e, o caractére abaixo for não branco
          * ou alfanumérico, ou seja, não altera
          * os galhos dobrados de ponta. */
         let condicao = {
            arvore[l][c] == GALHO_VH &&
            !(arvore[l+1][c].is_whitespace() ||
            arvore[l+1][c].is_ascii_alphanumeric())
         };
         // seguida a condição... faz troca.
         if condicao 
            { arvore[l][c] = GALHO_VHV; }
      }
   }
}

fn preenche_primeira_coluna(arvore:&mut Matriz) {
   let inicio = 1;  
   let mut fim = 1;

   // achando última dobradiça.
   for l in inicio..arvore.len() {
      if arvore[l][0] == GALHO_VH {
         if l > fim 
            { fim = l; }
      }
   }

   // mudando... baeado no intervalo confiável filtrado.
   for l in inicio..fim {
      if arvore[l][0] == GALHO_VH {
         arvore[l][0] = GALHO_VHV;
      }
      else if arvore[l][0].is_whitespace() {
         arvore[l][0] = GALHO_V;
      }
   }
}

/** Retorna string representado tudo dentro de um dado diretório, 
 * ramificando-os em arquivos e sub-diretórios. */
pub fn arvore<P>(caminho:&P, mostra_arquivos:bool) -> String 
  where P: AsRef<Path> + ?Sized 
{
   // String para concatenar strings representado trilha.
   let mut trilha = String::new();
   // Raiz, de onde parte a trilhagem...
   let raiz = caminho.as_ref();
   // Obtendo o nome do diretório raíz.
   let raiz_nome = raiz.file_name().unwrap().to_str().unwrap();
   // Espaçar cada vez mais, em cada novo sub-diretório.
   let mut profundidade: usize = 0;

   // Anexa a raíz do diretório no começo da representação.
   trilha.push_str(raiz_nome);
   trilha.push_str(":\n");

   // Se estiver configurado para mostrar arquivos...
   /*
   if mostra_arquivos 
      { desenha_trilha(&mut trilha, raiz, &mut profundidade); }
   else 
      { desenha_trilha_dirs(&mut trilha, raiz, &mut profundidade); }
    *
    * */
   let _= desenha_trilha_personalizado
      (&mut trilha, raiz, &mut profundidade, None, None, mostra_arquivos);

   // Fazendo ajustes...
   let mut matriz_arv = matriciar_string(trilha.clone());
   preenchendo_galhos(&mut matriz_arv); 
   troca_galhos_adequadamente(&mut matriz_arv);
   preenche_primeira_coluna(&mut matriz_arv);

   /* Converte a matriz de caractéres de volta numa string, então retorna
    * o resultado. */
   matriz_para_string(&matriz_arv)
}

fn molda_caminho_de_acordo(esboco:&mut String, caminho: &Path, depth: usize) 
  -> Result<()>
{
   let tabulacao = " ".repeat(depth);
   let nome_do_caminho = caminho.file_name().unwrap().to_str().unwrap();
   let mut formatacao: String;

   if caminho.is_dir() {
      formatacao = format!(
          "{1}{2}{3}{4} {0}:\n",
         nome_do_caminho, tabulacao, GALHO_VH,GALHO_H,GALHO_H
      );
   } else {
      if caminho.is_symlink() {
         formatacao = format!(
            "{1}{2}{3}{4} {0}(->)\n",
            nome_do_caminho, tabulacao, GALHO_VH,GALHO_H,GALHO_H
         );
      } else {
         formatacao = format!(
            "{1}{2}{3}{4} \"{0}\"\n",
            nome_do_caminho, tabulacao, GALHO_VH,GALHO_H,GALHO_H
         );
      }
   }
   // ajusta a string na tela.
   ajusta_string(&mut formatacao, true);
   esboco.push_str(formatacao.as_str());
   Ok(())
}

type Exclusoes<'b> = Option<VecDeque<&'b str>>;
type MaxDepth = Option<usize>;

fn desenha_trilha_personalizado(esboco: &mut String, caminho:&Path, 
  profundidade:&mut usize, limite: MaxDepth, mut padrao: Exclusoes,
  mostra_arquivos: bool) 
  -> Result<()>
{
/* Escreva toda uma trilha, com ramificações de sub-diretórios e arquivos, 
 * dado uma raiz principal. Veja, ela não pecorre linque símbolicos. Esta
 * versão aqui pode-se impor um 'limite' de profundidade, assim como, 
 * que tipo de padrão ignorá, referente aos diretórios. Tais opções podem
 * ser ativadas ou não. */
   let mut entradas_do_diretorio = read_dir(caminho)?;

   if let Some(maximo) = limite { 
   // Se uma profundidade máxima for selecionada, para de ir até ela.
      if *profundidade / RECUO > maximo
         { return Ok(()); }
   }

   if let Some(ref mut fila) = padrao {
   /* Se algum dos subdiretórios, ou arquvios, tiver tal string compondo-a,
    * então o resto do processamento sobre tal será discartado imediatamente.
    */
      let mut total_de_exclusoes = fila.len();

      while total_de_exclusoes > 0 {
         let mut caminhos_partes = caminho.components();

         if let Some(pattern) = fila.pop_front() {
            while let Some(parte) = caminhos_partes.next() {
               if parte.as_os_str() == pattern 
                  { return Ok(()); }
            }
            fila.push_back(pattern);
         }
         total_de_exclusoes -= 1;
      }
   }

   // Encurtando para caber na tela; melhor a legibilidade do código.
   let visivel = mostra_arquivos;
   let depth = profundidade;

   while let Some(Ok(entry)) = entradas_do_diretorio.next() {
      let caminho = entry.path();

      if caminho.is_dir() && !caminho.is_symlink() 
      {
         let _= molda_caminho_de_acordo(esboco, &caminho, *depth);
          /* Cada chamada recursiva, aumenta a profundidade, quando termina
           * apenas recua de volta o espaçado. */
          (*depth) += RECUO; 
          let _= desenha_trilha_personalizado
           (esboco, &caminho, depth, limite, padrao.clone(), visivel);
          (*depth) -= RECUO; 

      } else {
      // se for apenas um arquivo, só registra.
         if mostra_arquivos
            { let _= molda_caminho_de_acordo(esboco, &caminho, *depth); }
      }
   }
   Ok(())
}

/** Mesmo capilaridade do que a árvore acima, porém, agora ela tem funções
 * extras, como: restrigir até determinada profundidade, e também exclui 
 * certos diretórios que forem listados. */
pub fn arvore_a<P>(caminho:&P, mostra_arquivos:bool, max: MaxDepth,
  exclusao: Exclusoes) -> String where P: AsRef<Path> + ?Sized 
{
   let raiz = caminho.as_ref();
   let mut profundidade: usize = 0;
   // String para concatenar strings representado trilha.
   let mut trilha = String::new();
   // Obtendo o nome do diretório raíz.
   let raiz_nome = raiz.file_name().unwrap().to_str().unwrap();

   // Anexa a raíz do diretório no começo da representação; seu cabeçalho.
   trilha.push_str(raiz_nome); trilha.push_str(":\n");

   // Se estiver configurado para mostrar arquivos...
   let _= desenha_trilha_personalizado
      (&mut trilha, raiz, &mut profundidade, max, exclusao, mostra_arquivos);

   // Remendando, renderizando, e fazendo últimos ajustes.
   let mut grade_de_desenho = matriciar_string(trilha);
   preenchendo_galhos(&mut grade_de_desenho); 
   troca_galhos_adequadamente(&mut grade_de_desenho);
   preenche_primeira_coluna(&mut grade_de_desenho);

   /* Converte a matriz de caractéres de volta numa string, então retorna
    * o resultado. */
   matriz_para_string(&grade_de_desenho)
}

#[cfg(test)]
#[cfg(target_os="linux")]
mod tests {
   // biblioteca padrão do Rust.
   use std::path::{Path};
   use std::env::{self, var};
   use super::*;

   // para testes.
   fn imprime(matriz: Matriz) {
       // pega a linha da matriz.
       for row in matriz {
           // coluna na linha.
           for cell in row { print!("{}", cell); }
           print!("\n");
       }
   }
   
   #[test]
   fn lista_subdirs_de_foto() {
      let caminho = concat!(env!("HOME"), "/Pictures");
      let pth = Path::new(caminho);
      let mut trilha = String::from("");
      let mut p = 0;
      desenha_trilha_dirs(&mut trilha,pth, &mut p);
      println!("caminho:\n{}",trilha);
      assert!(true);
   }

   #[test]
   fn ramifica_ambos_modos() {
      let pth = concat!(env!("HOME"), "/Pictures");
      let primeiro = arvore(pth,true);
      let segundo = arvore(pth, false);

      println!("resultado(1):\n{}\n\nresultado(2):\n{}",primeiro,segundo);
      assert!(true);
   }

   #[test]
   fn coloca_galhos() -> std::io::Result<()> {
      let caminho = env::current_dir()?;
      let arv = arvore(&caminho,false);
      let mut matriz_arv = matriciar_string(arv);
      preenchendo_galhos(&mut matriz_arv);
      imprime(matriz_arv);

      assert!(true);
      Ok(())
   }

   #[test]
   fn conserta_galhos_desajustados() {
      let caminho = var("HOME").unwrap() + "/Documents/códigos_rust";
      let arv = arvore(caminho.as_str(),true);
      let mut matriz_arv = matriciar_string(arv);
      preenchendo_galhos(&mut matriz_arv);
      troca_galhos_adequadamente(&mut matriz_arv);
      imprime(matriz_arv);
      assert!(true);
   }

   #[test]
   fn testando_arvore_implementacao_terminada() {
      let nucleo = match var("HOME") {
         Ok(s) => s,
         Err(_) => { panic!("não existe tal variável!"); },
      };
      let caminho = nucleo.clone() + "/Videos";
      let arv1 = arvore(caminho.as_str(), true);
      let caminho = nucleo.clone() + "/Documents/códigos";
      let arv2 = arvore(caminho.as_str(),true);

      println!("{}\n\n{}\n",arv1, arv2);

      assert!(true);
   }

   #[test]
   fn escupidor_de_trilha_personalizado() {
      const N: usize = 2_000;
      let mut e = String::with_capacity(N);
      let r = env::current_dir().unwrap();
      let mut p: usize = 0;
      let l = VecDeque::from(["deps", "implementors", "doc", "debug"]);

      println!("Limitação por profundidade:");
      desenha_trilha_personalizado
         (&mut e, &r, &mut p, Some(3), None, true).unwrap();
      println!("{}", e);

      println!("\nExclusão de subdiretórios:");
      e.clear();
      desenha_trilha_personalizado
         (&mut e, &r, &mut p, None, Some(l), true).unwrap();
      println!("{}", e);

      println!("\nNão mostra arquivos:");
      e.clear();
      desenha_trilha_personalizado
         (&mut e, &r, &mut p, None, None, false).unwrap();
      println!("{}", e);
   }

   #[test]
   fn funcao_de_arvore_personalizada() {
      let mut raiz = Path::new(env!("CCODES")).to_path_buf();
      raiz.push("utilitarios-em-c");

      println!("Impressão normal, que o velho método faz:");
      let tree_a = arvore_a(&raiz, true, None, None);
      println!("{tree_a:}");

      println!("\nComo limite de alcance:");
      let tree_b = arvore_a(&raiz, true, Some(1), None);
      println!("{tree_b:}");

      println!("\nComo excluindo alguns diretórios:");
      let lista = VecDeque::from(["static", "shared"]);
      let tree_c = arvore_a(&raiz, true, None, Some(lista));
      println!("{tree_c:}");
   }
}
