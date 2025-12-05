use std::fs;
pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("input")
}

fn part1_sol(map: Vec<Vec<char>>)-> i32{
    let H = map.len() ;
    let W = map[0].len();

    let mut accessible = 0;

    for h in 0..H {
        for w in 0..W{
            if map[h][w]!='@'{
                continue;
            }
            let mut perip_rolls = 0;
            for i in 0..3{
                for j in 0..3{
                    let h_index = h as i32 + i as i32 - 1;
                    let w_index = w as i32 + j as i32 - 1;
                    if h_index == h as i32 && w_index ==w as i32{
                        continue;
                    }
                    if h_index<0 || h_index>=(H as i32){
                        continue;
                    }
                    if w_index<0 || w_index>=(W as i32){
                        continue;
                    }
                    if map[h_index as usize][w_index as usize] == '@'{
                        perip_rolls +=1;
                    }
                }
            }
            if perip_rolls<4 {
                accessible+=1;
            }
            // println!("{h} {w} {perip_rolls}");
        }
    }

    accessible
}

fn part2_sol(map: Vec<Vec<char>>)-> i32{
    let H = map.len() ;
    let W = map[0].len();

    let mut total_accessible = 0;
    let mut new_map = map.clone();
    loop {
        let mut accessible = 0;
        let mut rem_coord: Vec<(usize, usize)> = Vec::new();
        for h in 0..H {
            for w in 0..W{
                if new_map[h][w]!='@'{
                    continue;
                }
                let mut perip_rolls = 0;
                for i in 0..3{
                    for j in 0..3{
                        let h_index = h as i32 + i as i32 - 1;
                        let w_index = w as i32 + j as i32 - 1;
                        if h_index == h as i32 && w_index ==w as i32{
                            continue;
                        }
                        if h_index<0 || h_index>=(H as i32){
                            continue;
                        }
                        if w_index<0 || w_index>=(W as i32){
                            continue;
                        }
                        if new_map[h_index as usize][w_index as usize] == '@'{
                            perip_rolls +=1;
                        }
                    }
                }
                if perip_rolls<4 {
                    accessible+=1;
                    rem_coord.push((h,w));
                }
                // println!("{h} {w} {perip_rolls}");
            }
        }
        if accessible == 0 {
            break;
        }

        total_accessible += accessible;
        for (h,w) in rem_coord{
            new_map[h][w] = '.';
        }
    }
    total_accessible
}


fn main() {
    let raw_input = read_input("day04/src/input04.txt");
    let map: Vec<Vec<char>> = raw_input.lines().map(|line| line.chars().collect()).collect();
    let part1_res = part1_sol(map.clone());
    println!("{}", part1_res);
    let part2_res = part2_sol(map.clone());
    println!("{}", part2_res);
}
