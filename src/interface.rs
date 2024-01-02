use std::{thread, time::Duration};

pub fn calculator_headers() -> () {
    println!("{:-^100}", " Calculadora ");
    println!(
        "Escolha qual operação deseja realizar: \n\
        1: Soma\n\
        2: Subtração\n\
        3: Multiplicação\n\
        4: Divisão\n\
        Qualquer tecla: Sair\n"
    );
}

pub fn calculator_footer() -> () {
    println!("{:-^100}", " Fim ");
}

pub fn digit_a_number() -> () {
    println!("Digite o primeiro número: ");
}

pub fn digit_b_number() -> () {
    println!("Digite o segundo número: ");
}

pub fn clean_terminal() -> () {
    thread::sleep(Duration::from_millis(3000));
    print!("{esc}c", esc = 27 as char);
}