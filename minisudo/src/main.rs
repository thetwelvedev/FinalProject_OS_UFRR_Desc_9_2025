extern crate bcrypt;
use bcrypt::{verify};
use users::{get_current_uid, get_user_by_uid};
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Parser)]
#[command(name = "minisudo")]
#[command(about = "Executa comandos com permissões controladas", long_about = None)]
struct Cli {
    /// Comando a ser executado
    comando: String,

    /// Argumentos do comando
    argumentos: Vec<String>,
}

fn main() {
    
    let args = Cli::parse();

    //Pega o id do usuário atual do sistema
    let uid = get_current_uid();
    
    //Verificação de usuário válido
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
    let Some(hash) = hash_encontrado else {
        eprintln!("Usuário '{}' não está autorizado no minisudoers.", username);
        std::process::exit(1);
    };

    let senha = rpassword::prompt_password(format!("[minisudo] senha para {}: ", username)).unwrap();
    if !verify(&senha, &hash).unwrap_or(false) {
        eprintln!("Senha incorreta.");
        std::process::exit(1);
    }

    //Abre o arquivo minisudoers por caminho temporário
    let file = File::open("./config/minisudoers").expect("Erro ao abrir minisudoers");
    let reader = BufReader::new(file);
    let mut permitido = false;

    //Realiza a leitura do arquivo minisudoers e verifica se o usuário tem permissão para executar o arquivo
    for linha in reader.lines() {
        if let Ok(line) = linha {
            let mut partes = line.trim().split_whitespace();
            if let Some(user_entry) = partes.next() {
                if user_entry == username {
                    let comandos: Vec<&str> = partes.collect();
                    if comandos.contains(&"ALL") || comandos.contains(&args.comando.as_str()) {
                        permitido = true;
                        break;
                    }
                }
            }
        }
    }

    if !permitido {
        eprintln!("Permissão negada: você não pode executar '{}'", args.comando);
        std::process::exit(1);
    }

    // Obter horário atual
    let agora = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Montar comando completo
    let comando_completo = format!("{} {}", args.comando, args.argumentos.join(" ")).trim().to_string();

    // Montar entrada de log
    let entrada_log = format!(
        "[{}] usuário: {}, comando: {}\n",
        agora, username, comando_completo
    );

    // Abrir (ou criar) arquivo de log em modo append
    let mut arquivo_log = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./logs/minisudo.log")
        .expect("Erro ao abrir ou criar o arquivo de log");

    // Escrever no log
    if let Err(e) = arquivo_log.write_all(entrada_log.as_bytes()) {
        eprintln!("Erro ao escrever no log: {}", e);
    }

    println!("Acesso concedido.");
    // Aqui você pode executar o comando desejado com privilégio
            
    // Simula a execução do comando passado pelo usuário
    println!("(simulado como root) Comando '{}' executado com sucesso!", args.comando);

}