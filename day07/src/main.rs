use std::{collections::HashSet, fs};
pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("input")
}

fn parse_input(line: &str) ->Vec<(char, usize)>{
    let mut line_info: Vec<(char, usize)> = Vec::new();

    for (i,ll) in line.chars().enumerate(){
        match ll {
            'S' | '^' => line_info.push((ll,i)),
            _ => {}
        }
    }

    line_info
}

fn main() {
    let raw_input = read_input("day07/src/input07.txt");
    let line_info: Vec<Vec<(char, usize)>> = raw_input.lines().map(parse_input).collect();

    // println!("{:?}", line_info);
    let mut total_splits: usize = 0;
    let mut splits: HashSet<usize> = HashSet::new();
    line_info.iter().for_each(|l_i|  {
        l_i.iter().for_each(|ll_i|{
            let (charr, idx) = ll_i;
            match *charr {
                'S' => { splits.insert(*idx); },
                '^' => {
                    if splits.contains(idx){
                        splits.remove(idx);
                        splits.insert(*idx-1);
                        splits.insert(*idx+1);
                        total_splits+=1;
                    }
                },
                _ => {}
            };
        });
    });
    println!("{}", total_splits);
    let mut splits: Vec<usize> = vec![0;raw_input.lines().collect::<Vec<&str>>()[0].len()];
    splits[line_info[0][0].1] = 1;

    line_info[1..].iter().for_each(|l_i| {
        l_i.iter().for_each(|ll_i|{
            let (charr, idx) = ll_i;
            match *charr {
                '^' => {
                    splits[*idx-1] += splits[*idx];
                    splits[*idx+1] += splits[*idx];
                    splits[*idx] = 0;
                },
                _ => {}
            };
        });
    });

    println!("{}", splits.iter().sum::<usize>());
}
