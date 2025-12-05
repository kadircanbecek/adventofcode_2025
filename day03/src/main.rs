use std::fs;
pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("input")
}

fn parse_line(line: &str)->Option<Vec<i64>>{
    let parts = line.chars();
    let mut data: Vec<i64>= Vec::new();
    for part in parts{
        match part.to_string().parse(){
            Ok(n)=> data.push(n),
            _ => return None
        };
    }
    Some(data)
}

fn find_batteries (nums: Vec<i64>) -> i64{
    let mut index1 = 0;
    let mut max_val1 = 0;
    let mut max_index1 = 0;
    for numm in &nums{
        if *numm>max_val1 {
            max_val1 = *numm;
            max_index1 = index1;
        }
        index1+=1;
    }

    if max_index1 != nums.len()-1 {
        let mut max_val2 = 0;
        
        for index2 in max_index1+1..nums.len(){
            if nums[index2]>max_val2 {
                max_val2 = nums[index2];
            }
        }

        let total_val = max_val1*10+max_val2;
        println!("{}", total_val);
        return total_val;
    } else {
        let mut max_val2 = 0;
        for index2 in 0..max_index1{
            if nums[index2]>max_val2 {
                max_val2 = nums[index2];
            }
        }

        let total_val = max_val2*10 + max_val1;
        println!("{}", total_val);
        return total_val;
    }

    
}
fn vector_argmax(nums: Vec<i64>)->(i64, usize){
    let mut max_index=0;
    let mut max_val=0;
    for i in 0..nums.len(){
        if max_val<nums[i]{
            max_index = i;
            max_val = nums[i];
        }
    }
    return (max_val, max_index);
}

fn find_batteries_p2 (nums: Vec<i64>) -> i64{

    let mut total_val = 0;
    let mut index = 12;
    let mut curr_idx = 0;
    while index>0{
        let (max_val, max_idx) = vector_argmax(nums[curr_idx..nums.len()-index+1].to_vec());
        index-=1;
        curr_idx+=max_idx+1;
        // println!("{}", max_idx+1);
        total_val= total_val*10+max_val
    }
    // println!("{}", total_val);
    total_val
}
fn main() {
    let raw_input = read_input("day03/src/input03.txt");
    let part1_res: i64 = raw_input.lines().filter_map(parse_line).map(find_batteries).sum();
    println!("{}", part1_res);
    let part2_res: i64 = raw_input.lines().filter_map(parse_line).map(find_batteries_p2).sum();
    println!("{}", part2_res);

}
