use std::fs;

fn main()
{
    let input_data = fs::read_to_string("./inputJ4.txt").unwrap();
    let splitted = input_data.split("\n").collect::<Vec<_>>();

    let mut matrix : Vec<Vec<char>> = Vec::new(); 

    for current in splitted
    {
        matrix.push(current.chars().collect::<Vec<_>>());
    }

    let mut valid_count : i32 = 0;

    for i in 1..matrix.len()-1
    {
        for j in 1..matrix[i].len()-1
        {
            if matrix[i][j] == 'A'
            {
                let mut valid = false;

                if matrix[i-1][j-1] == 'M' && matrix[i+1][j+1] == 'S'
                {
                    valid = true;
                }
                else if matrix[i-1][j-1] == 'S' && matrix[i+1][j+1] == 'M'
                {
                    valid = true;
                }

                if !valid
                {
                    continue;
                }

                let mut valid = false;

                if matrix[i-1][j+1] == 'M' && matrix[i+1][j-1] == 'S'
                {
                    valid = true;
                }
                else if matrix[i-1][j+1] == 'S' && matrix[i+1][j-1] == 'M'
                {
                    valid = true;
                }

                valid_count += valid as i32;
            }
        }
    }

    println!("{}", valid_count);
}