use std::fs;

fn compute_operation(value_sequence : &Vec<i64>, op_sequence : &Vec<char>) -> i64
{
    let mut current_result = value_sequence[0];

    if value_sequence.len() > 1
    {
        let sub_result = compute_operation(&value_sequence[1..].to_vec(), &op_sequence[1..].to_vec());

        if op_sequence[0] == '+'
        {
            current_result += sub_result;
        }
        else if op_sequence[0] == 'c'
        {
            current_result = (sub_result.to_string()+&current_result.to_string()).parse().unwrap();
        }
        else
        {
            current_result *= sub_result;
        }
    }

    return current_result;
}

fn check(value_sequence: &Vec<i64>, result:i64) -> bool
{
    let mut sequences = Vec::new();

    sequences.push(Vec::new());

    for _i in 0..value_sequence.len()-2
    {
        let mut new_sequences : Vec<Vec<char>> = Vec::new();

        for mut current in sequences
        {
            for current_char in ['+', '*', 'c']
            {
                current.push(current_char);
                new_sequences.push(current.clone());
                current.pop();
            }
        }

        sequences = new_sequences;
    }

    for mut op_sequence in sequences
    {
        for current_char in ['+', '*', 'c']
        {
            op_sequence.push(current_char);

            if compute_operation(&value_sequence, &op_sequence) == result
            {
                return true;
            }

            op_sequence.pop();
        }
    }

    return false;
}

fn main()
{
    let input_data = fs::read_to_string("./inputJ7.txt").unwrap();

    let mut final_count = 0;
    for current in input_data.split('\n')
    {
        let splited = current.split(": ").collect::<Vec<_>>();

        let result = splited[0].parse().unwrap();
        let mut sequence = splited[1].split(' ').map(|e| e.parse().unwrap()).collect::<Vec<_>>();
        sequence.reverse();

        if check(&sequence, result)
        {
            final_count += result;
        }
    }

    println!("{}", final_count);
}