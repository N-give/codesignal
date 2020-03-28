fn main() {
    // a: [9876, 5432, 1999]
    // b: [1, 8001]
    let a = [9876, 5432, 1999].iter().rev().fold(None, |acc, num| {
        let mut prev = Box::new(List::new(*num));
        prev.next = acc;
        Some(prev)
    });
    let b = [1, 8001].iter().rev().fold(None, |acc, num| {
        let mut prev = Box::new(List::new(*num));
        prev.next = acc;
        Some(prev)
    });
    println!("{:?}", a);
    println!("{:?}", b);
    let mut a = add_two_huge_numbers(a, b);
    println!("{:?}", a);
    while let Some(num) = a {
        print!("{} ", num.value);
        a = num.next;
    }
    println!();
}

#[derive(Debug)]
struct List<T> {
    value: T,
    next: Option<Box<List<T>>>,
}

impl<T> List<T> {
    fn new(v: T) -> Self {
        List {
            value: v,
            next: None,
        }
    }
}

type ListNode<T> = Option<Box<List<T>>>;

fn add_two_huge_numbers(
    mut a: ListNode<i32>,
    mut b: ListNode<i32>,
) -> ListNode<i32> {
    let mut next: ListNode<i32> = None;
    let mut carry: i32 = 0;
    loop {
        let (a_temp, a_last) = list_pop(a);
        let (b_temp, b_last) = list_pop(b);
        a = a_temp;
        b = b_temp;

        // TODO this should continue to pull from a longer list until all nodes
        // are in the result list
        match (a_last, b_last) {
            (Some(a1), Some(b1)) => {
                let mut val = a1.value + b1.value + carry;
                if (val / 10000) > 0 {
                    carry = val / 10000;
                    val %= 10000;
                } else {
                    carry = 0;
                }

                let mut prev = Box::new(List::new(val));
                prev.next = next;
                next = Some(prev);
            }

            (Some(a1), None) => {
                let mut val = a1.value + carry;
                if (val / 10000) > 0 {
                    carry = val / 10000;
                    val %= 10000;
                } else {
                    carry = 0;
                }

                let mut prev = Box::new(List::new(val));
                prev.next = next;
                next = Some(prev);
            }

            (None, Some(b1)) => {
                let mut val = b1.value + carry;
                if (val / 10000) > 0 {
                    carry = val / 10000;
                    val %= 10000;
                } else {
                    carry = 0;
                }

                let mut prev = Box::new(List::new(val));
                prev.next = next;
                next = Some(prev);
            }

            (None, None) => break,
        }
    }
    if carry > 0 {
        let mut prev = Box::new(List::new(carry));
        prev.next = next;
        next = Some(prev);
    }

    next
}

fn list_pop(b: ListNode<i32>) -> (ListNode<i32>, ListNode<i32>) {
    println!("{:?}", b);
    match b {
        Some(mut b1) => {
            if b1.next.is_some() {
                let (next, last) = list_pop(b1.next);
                b1.next = next;
                (Some(b1), last)
            } else {
                (None, Some(b1))
            }
        }
        None => (None, None),
    }
}
