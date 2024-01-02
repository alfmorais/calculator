mod interface;
mod logic;
mod math;
mod user_input;


fn main() {
    loop {
        interface::calculator_headers();

        let operation = user_input::user_input_operation_integer();

        if logic::has_operation_in_acceptable_operators(operation) == false {
            interface::calculator_footer();
            break;
        }

        interface::digit_a_number();
        let number_a = user_input::user_input_operation_integer();

        interface::digit_b_number();
        let number_b: i32 = user_input::user_input_operation_integer();

        match operation {
            1 => math::addition(number_a, number_b),
            2 => math::subtraction(number_a, number_b),
            3 => math::multiplication(number_a, number_b),
            4 => math::division(number_a, number_b),
            _ => break,
        };

        interface::clean_terminal();
    }
}
