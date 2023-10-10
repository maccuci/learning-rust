use std::io::*;
use rand::prelude::*;

fn main() {
    // average();
    // fahrenheit_conversor();
    // todo_list();
    guess_the_number();
}

fn average() {
    println!("Digite uma lista de números separados por espaço:");

    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Falha ao ler a linha");
    let numbers: Vec<isize> = input
        .split_whitespace()
        .map(|s: &str| s.trim().parse().expect("Falha ao converter o número."))
        .collect();

    if numbers.is_empty() {
        println!("A lista está vazia.");
        return;
    }

    let sum: isize = numbers.iter().sum();
    let average: isize = sum / numbers.len() as isize;
    println!("Números inseridos: {:?}", numbers);
    print!("Média: {:?}", average);
}

fn fahrenheit_conversor() {
    println!("Digite a temperatura em Celsius:");
    let mut temperature_input: String = String::new();
    stdin()
        .read_line(&mut temperature_input)
        .expect("Falha ao ler a linha");
    let temperature: f64 = temperature_input
        .trim()
        .parse()
        .expect("Falha ao converter o número.");
    let result: f64 = temperature * 1.8 + 32.0;
    println!("Temperatura em Fahrenheit: {:.2}", result);
}

fn todo_list() {
    let mut todo_list: Vec<String> = Vec::new();

    loop {
        println!("Insira uma nova tarefa (Para sair digite 'sair'): ");
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Falha ao ler a linha");

        let input = input.trim();

        if input == "sair" {
            break;
        }

        todo_list.push(input.to_string());
        println!("Tarefas: {:?}", todo_list);
    }
}

fn guess_the_number() {
    println!("Insira um número para adivinhar: ");
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Falha ao ler a linha");
    
    let random_number: isize = rand::thread_rng().gen_range(1..10);

    let ans: isize = input.trim().parse().unwrap();
    if ans == random_number {
        println!("Parabéns! Você acertou o número!");
    } else {
        println!("Você errou o número! O número correto era: {:?}", random_number);
    }
}