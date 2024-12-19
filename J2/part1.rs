use std::fs;

fn main() {

    let data = fs::read_to_string("inputJ2.txt").unwrap();
    let mut count = 0;

    for (_,element) in data.split("\n").enumerate()
    {
        let mut good = true;

        let mut previous : i32 = 0;

        for current in element.split(" ").map(|s| s.parse().unwrap()).collect::<Vec<i32>>().windows(2)
        {
            if !(1..4).contains(&(current[0] - current[1]).abs()) ||
                previous != 0 && previous.signum() != (current[0] - current[1]).signum()
            {
                good = false;
                break;
            }
            previous = current[0] - current[1];
        }

        count += good as i32;
    }

    println!("{}",count);

}