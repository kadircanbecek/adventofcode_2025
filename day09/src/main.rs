use std::{cmp::{max, min}, fs};
pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("input")
}

fn parse_line(line: &str)->Option<(i64, i64)>{
    let parts: Vec<&str> = line.split(",").collect();
    if parts.len()!=2 {
        return None;
    }
    let x: i64 = match parts[0].parse(){
        Ok(n) => n,
        _ => return None
    };
    
    let y: i64 = match parts[1].parse(){
        Ok(n) => n,
        _ => return None
    };

    Some((x,y))
}
fn solve_part1(xycoords: &Vec<(i64,i64)>)->i64{
    let mut max_area = 0;
    for i in 0..xycoords.len(){
        for j in i+1..xycoords.len(){
            let x_diff = (xycoords[i].0 - xycoords[j].0).abs() +1;
            let y_diff = (xycoords[i].1 - xycoords[j].1).abs() +1;
            let area = x_diff*y_diff;
            if area> max_area{
                max_area = area;
            }
        }
    }

    max_area
}

fn solve_part2(xycoords: &Vec<(i64,i64)>)->i64{
    let mut max_area = 0;
    for i in 0..xycoords.len(){
        for j in i+1..xycoords.len(){
            let (x1,y1) = xycoords[i];
            let (x2,y2) = xycoords[j];
            let mut cross = false;
            let rect_min_x= min(x1,x2);
            let rect_max_x= max(x1,x2);
            let rect_min_y= min(y1,y2);
            let rect_max_y= max(y1,y2);
            for k in 0..xycoords.len(){
                let l =(k+1)%xycoords.len();
                let (x3,y3) = xycoords[k];
                let (x4,y4) = xycoords[l];
                let edge_min_x= min(x3,x4);
                let edge_max_x= max(x3,x4);
                let edge_min_y= min(y3,y4);
                let edge_max_y= max(y3,y4);
                if( rect_min_x<edge_max_x) &&(rect_max_x> edge_min_x) && 
                (rect_min_y<edge_max_y) &&(rect_max_y>edge_min_y){
                    cross =true;
                    break;
                }
            }
            if !cross{
                let x_diff = (x1 - x2).abs() +1;
                let y_diff = (y1 - y2).abs() +1;
                let area = x_diff*y_diff;
                if area> max_area{
                    max_area = area;
                }
            }
        }
    }

    max_area
}
fn main() {
    let raw_input = read_input("day09/src/input09.txt");
    let input_pairs: Vec<(i64, i64)> = raw_input.lines().filter_map(parse_line).collect();

    let result_part1 = solve_part1(&input_pairs);
    println!("{}", result_part1);
    let result_part2 = solve_part2(&input_pairs);
    println!("{}", result_part2);
}
