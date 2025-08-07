use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;
use predicates::str::contains;


/// Função auxiliar para criar ambiente temporário com arquivos de config
fn criar_ambiente(username: &str, hash_senha: &str, comandos: &str) -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
    let dir = tempdir().unwrap();

    // minisudoers
    let minisudoers_path = dir.path().join("minisudoers");
    fs::write(&minisudoers_path, format!("{} {}\n", username, comandos)).unwrap();

    // minisudo_password
    let password_path = dir.path().join("minisudo_password");
    fs::write(&password_path, format!("{}:{}", username, hash_senha)).unwrap();

    (dir, minisudoers_path, password_path)
}

#[test]
fn usuario_nao_autorizado_falha() {
    // Criamos ambiente com um usuário fictício que não será o usuário real
    let (_dir, minisudoers_path, password_path) =
        criar_ambiente("usuario_ficticio", "$2b$12$abcdefghijklmnopqrstuv", "ls");

    let mut cmd = Command::cargo_bin("minisudo").unwrap();
    cmd.env("MINISUDO_TEST_USER", "teste_usuario")
        .env("MINISUDOERS_PATH", minisudoers_path)
        .env("MINISUDO_PASSWORD_PATH", password_path)
        .arg("ls");

    cmd.assert()
        .failure()
        .stderr(contains("não tem permissão"));
}

#[test]
fn erro_de_senha() {
    // Senha errada
    let hash_errada = bcrypt::hash("senha", bcrypt::DEFAULT_COST).unwrap();

    let (_dir, minisudoers_path, password_path) =
        criar_ambiente("teste_usuario_senha_errada", &hash_errada, "ls");

    let mut cmd = Command::cargo_bin("minisudo").unwrap();
    cmd.env("MINISUDO_TEST_USER", "teste_usuario_senha_errada")
        .env("MINISUDOERS_PATH", minisudoers_path)
        .env("MINISUDO_PASSWORD_PATH", password_path)
        .arg("ls")
        // Simulando que o usuário digitou senha errada
        .write_stdin("senha_errada\nsenha_errada\nsenha_errada\n");

    cmd.assert()
        .failure()
        .stderr(contains("3 tentativas de senha incorreta"));
}

#[test]
fn comando_negado() {
    // Usuário autenticado mas não tem permissão para "rm"
    // Hash gerado para a senha "1234"
    let hash_1234 = bcrypt::hash("1234", bcrypt::DEFAULT_COST).unwrap();

    let (_dir, minisudoers_path, password_path) =
        criar_ambiente("teste_usuario", &hash_1234, "ls");

    let mut cmd = Command::cargo_bin("minisudo").unwrap();
    cmd.env("MINISUDO_TEST_USER", "teste_usuario")
        .env("MINISUDOERS_PATH", minisudoers_path)
        .env("MINISUDO_PASSWORD_PATH", password_path)
        .arg("rm")
        .write_stdin("1234\n"); // senha correta

    cmd.assert()
        .failure()
        .stderr(contains("não tem permissão"));
}

#[test]
fn execucao_sucesso() {
    // Usuário com permissão para ALL
    let hash_1234 = bcrypt::hash("1234", bcrypt::DEFAULT_COST).unwrap();

    let (_dir, minisudoers_path, password_path) =
        criar_ambiente("teste_usuario", &hash_1234, "ALL");

    let mut cmd = Command::cargo_bin("minisudo").unwrap();
    cmd.env("MINISUDO_TEST_USER", "teste_usuario")
        .env("MINISUDOERS_PATH", minisudoers_path)
        .env("MINISUDO_PASSWORD_PATH", password_path)
        .arg("ls")
        .write_stdin("1234\n");

    cmd.assert()
        .success()
        .stdout(contains("Acesso concedido"))
        .stdout(contains("(simulado como root)"));
}