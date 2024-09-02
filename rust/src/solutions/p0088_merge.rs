pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, _n: i32) {
        let mut x = 0;
        for i in m..nums1.len() as i32 {
            nums1[i as usize] = nums2[x];
            x += 1;
        }
        nums1.sort();
    }
}

pub struct Solution2;

impl Solution2 {
    // try O(m + n)
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        
    }
}
