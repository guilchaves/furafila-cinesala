# Furafila Cinesala
Uma aplicação em CLI desenvolvida em Rust com o propósito de me avisar quando o [Cinesala](https://www.veloxtickets.com/Portal/Local/Cinema/Sao-Paulo/CINESALA/CSL/) está com novas sessões.

## Motivação

Não precisar ficar apertando Ctrl + Shift + R na página de bilhetes do Cinesala para saber se existem novas sessões de cinema.
O cinesala apenas disponibiliza ingressos para sessões semanais, sendo um dos poucos cinemas com sofázinho duplo, que são sempre super concorridos. Como um bom namorado resolvi automatizar esse processo e sempre garantir filme e sofázinho para eu e minha amada 🥰.

## Como funciona 

A aplicação utiliza técnicas de web scraping para extrair informaçòes do site do cinema. Ele verifica as atualizações nos dias de sessão comparando o cronograma de sessão atual com o cronograma armazenado anteriormente.
Quando novas sessões são detectadas, ele envia uma notificação para o console do usuário.

![img](./cli.png)

## Todo

- Adicionar integração com o Telegram, para notificação em tempo real via celular.

## Instalação 

1. Clone o repositório
```bash
git clone https://github.com/seudomdeusuario/furafila-cinesala.git
```

2. Navegue até o diretório do projeto
```bash
cd furafila-cinesala
```

3. Compile o projeto usando o Cargo
```bash
cargo build --release
```

## Rodar o projeto

Dentro de diretório do projeto, execute
```bash
./target/release/furafila-cinesala
```

## Licença

Este projeto está licenciado sobre Licença Creative Commons - consule o arquivo [LICENSE](https://github.com/guilchaves/furafila-cinesala/blob/main/LICENSE) para obter detalhes.





