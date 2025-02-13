/*!   Um bocado de funções que serão portados para serem usados em projetos 
 * em C e C++.
 *    Vamos começar tal usando os módulos que não são muitos entrelaçados com
 * a biblioteca, depois, começamos com este que são bem mais complicados.
 */

// Módulos importantes...
mod tree;
mod written_numerals;

// Exportando apenas o necessário:
pub use written_numerals::{escrita_por_extenso};
pub use tree::{gera_arvore, gera_arvore_config};
