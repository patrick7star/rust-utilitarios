
/* Reescrita do código de tabelar
 * tais colunas, baseado no código
 * similar feito em Python. Se 
 * ficar mais simples que o atual,
 * este será substituído pelo novo.
 */

// biblioteca padrão do Rust:
use std::string::ToString;
use std::fmt::{
   Display,  Formatter,
   Result as Resultado
};
use std::marker::Copy;
use std::collections::VecDeque;
// extensão do módulo:
use super::{objeto::Coluna, StrExt, TRACO, LATERAL};
// do próprio caixote.
use crate::terminal_dimensao::{Largura, terminal_largura};

// componente que lacra a tabela.
const BARRA: char = '#';
const ESPACO: &'static str = ">";
const RECUO:usize  = 5;
type Fila = VecDeque<String>;

struct ColunaStr {
   forma_padrao: String,
   /* quantia padrão de linhas da Coluna,
    * ou uma nova preenchido com campos
    * em branco. */
   pub altura: Option<usize>,
   // células e cabeçalho mais espaços vázios.
   iterador: Vec<String>,
   // posição do atual item no iterador.
   posicao: usize,
   // largura da ColunaStr, que é a mesma
   // da Coluna passada na instância.
   pub largura: usize
}

// valor da diferença em absoluto de inteiros:
fn abs(x: usize, y:usize) -> usize {
   if x >= y { x - y }
   else { y - x }
}

impl ColunaStr {
   // método construtor:
   pub fn nova<X>(coluna: Coluna<X>, aumento: Option<usize>) 
   -> Self 
      where X: Display + Copy + Clone 
   {
      let mut iterador: Vec<String> = Vec::new();
      let forma_padrao = format!("{}", coluna);

      for linha in forma_padrao.lines() 
         { iterador.push(linha.to_string()); }

      match aumento {
         Some(valor) => {
            let lrg = coluna.largura();
            //let cv = &"-".repeat(lrg);
            let cv = campo_vago(lrg);
            let diferenca = abs(valor, coluna.linhas());

            for _ in 1..=diferenca 
               { iterador.push(cv.clone()); }
         } None => ()
      };

      Self {
         forma_padrao, altura: aumento, 
         iterador, posicao: 0, 
         largura: coluna.largura()
      }
   }
   // atualiza a quantia de campos vázios da ColunaStr.
   pub fn atualiza(&mut self, aumento: Option<usize>) {
      match aumento {
         Some(valor) => {
            let l = self.largura;
            let ql = match self.altura {
               Some(valor) => valor,
               None => 0
            };
            let cv = campo_vago(l);
            let diferenca = valor-ql;
            // adicionando a 'diferença' ...
            for _ in 1..=diferenca 
               { self.iterador.push(cv.clone()); }
            // resetando qualquer iteração.
            self.posicao = 0;
         } None => ()
      };
   }
}

// fazendo do objeto um iterador ...
impl Iterator for ColunaStr {
   type Item = String;

   // retorna a próxima célula string da Coluna.
   fn next(&mut self) -> Option<Self::Item> {
      if self.posicao <= self.iterador.len()-1 {
         self.posicao += 1;
         Some(self.iterador[self.posicao-1].clone())
      } else { self.posicao = 0; None }
   }
}

fn campo_vago(comprimento: usize) -> String {
   let mut vago = (&" ").repeat(comprimento);
   // computa o meio da string, recuado alguns passos.
   let meio = comprimento/2 + 1 - 3;
   vago.insert_str(meio, "---");
   // remove os extremos por causa dos "recuos".
   vago.pop().unwrap();
   vago.pop().unwrap();
   vago.pop().unwrap();
   return vago;
}

struct Tabela {
   // lista das strings já consertadas das Colunas.
   lista: Vec<ColunaStr>,
   /* diz se quer que use o máximo da 
    * largura do atual terminal preenchendo
    * com o restante da tabela, ao invés de
    * fazer um contínuo "pergaminho". */
   preenche_tela: bool,
   /* Salva construção inicialmente feita, pois
    * o processo de faze-lô é sempre custoso, 
    * então não dá para ficar reconstruindo 
    * em cada chamada.  */
   tabela_str: String,
   // maior quantia de linhas até agora da tabela.
   maior_ql: Option<usize>,
   // se é a primeira inserção.
   primeira: bool,
}

