use crate::tela::Ponto;

// máximo de items que podem ser empilhados.
pub const CAPACIDADE: u8 = 20;

/* Registra uma "mudança" feita na tela,
 * tendo os últimos `Ponto`s que foram 
 * escrito nela. Então é isso que significa
 * uma mudança ocorrida, um aglomerado de 
 * pontos recentementes "printados" na tela.
 */
 pub struct Mudanca {
   /* pilha contendo pontos que foram, 
    * ultimamente, marcados. Também o símbolo
    * que estava alí.*/
   pub pontilhados_escritos: Vec<(Ponto, char)>,
}

// apelido para melhor legibilidade.
impl Mudanca {
   /* funciona junto com o método abaixo, que permite
    * dimensionar a lista de mudanças a desfazer/ou
    * refazer futuramente. */
   pub fn cria_vazio() -> Self 
      { Self { pontilhados_escritos: Vec::new() } }

   /* às vezes têm que adicionar algum que não 
    * foi inicialmente pensado. */
   pub fn incrementa(&mut self, pixel:(Ponto, char)) 
      { self.pontilhados_escritos.push(pixel); }
}
