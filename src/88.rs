impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut idx = (m + n) as usize;
        let mut i = m as usize;
        let mut j = n as usize;
        while idx > 0 {
            if i > 0 && j > 0 {
                if nums1[i - 1] > nums2[j - 1] {
                    nums1[idx - 1] = nums1[i - 1];
                    i -= 1;
                } else {
                    nums1[idx - 1] = nums2[j - 1];
                    j -= 1;
                }
            } else if i > 0 {
                nums1[idx - 1] = nums1[i - 1];
                i -= 1;
            } else {
                nums1[idx - 1] = nums2[j - 1];
                j -= 1;
            }
            idx -= 1;
        }
    }
}