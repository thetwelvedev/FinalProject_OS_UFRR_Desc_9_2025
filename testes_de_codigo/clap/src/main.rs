use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    usuario: String,

    #[arg(short, long)]
    senha: String,
}

fn main() {
    let args = Args::parse();
    println!("Usuário: {}", args.usuario);
    println!("Senha: {}", args.senha);
}

