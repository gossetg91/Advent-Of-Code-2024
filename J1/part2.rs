use std::fs;

fn main()
{
    let content = fs::read_to_string("inputJ1.txt").unwrap();

    let mut array1 : Vec<i32> = Vec::new();
    let mut array2 : Vec<i32> = Vec::new();

    for (_, element) in content.split("\n").enumerate()
    {
        let tmp : Vec<_> = element.split("   ").collect();
        array1.push(tmp[0].parse().unwrap());
        array2.push(tmp[1].parse().unwrap());
    }

    let mut sum = 0;
    for element in array1
    {
        let count = array2.iter().filter(|&value| *value == element).count();

        sum += element*count as i32;
    }

    println!("{}", sum);
}