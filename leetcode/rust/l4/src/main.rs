use std::cmp::min;

fn do_find_median_sorted_array(nums: &[i32]) -> f64 {
    if 0 == nums.len() % 2 {
        let middle = nums.len() >> 1;
        (nums[middle - 1] + nums[middle]) as f64 / 2.0
    } else {
        (nums[nums.len() >> 1]) as f64
    }
}

fn array_find_greater(nums: &[i32], start: usize, end: usize, v: i32) -> Option<usize> {
    assert!(end <= nums.len());

    if start == end {
        return None;
    }

    assert!(start < end);

    let middle = (end + start) >> 1;

    if middle == start {
        if nums[start] > v {
            return Some(start);
        }

        return None;
    }

    if nums[middle] > v {
        match array_find_greater(nums, start, middle, v) {
            Some(i) => Some(i),
            _ => Some(middle),
        }
    } else {
        array_find_greater(nums, middle, end, v)
    }
}

fn do_find_median_sorted_arrays(is_even: bool, nums1: &[i32], nums2: &[i32]) -> f64 {
    if 0 == nums1.len() {
        return do_find_median_sorted_array(nums2);
    }

    if 0 == nums2.len() {
        return do_find_median_sorted_array(nums1);
    }

    if nums1[0] > nums2[0] {
        return do_find_median_sorted_arrays(is_even, nums2, nums1);
    }

    let last1 = nums1.last().unwrap();

    if last1 <= &nums2[0] {
        if nums1.len() > nums2.len() {
            return do_find_median_sorted_array(&nums1[nums2.len()..]);
        } else if nums1.len() < nums2.len() {
            return do_find_median_sorted_array(&nums2[..(nums2.len() - nums1.len())]);
        } else {
            return (last1 + nums2[0]) as f64 / 2.0;
        }
    }

    if nums2.len() == 1 {
        if nums1.len() == 2 {
            return nums2[0] as f64;
        }

        let middle = nums1.len() >> 1;
        if is_even {
            if nums1[middle - 1] >= nums2[0] {
                return ((nums1[middle - 1] + nums1[middle]) as f64) / 2.0;
            } else if nums1[middle + 1] <= nums2[0] {
                return ((nums1[middle + 1] + nums1[middle]) as f64) / 2.0;
            } else {
                return ((nums2[0] + nums1[middle]) as f64) / 2.0;
            }
        } else {
            if nums1[middle - 1] >= nums2[0] {
                return nums1[middle - 1] as f64;
            } else if nums1[middle] <= nums2[0] {
                return nums1[middle] as f64;
            } else {
                return nums2[0] as f64;
            }
        }
    }

    if nums2.len() == 2 {
        if nums1.len() == 2 {
            if nums2[1] >= nums1[1] {
                return ((nums2[0] + nums1[1]) as f64) / 2.0;
            } else {
                return ((nums2[0] + nums2[1]) as f64) / 2.0;
            }
        }
    }

    let last2 = nums2.last().unwrap();

    let left = array_find_greater(nums1, 0, nums1.len(), nums2[0]).unwrap();

    if last2 > last1 {
        let right = array_find_greater(nums2, 0, nums2.len(), *last1).unwrap();
        let n = min(left, nums2.len() - right);
        assert!(n > 0);

        return do_find_median_sorted_arrays(is_even, &nums1[n..], &nums2[..(nums2.len() - n)]);
    } else if last1 > last2 {
        let right = array_find_greater(nums1, left, nums1.len(), *last2).unwrap();
        let n = min(left, nums1.len() - right);
        assert!(n > 0);

        return do_find_median_sorted_arrays(is_even, &nums1[n..(nums1.len() - n)], &nums2[..]);
    } else {
        return do_find_median_sorted_arrays(is_even, &nums1[1..], &nums2[..(nums2.len() - 1)]);
    }
}

struct Solution;

impl Solution {
    #![allow(dead_code)]
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let is_even = 0 == (nums1.len() + nums2.len()) % 2;
        return do_find_median_sorted_arrays(is_even, &nums1[..], &nums2[..]);
    }
}

fn main() {
    println!("Hello, world!");
}
