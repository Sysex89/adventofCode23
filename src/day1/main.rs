use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut res: u64 = 0;
   if let Ok(lines) = read_lines("../../input") {
        for line in lines {
          if  let Ok(ip) = line {

                res += get_first_and_last(ip);
            }
        }
       println!("{}", res);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_first_and_last(mut line: String) -> u64 {
    line = convert_to_usable(line);
    let digits: Vec<&u8> = line.as_bytes().iter().filter(|x| x.is_ascii_digit()).collect();
    (((*digits.first().unwrap()- 48 ) * 10) + (*digits.last().unwrap()) - 48).into()
}

fn convert_to_usable(mut line: String) -> String {
    line = line.replace("one", "o1e");
    line = line.replace("two", "t2o");
    line = line.replace("three", "t3e");
    line = line.replace("four", "f4r");
    line = line.replace("five", "f5e");
    line = line.replace("six", "s6x");
    line = line.replace("seven", "s7n");
    line = line.replace("eight", "e8t");
    line.replace("nine", "n9e")
}