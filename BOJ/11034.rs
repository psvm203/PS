use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();
    
    let output = lines.map(|line| {
        let mut nums: Vec<usize> = line.split(' ').flat_map(|x| x.parse()).collect();
        let mut count = 0;
        loop {
            if nums[0] + 1 == nums[1] && nums[1] + 1 == nums[2] {
                break;
            }
            
            if nums[1] - nums[0] < nums[2] - nums[1] {
                nums[0] = nums[2] - 1;
                nums.sort();
            } else {
                nums[2] = nums[0] + 1;
                nums.sort();
            }
            
            count += 1;
        }
        
        count.to_string()
    }).collect::<Vec<_>>().join("\n");
    
    println!("{output}");
}
