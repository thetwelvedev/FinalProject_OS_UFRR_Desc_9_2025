extern crate bcrypt;
use bcrypt::{DEFAULT_COST, hash, verify};

fn main() {
    
    //let password = rpassword::read_password().unwrap();//Lê a senha do usuário pelo terminal

    let password = rpassword::prompt_password("Digite sua senha: ").unwrap();
    println!("Sua senha é: {}", password);
    
    let hashed = hash(password, DEFAULT_COST).unwrap();//Hash criado referente a senha
    println!("Senha salva com sucesso!");

    
    let validation = rpassword::prompt_password("Informe sua senha para prosseguir: ").unwrap();
    
    let valid = verify(validation, &hashed).unwrap();//Verificação da senha do usuário

    if valid {
        println!("Sua senha foi validada com sucesso!");
    }else {
        println!("Erro ao tentar validar sua senha.",);
    }    
}