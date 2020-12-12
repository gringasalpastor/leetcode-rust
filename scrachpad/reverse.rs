pub fn reverse(list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut previous = None;
    let mut current = list;

    while let Some(ref mut cur) = current {
        let next = cur.next.take();
        cur.next = previous;

        previous = current;
        current = next;
    }

    return previous;
}