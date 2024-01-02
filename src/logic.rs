pub fn has_operation_in_acceptable_operators(user_operation: i32) -> bool {
    let acceptable_operation = vec![1,2,3,4];

    for &operation in acceptable_operation.iter() {
        if operation == user_operation {
            return true;
        }
    }

    return false;
}