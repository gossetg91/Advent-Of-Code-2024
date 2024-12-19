use std::fs;

fn main()
{
    let input_data = fs::read_to_string("./inputJ4.txt").unwrap();
    let splitted = input_data.split("\n").collect::<Vec<_>>();

    let mut matrix : Vec<Vec<char>> = Vec::new(); 
    let sequence = ['X', 'M', 'A', 'S'];

    for current in splitted
    {
        matrix.push(current.chars().collect::<Vec<_>>());
    }

    let mut valid_count : i32 = 0;

    for i in 0..matrix.len()
    {
        for j in 0..matrix[i].len()
        {
            if matrix[i][j] == sequence[0]
            {
                let mut dir_ok = [true, true, true, true, true, true, true, true];

                for k in 1..sequence.len()
                {
                    dir_ok[0] &= (i as i32 - k as i32) >=0 && matrix[i-k][j] == sequence[k];
                    dir_ok[1] &= i+k < matrix.len() && matrix[i+k][j] == sequence[k];
                    dir_ok[2] &= (j as i32 - k as i32) >=0 && matrix[i][j-k] == sequence[k];
                    dir_ok[3] &= j+k < matrix[i].len() && matrix[i][j+k] == sequence[k];

                    dir_ok[4] &= (i as i32 - k as i32) >=0 && (j as i32 - k as i32) >=0 && matrix[i-k][j-k] == sequence[k];
                    dir_ok[5] &= i+k < matrix.len() && j+k < matrix[i].len() && matrix[i+k][j+k] == sequence[k];
                    dir_ok[6] &= (i as i32 - k as i32) >=0 && j+k < matrix[i].len() && matrix[i-k][j+k] == sequence[k];
                    dir_ok[7] &= i+k < matrix.len() && (j as i32 - k as i32) >=0 && matrix[i+k][j-k] == sequence[k];

                    if dir_ok.iter().all(|&e| !e)
                    {
                        break;
                    }
                }
                valid_count += dir_ok.iter().map(|&elem| elem as i32 ).sum::<i32>();
            }
        }
    }

    println!("{}", valid_count);
}