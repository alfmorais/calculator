use std::io;

pub fn user_input_operation_integer() -> i32 {
    let mut operation = String::new();

    io::stdin()
        .read_line(&mut operation)
        .expect("Digito não reconhecido");

    let integer_operation: i32 = operation.trim().parse().expect("Você não digitou um número");
    // TODO: Tratar esse erro saindo do loop no programa principal
    return integer_operation;
}


