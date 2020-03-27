fn main() {
    let list =
        [1, 1, 1]
            .iter()
            .rev()
            .fold(None, |acc: ListNode<i32>, value: &i32| {
                let mut prev = Box::new(List::new(*value));
                prev.next = acc;
                Some(prev)
            });
    println!("{}", isListPalindrome(list));
}

struct List<T> {
    value: T,
    next: Option<Box<List<T>>>,
}

impl<T> List<T> {
    fn new(value: T) -> Self {
        List { value, next: None }
    }
}

type ListNode<T> = Option<Box<List<T>>>;

fn isListPalindrome(mut l: ListNode<i32>) -> bool {
    let mut list = Vec::new();
    while let Some(l1) = l {
        list.push(l1.value);
        l = l1.next;
    }

    let len = list.len();
    for i in 0..(len / 2) {
        if list[i] != list[len - 1 - i] {
            return false;
        }
    }
    true
}
