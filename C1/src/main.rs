type Heap<T> = Vec<T>;

fn heapify<T>(mut h: Heap<T>, mut i: usize, gt: fn(&T, &T) -> bool) -> Heap<T> {
    let n = h.len();
    loop {
        let l = 2 * i + 1;
        let r = 2 * i + 2;

        let mut largest = i;

        if l < n && gt(&h[l], &h[largest]) {
            largest = l;
        }
        if r < n && gt(&h[r], &h[largest]) {
            largest = r;
        }

        if largest != i {
            h.swap(i, largest);
            i = largest;
        } else {
            break;
        }
    }
    h
}

fn reheapify<T>(mut h: Heap<T>, mut i: usize, gt: fn(&T, &T) -> bool) -> Heap<T> {
    while i > 0 {
        let parent = (i - 1) / 2;
        if gt(&h[i], &h[parent]) {
            h.swap(i, parent);
            i = parent;
        } else {
            break;
        }
    }
    h
}

fn vec_to_heap<T>(xs: Vec<T>, gt: fn(&T, &T) -> bool) -> Heap<T> {
    let mut h: Heap<T> = Vec::with_capacity(xs.len());
    for (i, x) in xs.into_iter().enumerate() {
        h.push(x);
        h = reheapify(h, i, gt);
    }
    h
}

fn heap_to_vec<T>(mut h: Heap<T>, gt: fn(&T, &T) -> bool) -> Vec<T> {
    let mut out: Vec<T> = Vec::with_capacity(h.len());
    while !h.is_empty() {
        let n = h.len();
        h.swap(0, n - 1);
        out.push(h.pop().unwrap());
        if !h.is_empty() {
            h = heapify(h, 0, gt);
        }
    }
    out
}

fn hsort<T>(xs: Vec<T>, gt: fn(&T, &T) -> bool) -> Vec<T> {
    heap_to_vec(vec_to_heap(xs, gt), gt)
}

fn main() {
    let xs: Vec<u64> = vec![2, 4, 6, 8, 5, 3, 7];

    fn f(x: &u64, y: &u64) -> bool {
        x > y
    }

    dbg!(&xs);
    let sorted = hsort(xs, f);
    dbg!(&sorted);
}

