fn main() {
    let lines: Vec<String> = include_str!("input.txt")
    .lines()
    .map(|line| line.to_string())
    .collect();
    let mut count = 0;
    for i in 3..lines.clone().len() {
        if((lines[i-3].parse::<i32>().unwrap() + lines[i-2].parse::<i32>().unwrap() + lines[i-1].parse::<i32>().unwrap()) < 
        (lines[i-2].parse::<i32>().unwrap() + lines[i-1].parse::<i32>().unwrap() + lines[i].parse::<i32>().unwrap())){
            count+=1;
        }
    }
    print!("{}", count)
}

#[test]
fn test1(){
    assert_eq!(2,2)
}