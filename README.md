# Minisudo

## Projeto Final de Sistemas Operacionais

---

**Integrantes:** [Leonardo Castro](https://github.com/thetwelvedev) e [Álefe Alves](https://github.com/AlefeAlvesC)

**Descrição:**  
O objetivo deste projeto é implementar uma versão simplificada do utilitário `sudo` do Linux, denominado **minisudo**, utilizando a linguagem **Rust**. O software será testado em um ambiente virtualizado com **QEMU** e uma distribuição Linux leve. A atividade visa integrar conhecimentos de segurança, controle de permissões e programação segura.

---

## Índice
- [Minisudo](#minisudo)
  - [Projeto Final de Sistemas Operacionais](#projeto-final-de-sistemas-operacionais)
  - [Índice](#índice)
  - [Descrição do Projeto](#descrição-do-projeto)
  - [Funcionalidades](#funcionalidades)
  - [Estrutura do Minisudo](#estrutura-do-minisudo)
  - [Crates Utilizados](#crates-utilizados)
    - [bcrypt](#bcrypt)
    - [rpassword](#rpassword)
    - [clap](#clap)
    - [users](#users)
    - [chrono](#chrono)
  - [Artigo](#artigo)
  - [Referências](#referências)
---

## Descrição do Projeto

O `minisudo` é uma aplicação de terminal que simula o comportamento do `sudo` tradicional:

- Pede autenticação por senha.
- Permite execução de comandos com privilégios administrativos simulados.
- Registra logs de execução.
- Implementa medidas básicas de segurança.

---

## Funcionalidades

- Autenticação simples.
- Execução de comandos simulados.
- Registro de uso.
- Testes automatizados no ambiente QEMU.

---

## Estrutura do Minisudo
```bash
minisudo/
├── Cargo.toml
├── config/
│   ├── minisudo_password
│   └── minisudoers
├── logs/
│   └── minisudo.log (gerado em runtime)
└── src/
    └── main.rs
```

## Crates Utilizados

### bcrypt
> Usado para:
* Verificar se a senha digitada corresponde ao hash armazenado.
* Garantir autenticação segura utilizando o algoritmo bcrypt.

### rpassword
> Usado para:
* Solicitar a senha do usuário no terminal de forma segura.
* Impedir que os caracteres da senha apareçam enquanto o usuário digita 

### clap
> Usado para:
* Tratar argumentos da linha de comando.
* Definir o nome do comando (comando) e os seus argumentos (argumentos) de forma estruturada.

### users
> Usado para:
* Obter o UID (identificador do usuário atual)
* Obter o nome de login do usuário com base no UID.
* Identificar qual usuário está tentando executar o comando.

### chrono
> Usado para:
* Obter a data e hora atual do sistema.
* Formatar a data/hora em uma string legível.
* Registrar a data e hora de execução dos comandos no arquivo de log.

## Artigo

<div style="font-size: 20px; font-weight: bold; color: black;">Para uma vizualização mais detalhada do relatório:</div> 

* [Acesse o Artigo]()

---

## Referências

- TANENBAUM, Andrew S.; WOODHULL, Albert S. Sistemas operacionais: projeto e implementação. Tradução de João Tortello. 3. ed. Porto Alegre: Bookman, 2008. Recurso eletrônico.
