use vector_array::vec::VecArray;
use vector_array::vec_arr;

fn main() {
    let mut vec: VecArray<String, 4> = vec_arr![
        "Hello".to_string(),
        "Hello".to_string(),
        "Hello".to_string(),
        "Hello".to_string()
    ];
    vec.push("Hello".to_owned()).unwrap();
    // dbg!(&vec[0]);
}
