## `chip8-rs` - um simples interpretador de CHIP-8 escrito em Rust

<p align="center"><target="_blank" rel="noopener noreferrer"><img src="assets/space_invaders_demo.jpeg?raw=true" alt="space invaders"></a></p>

Caso queira executar o emulador em sua distribuição GNU/Linux diretamente, vocẽ pode baixar um binário pré-compilado com os comandos abaixo 

1. Baixe o binário

```bash
wget https://github.com/mickael-lima/chip8-rs/releases/download/release/chip8-rs
```
2. Dê permissão de execução 
```bash
chmod +x chip8-rs
```
3. Execute-o com `./chip8-rs` seguindo [as instruções de como usá-lo](#Instruções de Uso)

### Instruções de Compilação

Para compilar esse interpretador localmente, é necessário ter os pacotes `rust` e `cargo` instalados e operacionais em sua distribuição GNU/Linux. Atendido essas exigências:

1. Clone o repositório utilizando `git`
> git clone https://github.com/mickael-lima/chip8-rs

2. Dentro do diretório `chip8-rs`, use o `cargo` para gerar um binário. 

```bash
cargo build --release
```

3. O binário será criado no diretório `target/release/` com o nome `chip8-rs`

> [!TIP]
> Você pode compilar e rodar o binário diretamente com `cargo run` sem problemas.

### Instruções de Uso

Para carregar uma ROM no interpretador, é necessário apenas apontar a localização dela como argumento na linha de comando, isso é: `./chip8-rs /caminho/para/rom`. Observe que a ROM não deverá exceder o tamanho de `3584 kb` (apesar de que dificilmente existirá uma ROM de CHIP-8 que chegue nesse valor pelo sistema ter apenas `4096 kb` de RAM). O interpretador não possui outros argumentos além do próprio caminho da ROM.

#### ROMs 

Você pode achar ROMs de CHIP-8 (majoritariamente de jogos e demos) [nesse repositório](https://github.com/kripod/chip8-roms). Caso queira modificar a implementação de algum op-code, recomendo utilizar [essas ROMs de teste][https://github.com/corax89/chip8-test-rom] para garantir que tudo está ocorrendo bem.

#### Controle

Os controles do interpretador foram implementados seguindo a conveção mais usada para o mapa de teclas. Infelizmente as ROMs não são diretas em relação a qual tecla usar para controlar o jogo, então é necessário chutar para descobrir.

| 1 	| 2 	| 3 	| 4 	|
|---	|---	|---	|---	|
| Q 	| W 	| E 	| R 	|
| A 	| S 	| D 	| F 	|
| Z 	| X 	| C 	| V 	|


### TO-DOs

- Adicionar áudio