impl Tabela {
   // método construtor:
   pub fn nova(maximo_de_tela: bool) -> Self {
      Self { 
         lista: Vec::new(), 
         preenche_tela: maximo_de_tela,
         tabela_str: String::new(),
         maior_ql: None, 
         primeira: maximo_de_tela
      }
   }
   // adiciona nova Coluna passada.
   pub fn adiciona<Y>(&mut self, coluna: Coluna<Y>) 
      where Y: Display + Clone + Copy 
   {
      // atualizando primeiramente altura da tabela.
      match self.maior_ql {
         Some(h) => {
            if h < coluna.linhas()
               { self.maior_ql = Some(coluna.linhas()); }
         } None => 
               { self.maior_ql = Some(coluna.linhas()); }
      };
      // transforma a mesma numa versão textual.
      let aumento: Option<usize>;
      if self.primeira { aumento = None; }
      else { aumento = self.maior_ql; }
      let coluna_str = ColunaStr::nova(coluna, aumento);
      // insere na memória da tabela.
      self.lista.push(coluna_str);
      // refaz desenho da tabela.
      self.desenha_tabela();
   }
   // método auxiliar para construção da tabela como string.
   fn desenha_tabela(&mut self) {
      // limpa string antiga.
      self.tabela_str.clear();
      /* atualizando altura das devidas
       * colunas para que estejam todas
       * numa "altura" comum. */
      for coluna_str in self.lista.iter_mut() 
         { coluna_str.atualiza(self.maior_ql); }

      let mut mql:usize = self.maior_ql.unwrap();
      /* concatenando cada célula de cada 
       * 'coluna textual', capturando elas 
       * dos estágios em suas devidas estágio
       * da iteração. */
      while mql > 0 {
         for coluna_str in self.lista.iter_mut() {
            let ts = coluna_str.next().unwrap();
            self.tabela_str.push(BARRA);
            self.tabela_str.push_str(ts.as_str());
         }
         // quebra de linha.
         if !self.primeira
            { self.tabela_str.push(BARRA); }
         // fecha tabela lateralmente e adiciona
         // quebra-de-linha.
         self.tabela_str.push(BARRA);
         self.tabela_str.push('\n');
         mql -= 1;
      }
      
      // dive a tabela em frações, talvez,... iguais!
      if self.preenche_tela { 
         let ql = self.ql_otimizada();
         self.fraciona(dbg!(ql)); 
      }
      // fecha a tabela completamente.
      self.tampa();
      // aplica revestimento.
      self.revestimento();
   }
   // fraciona tabela em 'n' partes.
   fn fraciona(&mut self, mut qtd: usize) { 
      let mut linhas = self.tabela_str.lines();
      /* não faz nada se a quantia for igual
       * ao 'total atual' de linhas. */
      if linhas.clone().count() == qtd 
         { return (); }
      /* números inssuficiente de linhas
       * também não é aceitável. */
      if qtd < 3 { return  (); }

      type Fatias = VecDeque<String>;
      let mut fila: Fatias = Fatias::new();
      let mut contador = 0;

      for _ in 1..=qtd { 
         let linha = linhas.next().unwrap();
         fila.push_back(linha.to_string()); 
      }

      while let Some(linha) = linhas.next() {
         // tira o primeiro elemento.
         let mut primeira = fila.pop_front().unwrap();
         // concatena com o que veio.
         primeira += &ESPACO.repeat(RECUO);
         primeira.push_str(linha);
         // põe novamente no fim da fila.
         fila.push_back(primeira);
         /* contando a quantia de vezes que isso foi
          * realizado, para uma sicronização. */
         contador += 1;
      }
      /* Em caso de uma divisão não completa,
       * o cabeçalho e algums linhas terminal
       * no final, ao invés do começo. Para 
       * desfazer tal é preciso faz operação
       * inversa na "deck". Quantas vezes? Bem
       * a quantia restante para que o 'contador'
       * fique divisível pela quantia de linhas
       * demandas. */
       while contador % qtd != 0 {
         // remove do fim ...
         let x = fila.pop_back().unwrap();
         // coloca na frente.
         fila.push_front(x);
         contador -= 1;
       }
       // concatenando tudo no fim.
       // limpa, primeiramente, a anterior.
       self.tabela_str.clear();
       while let Some(linha) = fila.pop_front() { 
         self.tabela_str += linha.as_str();  
         self.tabela_str.push('\n')
      }
   }
   /* tampa a tabela, tanto às partes superiores
    * e inferiores, quanto entre as linhas. */
   fn tampa(&mut self) {
      let mut linhas = self.tabela_str.lines();
      let mut fila: Fila = Fila::new();

      // tampas superiores e inferiores especiais:
      let mut superior: Option<String> = None;
      let mut inferior: Option<String> = None;

      while let Some(linha) = linhas.next() {
         let comprimento = StrExt::len(linha);
         //let barra = &"#".repeat(comprimento);
         let barra = cria_barra(linha);

         /* se for o primeira caso, então
          * pega está barra, e sempre a 
          * retorna. */
         superior = match superior {
            Some(bar) => Some(bar),
            None => Some(barra.to_string())
         };

         fila.push_back(linha.to_string());
         fila.push_back(barra.to_string());
      }
      /* trocando visualização por 
       * ação concreta: 
      // concatenação é só esvaziar fila.
      println!("{}", superior.unwrap());
      while fila.len() > 0 {
         println!("{}", fila.pop_front().unwrap());
      } */
      self.tabela_str.clear();
      self.tabela_str = superior.unwrap();
      self.tabela_str.push('\n');
      while fila.len() > 0 { 
         let remocao = fila.pop_front().unwrap();
         self.tabela_str.push_str(remocao.as_str());
         self.tabela_str.push('\n');
      }
   }
   /* computa a quantia de linhas que reparti
    * a tabela, para que se caiba na tela
    * do terminal. */
   fn ql_otimizada(&self) -> usize {
      let ql = self.maior_ql.unwrap();
      // largura do terminal.
      let lt = {
         self.lista.iter()
         .map(|item| item.largura)
         .max().unwrap()
      };
      let a: usize;
      match terminal_largura() {
         Ok(Largura(l)) => { a = l as usize; }
         Err(_) => { a = 0; }
      };
      let n = a / (lt + RECUO + 1);
      return ql / n;
   }
   /* tarefa de revestimento. */
   fn revestimento(&self) {
      let mut alternador: bool = true;
      let mut nova_linha: String = String::new();

      for linha in self.tabela_str.lines() {
         if !alternador {
            let margem = &ESPACO.repeat(RECUO);
            let espaco = &" ".repeat(RECUO);
            nova_linha = linha.replace(margem, espaco);
            nova_linha = nova_linha.replace(
               BARRA.to_string().as_str(),
               LATERAL.to_string().as_str()
            );
         } else {
            let comprimento = linha.len();
            nova_linha = TRACO.repeat(comprimento-2);
            nova_linha.insert(0, LATERAL.chars().next().unwrap());
            nova_linha.push(LATERAL.chars().next().unwrap());
         }
         println!("{}", nova_linha);
         // entre linhas.
         alternador = !alternador;
      }
   }
}

