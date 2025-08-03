extern crate bcrypt;
use bcrypt::{verify};
use users::{get_current_uid, get_user_by_uid};
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Comando a ser executado com permissão
    #[arg(required = true)]
    comando: Vec<String>,
}


fn main() {
    
    let args = Args::parse();

    //Pega o id do usuário atual do sistema
    let uid = get_current_uid();
    
    //Aqui deve ocorrer a verificação do usuário para caso ele esteja cadastrado no minisudo
    let Some(user) = get_user_by_uid(uid) else{
        eprintln!("Erro: Usuário não encontrado!");
        std::process::exit(1);
    };
    let username = user.name().to_string_lossy().into_owned();
     
    //let file = match File::open("/etc/minisudo_password") {
    //Abre o arquivo minisudo_password por caminho temporário
    let file = match File::open("./config/minisudo_password") {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Erro: Não foi possível abrir o arquivo minisudo_password.");
            std::process::exit(1);
        }
    };
    let reader = BufReader::new(file);

    let mut hash_encontrado: Option<String> = None;

    //Leitura do arquivo  
    for linha in reader.lines() {
        if let Ok(line) = linha {
            let partes: Vec<&str> = line.trim().split(':').collect();
            if partes.len() != 2 {
                continue;
            }

            if partes[0] == username {
                hash_encontrado = Some(partes[1].to_string());
                break;
            }
        }
    }

    //Validação da senha
    if let Some(hash) = hash_encontrado {
        let senha = rpassword::prompt_password(format!("[minisudo] senha para {}: ", username)).unwrap();
        if verify(&senha, &hash).unwrap_or(false) {
            println!("Acesso concedido.");
            // Aqui você pode executar o comando desejado com privilégio
            
            // Executa o comando passado pelo usuário
            let status = Command::new(&args.comando[0])
                .args(&args.comando[1..])
                .status();
        
                match status {
                Ok(s) => std::process::exit(s.code().unwrap_or(0)),
                Err(e) => {
                    eprintln!("Erro ao executar o comando: {}", e);
                    std::process::exit(1);
                }
            }
            
        } else {
            eprintln!("Senha incorreta.");
            std::process::exit(1);
        }
    } else {
        eprintln!("Usuário '{}' não está autorizado no minisudoers.", username);
        std::process::exit(1);
    }
}