// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut list1, mut list2) = (list1, list2);
        let mut node = None;
        let mut cur = &mut node;
        *cur = loop {
            match (list1, list2) {
                (Some(mut a), Some(mut b)) => {
                    if a.val < b.val {
                        list1 = a.next.take();
                        list2 = Some(b);
                        cur = &mut cur.insert(a).next;
                    } else {
                        list1 = Some(a);
                        list2 = b.next.take();
                        cur = &mut cur.insert(b).next;
                    }
                },
                (x, y) => break x.or(y),
            }
        };
        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        for &val in values.iter() {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        dummy.next
    }

    fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = list;
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        result
    }

    #[test]
    fn test_merge_two_lists() {
        // 测试用例1：两个非空列表
        let list1 = create_list(vec![1, 2, 4]);
        let list2 = create_list(vec![1, 3, 4]);
        let merged = Solution::merge_two_lists(list1, list2);
        assert_eq!(list_to_vec(merged), vec![1, 1, 2, 3, 4, 4]);

        // 测试用例2：一个空列表和一个非空列表
        let list1 = create_list(vec![]);
        let list2 = create_list(vec![0]);
        let merged = Solution::merge_two_lists(list1, list2);
        assert_eq!(list_to_vec(merged), vec![0]);

        // 测试用例3：两个空列表
        let list1 = create_list(vec![]);
        let list2 = create_list(vec![]);
        let merged = Solution::merge_two_lists(list1, list2);
        assert_eq!(list_to_vec(merged), vec![]);

        // 测试用例4：不同长度的列表
        let list1 = create_list(vec![1, 3, 5]);
        let list2 = create_list(vec![2, 4, 6, 8]);
        let merged = Solution::merge_two_lists(list1, list2);
        assert_eq!(list_to_vec(merged), vec![1, 2, 3, 4, 5, 6, 8]);
    }
}