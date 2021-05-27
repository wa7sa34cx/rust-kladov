pub fn run() {
    // Copy 🤔
    let x = 1;
    let y = x;
    let z = x;
    assert_eq!(y, z);

    // ---
    println!();
    
    // Life without linkd 😞
    let xs = vec![1, 2, 3];
    print_vec(xs);
    print_vec(xs);



}

fn print_vec(xs: Vec<i32>) {
    for x in xs {
        println!("{}", x);
    }
}