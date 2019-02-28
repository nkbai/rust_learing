// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
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

struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut p, mut q, mut carry) = (l1, l2, 0);
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = head.as_mut();

        while p.is_some() || q.is_some() {
            let mut sum = carry;

            if let Some(v) = p {
                sum += v.val;
                p = v.next;
            }

            if let Some(v) = q {
                sum += v.val;
                q = v.next;
            }

            carry = sum / 10;
            if let Some(cur) = current {
                cur.next = Some(Box::new(ListNode::new(sum % 10)));
                current = cur.next.as_mut();
            }
        }
        if carry > 0 {
            current.unwrap().next = Some(Box::new(ListNode::new(carry)));
        }
        head.unwrap().next
    }
}

/* 另外一个解决方法，耗时8ms左右，比这个快4ms
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::kernel(l1, l2, 0)
    }

    fn kernel(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, inc: i32) -> Option<Box<ListNode>> {
        let mut ans = 0;
        let mut next = None;
        match (l1, l2) {
            (Some(l1), Some(l2)) => {
                ans = l1.val + l2.val + inc;
                next = Solution::kernel(l1.next, l2.next, ans / 10);
            }
            (Some(l1), None) => {
                ans = l1.val + inc;
                next = Solution::kernel(l1.next, None, ans / 10);
            }
            (None, Some(l2)) => {
                ans = l2.val + inc;
                next = Solution::kernel(None, l2.next, ans / 10);
            }
            (None, None) => {
                if inc != 0 {
                    return Some(Box::new(ListNode::new(inc)))
                } else {
                    return None
                }
            }
        }
        let mut tmp = ListNode::new(ans % 10);
        tmp.next = next;
        Some(Box::new(tmp))
    }
}
*/

/*
给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。

如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。

您可以假设除了数字 0 之外，这两个数都不会以 0 开头。

示例：

输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
输出：7 -> 0 -> 8
原因：342 + 465 = 807
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut l1 = Some(Box::new(ListNode::new(1)));
        let mut l2 = Some(Box::new(ListNode::new(9)));

        l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

        println!("l1 {:?}", l1);
        println!("l2 {:?}", l2);
        let mut x = Solution::add_two_numbers(l1, l2);
        println!("res {:?}", x);
    }
}

fn main() {
    let mut l1 = Some(Box::new(ListNode::new(1)));
    let mut l2 = Some(Box::new(ListNode::new(9)));

    l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

    println!("l1 {:?}", l1);
    println!("l2 {:?}", l2);
    let mut x = Solution::add_two_numbers(l1, l2);
    println!("res {:?}", x);
}