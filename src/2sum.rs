// dont know how to export this function to main.rs

// the og code
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![0];
    let mut j: i32 = (nums.len() - 1) as i32;
    while j > 0 {
        ans = vec![j];
        let mut i = j - 1;
        while i >= 0 {
            if nums[j as usize] == target - nums[i as usize] {
                ans.push(i);
                return ans;
            }
            i -= 1;
        }
        j -= 1;
    }
    ans
}

// optimized by AI, very cool
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (j, &num_j) in nums.iter().enumerate().rev() {
        for (i, &num_i) in nums[..j].iter().enumerate() {
            if num_j + num_i == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![0]
}
