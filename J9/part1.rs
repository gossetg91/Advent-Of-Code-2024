use std::fs;

fn main()
{
    let mut input_data = fs::read_to_string("inputJ9.txt").unwrap().chars().map(|e| e.to_string().parse::<i8>().unwrap()).collect::<Vec<_>>();

    let mut i=0;
    let mut j = input_data.len()-1;

    let mut id_debut=0;
    let mut id_fin : i32 = (input_data.len()/2) as i32;

    let mut disk_final : Vec<i32> = Vec::new();


    while i <= j
    {
        if i%2 == 0
        {
            while input_data[i] != 0
            {
                disk_final.push(id_debut);
                input_data[i] -= 1;
            }
            id_debut += 1;
        }
        else
        {
            while input_data[i] != 0
            {
                disk_final.push(id_fin);
                input_data[j] -= 1;
                input_data[i] -= 1;

                if input_data[j] == 0
                {
                    j -= 2;
                    id_fin -= 1;
                    if i > j
                    {
                        break;
                    }
                }
            }
        }
        i += 1;
    }

    let mut check_sum : i128 = 0;

    for i in 0..disk_final.len()
    {
        check_sum += (i as i32 * disk_final[i]) as i128;
    }

    println!("{}", check_sum);
}