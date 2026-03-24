use std::collections::HashMap;

// Marcando o que é um Produto na MegaStore.
#[derive(Debug, Clone)]
struct Produto {
    id: u32,
    nome: String,
    preco: f64,
    categoria: String,
}

fn main() {
    // Criando a Tabela Hash para guardar os produtos
    let mut catalogo = HashMap::new();

    // Alguns produtos de exemplo para testar
    let p1 = Produto {
        id: 10,
        nome: String::from("Notebook"),
        preco: 1500.0,
        categoria: String::from("Eletronicos"),
    };

    let p2 = Produto {
        id: 20,
        nome: String::from("Caderno"),
        preco: 25.50,
        categoria: String::from("Papelaria"),
    };

    // Nessa parte é criado o catálogo usando o ID
    catalogo.insert(p1.id, p1);
    catalogo.insert(p2.id, p2);

    println!("--- SISTEMA DE BUSCA MEGASTORE ---");

    // Nesse código simula a busca do produto pelo ID 10
    let id_procurado = 10;
    
    // O match serve para checar se encontrou ou não (demorei pra achar resolução disso)
    match catalogo.get(&id_procurado) {
        Some(produto_achado) => {
            println!("Produto encontrado!");
            println!("Nome: {}, Preço: R${}", produto_achado.nome, produto_achado.preco);
        },
        None => println!("Erro: Produto com ID {} não existe.", id_procurado),
    }
}

// --- ÁREA DE TESTES ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_se_adiciona_produto() {
        let mut lista = HashMap::new();
        let prod = Produto { id: 1, nome: String::from("Teste"), preco: 1.0, categoria: String::from("C") };
        
        lista.insert(prod.id, prod);
        
        // Essa parte verifica se a lista agora tem 1 item
        assert_eq!(lista.len(), 1);
    }

    #[test]
    fn teste_busca_por_id() {
        let mut lista = HashMap::new();
        let prod = Produto { id: 5, nome: String::from("Mouse"), preco: 50.0, categoria: String::from("TI") };
        lista.insert(prod.id, prod);

        let busca = lista.get(&5);
        assert!(busca.is_some());
    }
}