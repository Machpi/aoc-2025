pub struct Operation {
    pub numbers: Vec<i64>,
    pub operation: char,
}

pub fn process_operation(op: &Operation) -> i64 {
    match op.operation {
        '+' => op.numbers.iter().sum(),
        '*' => op.numbers.iter().product(),
        _ => panic!("Wrong operation"),
    }
}
