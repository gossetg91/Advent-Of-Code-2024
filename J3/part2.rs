use std::fs;

fn main()
{
    let input_data = fs::read_to_string("inputJ3.txt").unwrap();

    let mut final_sum = 0;
    for entry_seq in input_data.split("do()")
    {
        let filtered = entry_seq.split("don't()").collect::<Vec<_>>()[0];

        for current in filtered.split("mul(")
        {
            let extracted = current.split(")").collect::<Vec<_>>()[0].split(",").collect::<Vec<_>>();

            if extracted.len() == 2
            {
                let value1 = extracted[0].parse::<i32>();
                let value2 = extracted[1].parse::<i32>();

                if value1.is_ok() && value2.is_ok()
                {
                    final_sum += value1.unwrap()*value2.unwrap();
                }
            }
        }
    }

    println!("{}", final_sum);
}
