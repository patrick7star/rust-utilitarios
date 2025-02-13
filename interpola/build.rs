use std::process::{Command};
use std::path::{PathBuf};
use std::env::{current_exe};
use std::ffi::{OsStr};

const NOME_PROJETO: &'static str = "rust-utilitarios";


fn diretorio_base_do_projeto() -> PathBuf {
   let mut executavel = current_exe().unwrap();
   let nome = OsStr::new(NOME_PROJETO);

   while executavel.file_name().unwrap() != nome
      { executavel.pop(); }

   executavel
}

fn compilacao_do_arquivo_de_c_amostras() {
   let mut base = diretorio_base_do_projeto();
   base.push("interpola");

   let lib_exe = format!("{}/lib/libsample.a", base.display());
   let src = format!("{}/tests/amostras.c", base.display());
   let obj_build = format!("{}/lib/amostras.obj", base.display());

   Command::new("clang")
      .args(["-c", "-o", &obj_build, &src])
      .spawn().unwrap().wait().unwrap();
   Command::new("ar")
      .args(["crs", &lib_exe, &obj_build])
      .spawn().unwrap().wait().unwrap();
}

fn main() {
   compilacao_do_arquivo_de_c_amostras();

   let base = diretorio_base_do_projeto();
   let base_str = base.to_str().unwrap();
   let pth = format!("{base_str:}/interpola/lib");

   println!("cargo:rustc-link-search=native={}", pth);
   println!("cargo:rustc-link-lib=sample");
   println!("cargo:rerun-if-changed={base_str:}/tests/amostras.c");
}
