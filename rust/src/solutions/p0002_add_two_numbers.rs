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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut node = None;
        let mut cur = &mut node;
        let mut temp = 0;
        *cur = loop {
            match (l1, l2) {
                (Some(mut a), Some(mut b)) => {
                    let sum = a.val + b.val + temp;
                    if sum >= 10 {
                        a.val = sum - 10;
                        temp = 1;
                    } else {
                        a.val = sum;
                        temp = 0;
                    }
                    l1 = a.next.take();
                    l2 = b.next.take();
                    cur = &mut cur.insert(a).next;
                },
                (x, y) => {
                    let mut node = x.or(y);
                    let res = loop {
                        match node {
                            Some(mut c) => {
                                c.val = 
                            },
                            None => b
                        }
                    };

                    break node;
                }
            }
        };

        node
    }
}
