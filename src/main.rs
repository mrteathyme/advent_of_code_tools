use std::env;

#[tokio::main]
async fn main() {
    println!("{:?}",day1().await);
}

async fn get_puzzle() -> String {
    let token = env::var("AOC_SESSION");
    advent_of_code_tools::puzzle_input(token.expect("env var AOS_SESSION not set"),None,Some(1)).await.unwrap()
}

async fn day1() -> String {
    let puzzle_input = get_puzzle().await;
    let lines = puzzle_input.lines();
    let mut lines_vec = vec![];
    let mut elves = vec![0];
    let mut max = 0;
    let mut i = 0;
    for line in lines {
        if line.is_empty() == true {
            if elves[i] > max {
                max = elves[i];
            }
            i += 1;
            elves.push(0);
            }
        else {   
        let line_int = line.parse::<i32>().unwrap();
        lines_vec.push(line_int);
        elves[i] += line_int;
        }
    }
    lines_vec.sort();
    max.to_string()

}
