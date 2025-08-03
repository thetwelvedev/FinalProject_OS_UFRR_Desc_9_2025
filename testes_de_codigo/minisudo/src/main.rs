use bcrypt::{hash, verify, DEFAULT_COST};
use clap::{Parser, Subcommand};
use rpassword::read_password;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

const DB_PATH: &str = "usuarios.json";

#[derive(Serialize, Deserialize)]
struct Usuarios {
    dados: HashMap<String, String>, // nome_usuario -> hash_senha
}

impl Usuarios {
    fn carregar() -> Self {
        if Path::new(DB_PATH).exists() {
            let conteudo = fs::read_to_string(DB_PATH).expect("Erro ao ler o arquivo de usuários");
            serde_json::from_str(&conteudo).unwrap_or_else(|_| Usuarios {
                dados: HashMap::new(),
            })
        } else {
            Usuarios {
                dados: HashMap::new(),
            }
        }
    }

    fn salvar(&self) {
        let json = serde_json::to_string_pretty(&self).expect("Erro ao serializar JSON");
        fs::write(DB_PATH, json).expect("Erro ao salvar arquivo de usuários");
    }
}

#[derive(Parser)]
#[command(name = "MiniSudo", version, about = "Cadastro e login com bcrypt")]
struct Cli {
    #[command(subcommand)]
    comando: Comando,
}

#[derive(Subcommand)]
enum Comando {
    /// Cadastrar novo usuário
    Cadastrar {
        #[arg(short, long)]
        usuario: String,
    },
    /// Fazer login
    Login {
        #[arg(short, long)]
        usuario: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut usuarios = Usuarios::carregar();

    match cli.comando {
        Comando::Cadastrar { usuario } => {
            if usuarios.dados.contains_key(&usuario) {
                println!("⚠️ Usuário '{}' já existe.", usuario);
                return;
            }

            println!("Digite a nova senha para '{}':", usuario);
            let senha = read_password().expect("Erro ao ler senha");

            let hash_senha = hash(&senha, DEFAULT_COST).expect("Erro ao gerar hash");

            usuarios.dados.insert(usuario.clone(), hash_senha);
            usuarios.salvar();
            println!("✅ Usuário '{}' cadastrado com sucesso!", usuario);
        }

        Comando::Login { usuario } => {
            if let Some(hash_salvo) = usuarios.dados.get(&usuario) {
                println!("Digite a senha de '{}':", usuario);
                let senha = read_password().expect("Erro ao ler senha");

                match verify(&senha, hash_salvo) {
                    Ok(true) => println!("✅ Login bem-sucedido!"),
                    Ok(false) => println!("❌ Senha incorreta."),
                    Err(e) => println!("Erro ao verificar: {}", e),
                }
            } else {
                println!("❌ Usuário '{}' não encontrado.", usuario);
            }
        }
    }
}

