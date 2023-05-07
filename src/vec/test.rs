use crate::error::ArrTooSmall;
use crate::vec::VecArray;
use crate::vec_arr;

// All test rely on that .push and ::new work

#[test]
fn pop() {
    let mut vec: VecArray<_, 10> = VecArray::new();
    assert_eq!(vec.push(9), Ok(()));
    assert_eq!(vec.pop(), Some(9));
    assert_eq!(vec.pop(), None);
}

#[test]
fn push_err() {
    let mut vec: VecArray<u32, 1> = VecArray::new();
    assert_eq!(vec.push(9), Ok(()));
    assert_eq!(vec.push(2), Err(ArrTooSmall));
    assert_eq!(vec, vec_arr![9]);
}

#[test]
fn remove() {
    let mut vec: VecArray<_, 10> = vec_arr![0, 1, 2, 3, 4, 5];
    vec.remove(1);
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&2));
}

#[test]
fn insert() {
    let mut vec: VecArray<_, 10> = vec_arr![0, 1, 2, 3, 4, 5];
    vec.insert(2, 1);
    assert_eq!(vec, vec_arr![0, 1, 1, 2, 3, 4, 5]);
}

#[test]
fn swap() {
    let mut vec: VecArray<_, 10> = vec_arr![0, 1, 2, 3, 4, 5];
    vec.swap(2, 5);
    assert_eq!(vec, vec_arr![0, 1, 5, 3, 4, 2]);
}

#[test]
fn retain() {
    let mut vec: VecArray<_, 10> = vec_arr![0, 1, 2, 3, 4, 5];
    vec.retain(|&x| x % 2 == 0);
    assert_eq!(vec, vec_arr![0, 2, 4]);
}

#[test]
fn get() {
    let vec: VecArray<_, 10> = vec_arr![0, 1, 2, 3, 4, 5];
    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&1));
    assert_eq!(vec.get(2), Some(&2));
    assert_eq!(vec.get(3), Some(&3));
    assert_eq!(vec.get(4), Some(&4));
    assert_eq!(vec.get(5), Some(&5));
    assert_eq!(vec.get(6), None);
}

#[test]
fn from_vec_array() {
    let vec_arr: VecArray<_, 10> = vec_arr![0, 1, 2, 3, 4, 5];
    assert_eq!(Vec::from(vec_arr), vec![0, 1, 2, 3, 4, 5])
}

#[test]
fn index() {
    let vec: VecArray<_, 10> = vec_arr![0, 1, 2, 3, 4, 5];
    assert_eq!(vec[0], 0);
    assert_eq!(vec[1], 1);
    assert_eq!(vec[2], 2);
    assert_eq!(vec[3], 3);
    assert_eq!(vec[4], 4);
    assert_eq!(vec[5], 5);
}

#[test]
#[should_panic]
fn index_panic() {
    let vec: VecArray<_, 10> = vec_arr![0, 1, 2, 3, 4, 5];
    vec[6];
}

#[test]
fn index_mut() {
    let mut vec: VecArray<_, 10> = vec_arr![0, 1, 2, 3, 4, 5];
    vec[2] += 1;
    assert_eq!(vec, vec_arr![0, 1, 3, 3, 4, 5]);
}

#[test]
#[should_panic]
fn index_mut_panic() {
    let mut vec: VecArray<_, 10> = vec_arr![0, 1, 2, 3, 4, 5];
    vec[6] = 0;
}

#[test]
fn from_vec() {
    let vec_arr: VecArray<_, 10> = VecArray::from(vec![0, 1, 2, 3, 4, 5]);
    assert_eq!(vec_arr, vec_arr![0, 1, 2, 3, 4, 5])
}

#[test]
fn truncate() {
    let mut vec: VecArray<_, 10> = vec_arr![0, 1, 2, 3, 4, 5];
    vec.truncate(3);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec, vec_arr![0, 1, 2]);

    let mut vec: VecArray<_, 10> = vec_arr![0, 1, 2, 3, 4, 5];
    vec.truncate(9);
    assert_eq!(vec.len(), 6);
    assert_eq!(vec, vec_arr![0, 1, 2, 3, 4, 5]);
}
