fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(2);
    v.push(4);

    println!("{}", v[0]);

    let mut v2 = vec![2, 4, 6];
    println!("{}", v2[1]);

    let mut hashmap = HashMap::<u8, bool> = HashMap::new();
    hashmap.insert(5, true);
    hashmap.insert(6, false);

    let have_five = hashmap.remove(&5).unwrap();
}
