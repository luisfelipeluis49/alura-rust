fn main() {

    //let notas: [f32; 4] = [6.5; 4];
    let notas: [f32; 4] = [10.0, 8.0, 9.5, 6.0];
    let inteiro: usize = 0; //can't cast i32 nor i64

    println!("{}", notas[inteiro]);

    //println!("Notas: {}, {}, {} e {}", notas[0], notas[1], notas[2], notas[3]);
    for indice in 0..notas.len() {
        println!("A nota {} é: {}", indice + 1, notas[indice]);
    }

    matriz();

    println!("É fim de semana? {}", fim_de_semana(DiaDaSemana::Quarta));

    //let dia: DiaDaSemana = DiaDaSemana::Sexta;
    
    optional();

    currency_account();
}

#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta, 
    Quinta, 
    Sexta, 
    Sabado
}

fn fim_de_semana(dia_da_semana: DiaDaSemana) -> bool {
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false
    }
}

fn matriz() {
    let matriz: [[f32; 3]; 2] = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4]
    ];

    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);
        }
    }
}

fn optional() {
    let conteudo = read_file(String::from(""));

    match conteudo {
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo não existe")
    }
}

fn read_file(_path_file: String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
}

struct Titular {
    nome: String,
    sobrenome: String
}

struct Conta {
    titular: Titular,
    saldo: f64
}

impl Conta {
    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor;
    }
}

fn currency_account() {
    let mut conta: Conta = Conta { 
        titular: Titular { 
            nome: String::from("Luís"), 
            sobrenome: String::from("Silva")
        }, 
        saldo: 1500.00 
    };

    conta.sacar(325.00);

    println!("{} {} tem R$:{} em conta corrente", 
        conta.titular.nome, 
        conta.titular.sobrenome, 
        conta.saldo
    );
}
