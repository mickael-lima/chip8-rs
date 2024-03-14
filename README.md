## `chip8-rs` - um simples interpretador de CHIP-8 escrito em Rust

<p align="center"><target="_blank" rel="noopener noreferrer"><img src="assets/space_invaders_demo.jpeg?raw=true" alt="space invaders"></a></p>

### Instruções de Instalação

Para rodar esse interpretador localmente, é necessário ter os pacotes `rust` e `cargo` instalados e operacionais em sua distribuição GNU/Linux. Atendido essas exigências:

1. Clone o repositório utilizando `git`
> git clone https://github.com/mickael-lima/chip8-rs

2. Dentro do diretório `chip8-rs`, use o `cargo` para gerar um binário. 
> cargo build --release

3. O binário será criado no diretório `target/release/` com o nome `chip8-rs`

### Instruções de Uso

Para carregar uma ROM no interpretador, é necessário apenas apontar a localização dela como argumento na linha de comando, isso é: `./chip8-rs /caminho/para/rom`. Observe que a ROM não deverá exceder o tamanho de `3584 kb` (apesar de que dificilmente existirá uma ROM de CHIP-8 que chegue nesse valor pelo sistema ter apenas `4096 kb`). O interpretador não possui outros argumentos além do próprio caminho da ROM.

### TO-DOs

- Adicionar áudio
