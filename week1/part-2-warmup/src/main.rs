/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut v_: Vec<i32> = Vec::new();
    for i in v {
        v_.push(i + n);
    }
    return v_;
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    let len = v.len();
    let mut i = 0;
    while i < len {
        v[i] += n;
        i += 1;
    }
}

fn dedup(v: &mut Vec<i32>) {
    let mut my_set = HashSet::new();
    let mut v_: Vec<i32> = Vec::new();
    for i in v.iter() {
        if !my_set.contains(i) {
            my_set.insert(*i);
            v_.push(*i);
        } else {

        }
    }
    v.clear();
    for i in v_.iter() {
        v.push(*i);
        println!("{}", i)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
