
/* Reescrita do código de tabelar
 * tais colunas, baseado no código
 * similar feito em Python. Se 
 * ficar mais simples que o atual,
 * este será substituído pelo novo.
 */


pub struct Tabela<T> {
   lista: Vec<Coluna<T>>,
   // forma a tabela pensando em usar 
   // o máximo da tela do terminal.
   preenche_tela: bool
}

impl<U:Display + Copy> Display for Tabela<U> {
   fn fmt(&self, molde:&mut Formatter<'_>) -> Resultado {
      // string de concatenação.
      let mut s = String::from("");
      // maior comprimento de caractéres:
      let mut c_max:u8 = self.rotulo.len() as u8;
      
      // busca maior comprimento...
      for x in self.rol.clone() {
         let c = x.to_string().len();
         if c as u8 > c_max {
            c_max = c as u8;
         }
      }

      s.push_str(calibra_str(self.rotulo, c_max).as_str());
      s.push('\n');
      for v in self.rol.clone() {
         let  ss = calibra_str(v.to_string().as_str(), c_max);
         s.push_str(ss.as_str());
         s.push('\n');
      }
      // escrevendo no formatdor...
      write!(molde, "{}", s)
   }
}
