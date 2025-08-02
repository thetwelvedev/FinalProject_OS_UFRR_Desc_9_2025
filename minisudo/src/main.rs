extern crate bcrypt;
use bcrypt::{DEFAULT_COST, hash, verify};
use users::{get_current_uid, get_user_by_uid};

fn main() {
    
    //let password = rpassword::read_password().unwrap();//Lê a senha do usuário pelo terminal

    //Pega o id do usuário atual do sistema
    let uid = get_current_uid();
    
    //Aqui deve ocorrer a verificação do usuário para caso ele esteja cadastrado no minisudo
    if let Some(user) = get_user_by_uid(uid){
        
        //Definimos uma senha para o usuário, apenas teste.
        let password = rpassword::prompt_password(format!("[minisudo] Digite uma senha para {}: ", user.name().to_string_lossy())).unwrap();
        println!("Sua senha é: {}", password);
        
        //Senha passa pelo hash
        let hashed = hash(password, DEFAULT_COST).unwrap();//Hash criado referente a senha
        println!("Senha salva com sucesso!");

        //Teste de validação de senha do usuário
        let validation = rpassword::prompt_password(format!("[minisudo] senha para {}: ", user.name().to_string_lossy())).unwrap();
        let valid = verify(validation, &hashed).unwrap();//Verificação da senha do usuário
        
        if valid {
            println!("Sua senha foi validada com sucesso!");
        }else {
            println!("Erro ao tentar validar sua senha.",);
        } 

    }else{
        println!("Usuário não encontrado no sistema.");
    }
}