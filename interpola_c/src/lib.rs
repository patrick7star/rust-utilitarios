/*!   Um bocado de funções que serão portados para serem usados em projetos 
 * em C e C++.
 *    Vamos começar tal usando os módulos que não são muitos entrelaçados com
 * a biblioteca, depois, começamos com este que são bem mais complicados.
 */

// Módulos importantes...
mod dir_and_files_trees;
mod written_numerals;

// Exportando apenas o necessário:
pub use written_numerals::{escrita_por_extenso};
pub use dir_and_files_trees::{gera_arvore, gera_arvore_config};
