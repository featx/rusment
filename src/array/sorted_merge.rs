extern crate rusment;

mod array {
    pub struct Solution {

    }
}

impl Solution {
    fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut temps = Vec::new();
        let mut i = 0;
        let mut j = 0;
        loop {
            if nums1[i] > nums2[j] {
                temps.push(nums1[i + 1]);
                nums1[i + 1] = nums1[i];
                nums1[i] = nums2[j];
                i += 1;
                j += 1;
            } else if temps[0] < nums1[i] {
                temps.push(nums1[i]);
                nums1[i] = temps.pop();
            } else {
                i += 1;
            }
            if  (i == m) && (j == n) {
                break
            }
        }
    }
}