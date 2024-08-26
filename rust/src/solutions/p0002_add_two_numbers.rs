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

    // 3ms 2.1mb
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
                    if temp == 0 {
                        break x.or(y);
                    }
                    let mut l3 = x.or(y);
                    let mut node = None;
                    let mut cur2 = &mut node;
                    *cur2 = loop {
                        if let Some(mut c) = l3 {
                            let sum = c.val + temp;
                            if sum >= 10 {
                                c.val = sum - 10;
                                temp = 1;
                            } else {
                                c.val = sum;
                                temp = 0;
                            }
                            l3 = c.next.take();
                            cur2 = &mut cur2.insert(c).next;
                        } else {
                            if temp == 1 {
                                cur2 = &mut cur2.insert(Box::new(ListNode::new(1))).next;
                            }
                            break None;
                        }
                    };

                    break node;
                }
            }
        };

        node
    }
}
