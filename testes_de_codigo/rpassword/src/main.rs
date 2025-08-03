use rpassword::read_password;

fn main() {
    println!("Digite sua senha:");
    let senha = read_password().expect("Erro ao ler senha");
    println!("Senha digitada (oculta): {}", senha);
}

