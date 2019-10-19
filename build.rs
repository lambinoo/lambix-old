extern crate sha1;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use sha1::Sha1;


fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("can't find out directory"));
    let mut src_dir = get_project_dir();
    src_dir.push("src");

    let compiler = Compiler::new(&src_dir);
    compiler.compile(&out_dir, "liblambix.a").expect("error when compiling assembly files");

    println!("cargo:rustc-link-search=native={}", out_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=lambix");
}


#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    CompilerError(String)
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}

struct Compiler<'a> {
    src_dir: &'a Path
}

impl<'c> Compiler<'c> {
    pub fn new<'a>(src_dir: &'a Path) -> Compiler<'a> {
        Compiler { src_dir }
    }

    pub fn compile(&self, out_dir: &Path, libname: &str) -> Result<(), Error> {
        let mut out_files = Vec::new();
        Compiler::compile_files_in_directory(self.src_dir, out_dir, &mut out_files)?;
        Compiler::create_archive(libname, out_dir, &out_files)?;
        Ok(())
    }

    pub fn create_archive(libname: &str, out_dir: &Path, out_files: &Vec<PathBuf>) -> Result<(), Error> {
        let mut output = out_dir.to_path_buf();
        output.push(libname);

        let mut archive_process = Command::new("llvm-ar");
        archive_process
            .arg("crus")
            .arg(output.to_str().unwrap());

        for path in out_files.iter() {
            archive_process.arg(path.to_str().unwrap());
        }

        println!("{:?}", archive_process);

        let status = archive_process.status().expect("can't start archive process");
        if status.success() {
            Ok(())
        } else {
            Err(Error::CompilerError(
                String::from_utf8_lossy(&archive_process.output().unwrap().stderr).to_string()
            ))
        }
    }

    fn compile_file(src_file: &Path, out_dir: &Path, out_files: &mut Vec<PathBuf>) -> Result<(), Error> {
        let mut out_file = out_dir.to_owned();
        out_file.push(format!(
            "{}-{}.o",
            Sha1::from(src_file.to_str().unwrap()).digest().to_string(),
            src_file.file_name().unwrap().to_str().unwrap()
        ));

        let mut nasm_compiler = Command::new("nasm");
        nasm_compiler
            .arg(src_file.to_str().expect("invalid source file path"))
            .arg(&format!("-o {}", out_file.to_str().unwrap()))
            .arg("-f elf64");
        
        if env::var("PROFILE").expect("can't find current profile") == "debug" {
            nasm_compiler.arg("-g");
        }

        println!("{:?}", nasm_compiler);

        let output = nasm_compiler.output().unwrap();
        if nasm_compiler.status().expect("can't start nasm command").success() {
            out_files.push(out_file); 
            let warnings = String::from_utf8_lossy(&output.stderr).to_string();
            for line in warnings.lines() {
                println!("cargo:warning={}", line);
            }
            Ok(())
        } else {
            Err(Error::CompilerError(
                String::from_utf8_lossy(&output.stderr).to_string()
            ))
        }
    }

    fn compile_files_in_directory<'a, 'b>(dir: &Path, out_dir: &Path, out_files: &mut Vec<PathBuf>) -> Result<(), Error> {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                Compiler::compile_files_in_directory(&path, out_dir, out_files)?;
            } else if path.is_file() && path.extension().unwrap_or_default() == "S" {
                println!("rerun-if-changed={}", path.to_string_lossy());
                Compiler::compile_file(&path, out_dir, out_files)?;
            }
        }

        Ok(())
    }
}

fn get_project_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("can't find project root directory"))
}