impl Display for Tabela {
   fn fmt(&self, molde:&mut Formatter<'_>) -> Resultado 
      { write!(molde, "{}", self.tabela_str) }
}
      
fn cria_barra(string:&str) -> String {
   let comprimento = StrExt::len(string);
   return BARRA.to_string().repeat(comprimento).to_string();
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use crate::aleatorio::sortear;
   use super::*;

   #[test]
   fn testa_struct_ColunaStr() {
      let moedas_magicas = Coluna::nova(
         "moedas mágicas(qtd.)",
         vec![198, 1923, 038, 932, 38_839,
            3, 12, 538, 752, 65, 129]
      );

      let salarios = Coluna::nova(
         "salário(R$)",
          vec![5288.32, 1_000.10, 893.25, 1_839.00]
      );

      let generos = Coluna::nova(
         "gênero(macho/fêmea)", 
          vec!['F','F','F','F','M','M',
             'F','M','M','F','F','F','M']
      );

      let altura = Some(generos.linhas());

      let mut strI = ColunaStr::nova(moedas_magicas, altura);
      let mut strII = ColunaStr::nova(salarios, altura);
      let mut strIII = ColunaStr::nova(generos, altura);


      for _ in 1..=altura.unwrap() {
         let c1 = strI.next().unwrap();
         let c2 = strII.next().unwrap();
         let c3 = strIII.next().unwrap();
         println!("{}#{}#{}#", c1, c2, c3);
      }

      // avaliação manual.
      assert!(true);
   }

   #[test]
   fn testa_struct_Tabela() {
      let moedas_magicas = Coluna::nova(
         "moedas mágicas(qtd.)",
         vec![198, 1923, 038, 932, 38_839,
            3, 12, 538, 752, 657]
      );

      let salarios = Coluna::nova(
         "salário(R$)",
          vec![5288.32, 1_000.10, 893.25, 1_839.00]
      );

      let generos = Coluna::nova(
         "gênero(macho/fêmea)", 
          vec!['F','F','F','F','M','M',
             'F','M','M','F','F','F','M']
      );

      let nomes = Coluna::nova(
         "candidatos", vec!["Claúdio", "Ana Joana",
         "Marcelo", "Flávia", "Henrique Barcelos"]
      );

      let mut t = Tabela::nova(false);
      t.adiciona(moedas_magicas);
      println!("primeiro resultado:\n{}", t);
      t.adiciona(generos);
      println!("\tsegundo resultado:\n{}", t);
      t.adiciona(salarios);
      println!("\t\tterceiro resultado:\n{}", t);
      t.adiciona(nomes);
      println!("\n{}", t);
   }

   #[test]
   fn testa_struct_Tabela_parteI() {
      let quantias = |n| { 
         let mut array: Vec<u16> = Vec::new();
         for _ in 1..=n
            { array.push(sortear::u16(0..=35_000)); }
         array
      };
      let moedas_magicas = Coluna::nova(
         "moedas mágicas(qtd.)",
         quantias(128)
      );

      let salarios = Coluna::nova(
         "salário(R$)",
         quantias(151)
      );

      let quantias = |n, opI, opII| {
         let mut array: Vec<char> = Vec::new();
         for _ in 1..=n { 
            if sortear::bool() 
               { array.push(opI); }
            else
               { array.push(opII); }
         }
         array
      };
         
      let generos = Coluna::nova(
         "gênero(macho/fêmea)", 
         quantias(134, 'F', 'M')
      );

      let mut t = Tabela::nova(true);
      t.adiciona(moedas_magicas);
      t.adiciona(generos);
      t.adiciona(salarios);
      println!("\n\t\tverificando dobradura ...\n{}", t);
   }
}
