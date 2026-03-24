Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore
1. Sobre o Projeto
Esse projeto foi feito pra matéria de Data Structures Strategy and Implementation da faculdade.

A ideia aqui é resolver um problema real: a "MegaStore" é uma loja gigante que vende de tudo na internet, mas o sistema de busca deles era muito lento e ruim. Os clientes digitavam o nome do produto e demorava pra aparecer, ou às vezes nem aparecia direito. Isso fazia o pessoal desistir de comprar e ir pra outro site.

Então, o desafio foi criar um sistema de busca novo, mais rápido, usando a linguagem Rust e uma estrutura de dados chamada Tabela Hash (que é tipo um "índice" pra encontrar as coisas quase na hora). Assim, o cliente encontra o que quer rapidinho e a empresa (fictícia) vende mais.

2. Tecnologias que Usei

Rust:  Linguagem de programação que usei pra codar tudo
Cargo: É o "gerente" do Rust, ele ajuda a compilar e testar o código
HashMap: É a estrutura de dados da biblioteca padrão do Rust.

3. Como Rodar o Sistema
Se você quiser testar o projeto no seu computador siga os passos:

Primeiro, você precisa ter o Rust instalado. Dá pra baixar no site oficial.

Abre o terminal na pasta do projeto.

Digita esse comando:

cargo run

Pronto! O programa vai compilar e mostrar um exemplo de busca funcionando.

4. Como Rodar os Testes

Pra garantir que tudo tá funcionando, foi adicionado alguns testes automáticos. Pra rodar eles, é só digitar:

cargo test

O Rust vai executar os testes e mostrar se passou ou se deu algum erro. Assim a gente tem certeza que a busca tá confiável.

5. Como Funciona (Arquitetura e Algoritmos)

Criei uma estrutura chamada Produto que guarda as informações: ID, nome, preço e categoria.

Usei uma Tabela Hash (HashMap) pra armazenar os produtos. A chave é o ID do produto, e o valor é o produto em si.

Por que a Tabela Hash?

Porque após várias pesquiss descobri que ela é muito rápida! Em vez de ficar procurando produto por produto (igual o sistema antigo fazia), ela encontra direto usando a chave. 

6. Desempenho e Escalabilidade

A solução foi resolve o problema de lentidão porque não precisa varrer a lista inteira cada vez que alguém faz uma busca. O que conforme o catálogo da MegaStore crescer, o sistema continua rápido.

O Rust também ajuda bastante nisso, porque ele gerencia a memória de forma segura e eficiente. Ou seja, o sistema não vai ficar mais lento com o tempo.

7. Contribuições e Licença

Licença: Tô usando a licença MIT, que é bem tranquila e permite que outras pessoas usem o código se quiserem.

Considerações Finais
No começo é bem complicado pois nas aulas trabalhamos com python, mas com muita pesquisa e algumas ajudas deu tudo certo! Não conhecia o Rust, Mas com os conceitos de tabelas hash e a prática em Rust junto com ajuda do vscode (kkk), deu pra entender bem melhor como otimizar buscas.

É isso!