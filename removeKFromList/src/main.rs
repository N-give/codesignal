fn main() {
    let vals = vec![3, 1, 2, 3, 4, 5];
    let list =
        vals.iter()
            .rev()
            .fold(None, |acc: Option<Box<List<i32>>>, e: &i32| {
                let mut next = Box::new(List::new(*e));
                next.next = acc;
                Some(next)
            });

    // println!("{:#?}", list);
    let rem = removeKFromList(list, 3);
    println!("{:#?}", rem);
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

fn removeKFromList(l: ListNode<i32>, k: i32) -> ListNode<i32> {
    if let Some(list) = l {
        if list.value == k {
            return removeKFromList(list.next, k);
        } else {
            let mut ret = Box::new(List::new(list.value));
            ret.next = removeKFromList(list.next, k);
            return Some(ret);
        }
    }
    None
}
