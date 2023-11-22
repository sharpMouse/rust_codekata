#![cfg(test)]

use std::cmp::Ordering;

fn test_chop(chop: &dyn Fn(i32, &[i32]) -> isize, name: &str) {
    println!("{}", name);
    
    assert_eq!(-1, chop(3, &[]));
    assert_eq!(-1, chop(3, &[1]));
    assert_eq!(0,  chop(1, &[1]));

    assert_eq!(0,  chop(1, &[1, 3, 5]));
    assert_eq!(1,  chop(3, &[1, 3, 5]));
    assert_eq!(2,  chop(5, &[1, 3, 5]));
    assert_eq!(-1, chop(0, &[1, 3, 5]));
    assert_eq!(-1, chop(2, &[1, 3, 5]));
    assert_eq!(-1, chop(4, &[1, 3, 5]));
    assert_eq!(-1, chop(6, &[1, 3, 5]));

    assert_eq!(0,  chop(1, &[1, 3, 5, 7]));
    assert_eq!(1,  chop(3, &[1, 3, 5, 7]));
    assert_eq!(2,  chop(5, &[1, 3, 5, 7]));
    assert_eq!(3,  chop(7, &[1, 3, 5, 7]));
    assert_eq!(-1, chop(0, &[1, 3, 5, 7]));
    assert_eq!(-1, chop(2, &[1, 3, 5, 7]));
    assert_eq!(-1, chop(4, &[1, 3, 5, 7]));
    assert_eq!(-1, chop(6, &[1, 3, 5, 7]));
    assert_eq!(-1, chop(8, &[1, 3, 5, 7]));
}

fn chop_iterative(needle: i32, silo: &[i32])  -> isize {
    let mut start = 0;
    let mut end = silo.len();    
    while start < end {
        let current = (start + end) / 2;
        let element = silo[current];
        match needle.cmp(&element) {
            Ordering::Equal => return current as isize,
            Ordering::Less => end = current,
            Ordering::Greater => start = current + 1
        }
    }
    -1
}

fn chop_recursive_iter(needle: i32, silo: &[i32], start: usize, end: usize) -> isize {
    if start >= end {
        return -1;
    }
    let middle = (start + end) / 2;
    let element = silo[middle];
    match needle.cmp(&element) {
        Ordering::Less => chop_recursive_iter(needle, &silo, start, middle),
        Ordering::Greater => chop_recursive_iter(needle, &silo, middle + 1, end),
        Ordering::Equal => return middle as isize,
    }
}

fn chop_recursive(needle: i32, silo: &[i32]) -> isize {
    chop_recursive_iter(needle, &silo, 0, silo.len())
}

fn chop_functional_iter(needle: i32, silo: &[i32], offset: usize) -> isize {
    let start = 0;
    let end = silo.len();
    if start >= end {
        return -1;
    }
    
    let current = (start + end) / 2;
    match needle.cmp(&silo[current]) {
        Ordering::Less => chop_functional_iter(needle, &silo[..current], 0),
        Ordering::Greater => chop_functional_iter(
            needle, 
            &silo[current+1..], 
            offset + current+1
        ),
        Ordering::Equal => (current + offset) as isize,
    }    
}

fn chop_functional(needle: i32, silo: &[i32]) -> isize {
    chop_functional_iter(needle, silo, 0)
}

fn chop_functional_pure(needle: i32, silo: &[i32]) -> isize {
    let start = 0;
    let end = silo.len();
    if start >= end {
        return -1;
    }
    
    let middle_pos = (start + end) / 2;
    match needle.cmp(&silo[middle_pos]) {
        Ordering::Less => chop_functional_pure(needle, &silo[..middle_pos]),
        Ordering::Greater => {
            let pos = chop_functional_pure(needle, &silo[middle_pos+1..]);
            if pos == -1 { -1 } else { (middle_pos + 1) as isize + pos }
        },
        Ordering::Equal => middle_pos.try_into().unwrap(),
    }
}

fn chop_functional_clean(needle: i32, silo: &[i32]) -> isize {
    let end = silo.len();
    if end == 0 {
        -1
    } else {
        let middle_pos = end / 2;
        match needle.cmp(&silo[middle_pos]) {
            Ordering::Less => chop_functional_clean(needle, &silo[..middle_pos]),
            Ordering::Greater => {
                let pos = chop_functional_clean(needle, &silo[middle_pos+1..]);
                if pos == -1 { -1 } else { (middle_pos + 1) as isize + pos }
            },
            Ordering::Equal => middle_pos as isize,
        }
    }
}

// Might use tail recursion
fn chop_functional_tail(needle: i32, silo: &[i32]) -> isize {
    let end = silo.len();
    if end == 0 {
        return -1;
    }
    
    let middle_pos = end / 2;
    let middle_elem = silo[middle_pos];
    if needle == middle_elem {
        return middle_pos as isize;
    }
    if needle > middle_elem {
        let pos = chop_functional_tail(needle, &silo[middle_pos+1..]);
        return if pos == -1 { -1 } else { (middle_pos + 1) as isize + pos };
    }   
    chop_functional_tail(needle, &silo[..middle_pos])
}

// Using more suitable return value type
// Actually returning 'bool' will be better because we know the element already
#[allow(dead_code)]
fn chop_functional_element(needle: i32, silo: &[i32]) -> Option<i32> {
    let end = silo.len();
    if end == 0 {
        return None;
    }
    
    let middle_pos = end / 2;
    let middle_elem = silo[middle_pos];
    match needle.cmp(&middle_elem) {
        Ordering::Less => chop_functional_element(needle, &silo[..middle_pos]),
        Ordering::Greater => chop_functional_element(needle, &silo[middle_pos+1..]),
        Ordering::Equal => Some(middle_elem),
    }
}

#[test]
fn test_chops() {
    test_chop(&chop_iterative, "Iter");
    test_chop(&chop_recursive, "Recr");
    test_chop(&chop_functional, "Func");
    test_chop(&chop_functional_pure, "Pure");
    test_chop(&chop_functional_clean, "Clean");
    test_chop(&chop_functional_tail, "Tail");
}
