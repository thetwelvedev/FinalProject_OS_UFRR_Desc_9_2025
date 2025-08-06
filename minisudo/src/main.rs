extern crate bcrypt;
use bcrypt::verify;
use users::{get_current_uid, get_user_by_uid};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use clap::Parser;
use chrono::Local;

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

    // Pega o ID do usuário atual do sistema
    let uid = get_current_uid();

    // Verifica se o usuário é válido
    let Some(user) = get_user_by_uid(uid) else {
        eprintln!("Erro: Usuário não encontrado!");
        std::process::exit(1);
    };
    let username = user.name().to_string_lossy().into_owned();

    // Abre o arquivo minisudo_password no diretório temporário ./config
    let file = match File::open("./config/minisudo_password") {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Erro: Não foi possível abrir o arquivo minisudo_password.");
            std::process::exit(1);
        }
    };
    let reader = BufReader::new(file);

    let mut hash_encontrado: Option<String> = None;

    // Lê o arquivo linha por linha para encontrar o hash do usuário
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

    // Verifica se o hash foi encontrado
    let Some(hash) = hash_encontrado else {
        eprintln!("Usuário '{}' não está autorizado no minisudoers.", username);
        std::process::exit(1);
    };

    let max_tentativas = 3;
    let mut autenticado = false;

    // Solicita senha até 3 tentativas
    for _ in 1..=max_tentativas {
        let senha = rpassword::prompt_password(
            format!("[minisudo] senha para {}: ", username)
        ).unwrap();
        if verify(&senha, &hash).unwrap_or(false) {
            autenticado = true;
            break;
        } else {
            eprintln!("Sinto muito, tente novamente.");
        }
    }

    if !autenticado {
        eprintln!("minisudo: 3 tentativas de senha incorreta.");
        std::process::exit(1);
    }

    // Abre o arquivo minisudoers no diretório ./config
    let file = File::open("./config/minisudoers").expect("Erro ao abrir minisudoers");
    let reader = BufReader::new(file);
    let mut permitido = false;

    // Verifica se o usuário tem permissão para o comando solicitado
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

    // Obtem horário atual formatado
    let agora = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Monta o comando completo com argumentos
    let comando_completo = format!("{} {}", args.comando, args.argumentos.join(" ")).trim().to_string();

    // Monta a entrada de log
    let entrada_log = format!(
        "[{}] usuário: {}, comando: {}\n",
        agora, username, comando_completo
    );

    // Garante que o diretório ./logs existe antes de gravar o log
    // Isso evita erro se a pasta não existir
    fs::create_dir_all("./logs").expect("Erro ao criar diretório de logs");

    // Abre (ou cria) o arquivo de log em modo de adição (append)
    let mut arquivo_log = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./logs/minisudo.log")
        .expect("Erro ao abrir ou criar o arquivo de log");

    // Escreve a entrada no log
    if let Err(e) = arquivo_log.write_all(entrada_log.as_bytes()) {
        eprintln!("Erro ao escrever no log: {}", e);
    }

    println!("Acesso concedido.");
    println!("(simulado como root) Comando '{}' executado com sucesso!", args.comando);
}
