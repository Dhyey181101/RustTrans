
use std::cmp::Ordering;

struct Heap {
    n: i64,
}

impl PartialOrd for Heap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.n.cmp(&self.n))
    }
}

impl PartialEq for Heap {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

fn push(h: &mut Vec<Heap>, v: i64) {
    let mut h2 = vec![Heap { n: v }];
    h2.append(h);
    *h = h2;
    let len = h.len();
    let mut i = len / 2;
    while i > 0 {
        let mut j = 2 * i;
        if j + 1 < len && h[j + 1].partial_cmp(&h[j]).unwrap() == Ordering::Less {
            j += 1;
        }
        if h[i].partial_cmp(&h[j]).unwrap() != Ordering::Less {
            break;
        }
        h.swap(i, j);
        i = j;
    }
}

fn pop(h: &mut Vec<Heap>) -> Option<i64> {
    if h.len() == 1 {
        Some(h.pop()?.n)
    } else {
        let len = h.len();
        h.swap(1, len - 1);
        let v = h.pop()?.n;
        let mut i = 1;
        loop {
            let mut j = 2 * i;
            if j + 1 < len && h[j + 1].partial_cmp(&h[j]).unwrap() == Ordering::Less {
                j += 1;
            }
            if h[i].partial_cmp(&h[j]).unwrap() != Ordering::Less {
                break;
            }
            h.swap(i, j);
            i = j;
        }
        Some(v)
    }
}
