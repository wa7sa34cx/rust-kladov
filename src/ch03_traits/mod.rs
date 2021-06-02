pub fn run() {
    let x = 92u8;
    let y = match x {
        0..=128 => "small",
        129..=255 => "big",
    };
    println!("{}", y);
    println!();

    // ----

    let cat = Cat;
    cat.say();

    let dog = Dog;
    dog.say();
    say_twice(&dog);

    // ----
    println!();
    let x = 65;
    x.say();

    // ----
}

struct Cat;
impl Say for Cat {
    fn say(&self) {
        println!("meow!");
    }
}

struct Dog;
impl Say for Dog {
    fn say(&self) {
        println!("woof!");
    }
}

trait Say {
    fn say(&self);
}

fn say_twice<T: Say>(t: &T) {
    t.say();
    t.say();
}

impl Say for i32 {
    fn say(&self) {
        println!("hm... int-int?");
    }
}
