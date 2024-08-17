pub struct Solution;

impl Solution {

    // 有点儿慢 50ms 2.4mb
    // pub fn move_zeroes(nums: &mut Vec<i32>) {
    //     if nums.len() < 2 {
    //         return;
    //     }
    //     let mut p1 = 0;
    //     let mut p2 = 1;
    //     while p1 < nums.len() && p2 < nums.len() {
    //         if nums[p1] == 0 {
    //             while  p2 < nums.len() && nums[p2] == 0 {
    //                 p2 += 1;
    //             }
    //             if p1 < nums.len() && p2 < nums.len() {
    //                 let temp = nums[p1];
    //                 nums[p1] = nums[p2];
    //                 nums[p2] = temp;
    //             }
    //         }
    //         p1 += 1;
    //         p2 = p1 + 1;
    //     }
    // }

    // 4ms 2.1mb
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[j] = nums[i];
                if i != j {
                    nums[i] = 0;
                }
                j += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0,1,0,3,2];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1,3,2,0,0]);

        let mut nums = vec![4,2,4,0,0,3,0,5,1,0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![4,2,4,3,5,1,0,0,0,0]);
    }
}