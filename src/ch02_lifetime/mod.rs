#![allow(dead_code)]

pub fn run() {
    // Copy 🤔
    let x = 1;
    let y = x;
    let z = x;
    assert_eq!(y, z);

    // ---------------------------------------------
    println!();
    
    // Life without linkd 😞
    let xs = vec![1, 2, 3];
    print_vec(xs);
    // print_vec(xs); //use of moved value: `xs`

    // ---------------------------------------------
    println!();

    // Life with linkd 😍
    let xv = vec![1, 2, 3];
    print_vec_by_link(&xv);
    print_vec_by_link(&xv);

    // ---------------------------------------------
    println!();

    // Lifetime
    let r: &i32;
    {
        let y = 2;
        r = &y; // `y` does not live long enough!!!
    }
    // println!("{}", *r);

    // ---------------------------------------------
    println!();

    // another example
    let x = 1;
    let r: &i32;
    {
        let y = 2;
        r = f(&x, &y);
    }
    println!("{}", *r);

    // ---------------------------------------------
    println!();

    let s = S {value: 92};
    let rs = &s;
    println!("{:?}", *rs);

    // ---------------------------------------------
    println!();

    // Don't work 💩
    // let w = Wrapper { value: Box::new(92) };
    // let r: &i32 = &*w.value;
    // // w.value = Box::new(65);
    // let w = Wrapper { value: Box::new(62) };
    // println!("({})", r);

    // ---------------------------------------------
    println!();

    // murate after get reference
    // can't be compliled 💩
    // let mut xs = vec![1, 2, 3];
    // let x = &xs[0]; // immutable borrow occurs here
    // xs.push(4); // mutable borrow occurs here
    // println!("{}", x);

    // ---------------------------------------------
    println!();

    let mut w = Wrapper { value: Box::new(96)};
    let r: &i32 = &*w.value;
    // fw(&mut w); // cannot borrow `w` as mutable because it is also borrowed as immutable
    println!("{}", r);

    // ---------------------------------------------
    println!();

    // block initialization
    // let omelet = {
    //     let eggs = get_eggs(&mut refrigerator, 3);
    //     let bacon = open_bacon(&mut refrigerator);
    //     fry(eggs, bacon);
    // };
    
    // ---------------------------------------------
    println!();

    // for
    for x in vec![1, 2, 3, 4, 5] {
        println!("x = {}", x);
    }

    println!();

    let xs = vec![1, 2, 3, 4, 5];
    for i in 0..xs.len() {
        let x = xs[i];
        println!("x = {}", x);
    }

    println!();

    for i in (0..=10).step_by(2) {
        print!("{}, ", i);
    }

    println!();

    // ---------------------------------------------
    println!();

    // `;` after let always needed! 
    let x = 666;
    let s = if x > 6 {
        "positive"
    } else {
        "negative"
    };
    println!("s = {}", s);

    // ---------------------------------------------
    println!();

    // Structures
    let p1 = Point { x: 2.0, y: 3.0 };
    let p2 = Point { x: 3.0, .. p1};
    assert_eq!(p2.x, p2.y);

    let p = Point::origin();
    assert_eq!(p.distance_from_origin(), 0.0);

    // ---------------------------------------------
    println!();

    let g = Goo(0.0, 1.0);
    assert_eq!(g.0, 0.0);

    // ---------------------------------------------
    println!();

    // let km = Kilometers(5.0);
    // assert_eq!(5.0, km); // Panic!

    // ---------------------------------------------
    println!();

    let d1: Distance<Kilometers> = Distance {
        amount: 500.0,
        metric: Kilometers,
    };

    let d2: Distance<Miles> = Distance {
        amount: 500.0,
        metric: Miles,
    };
    assert_eq!(d1.amount, d2.amount);

    // ---------------------------------------------
    println!();

    let v = vec![1, 2, 3];
    match v.get(5) {
        Some(value) => println!("{}", value),
        // None => panic!("out of bond access"),
        None => println!("out of bond access"),
    }
}

// print vector
fn print_vec(xs: Vec<i32>) {
    for x in xs {
        print!("{}, ", x);
    }
    println!();
}

// print vector by link
fn print_vec_by_link(xs: &Vec<i32>) {
    for x in xs {
        print!("{}, ", x);
    }
    println!();
}

// functioin with lifetimes
fn f<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    let z = x;
    z
}

#[derive(Debug)]
struct S {value: i32}

struct Wrapper {
    value: Box<i32>
}

fn fw(w: &mut Wrapper) {
    w.value = Box::new(62);
}

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn scale(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }

    fn origin() -> Point {
        Point {x: 0.0, y: 0.0 }
    }
}

// Taple Struct
struct Goo(f64, f64);

impl Goo {
    fn origin() -> Goo {
        Goo(0.0, 0.0)
    }

    fn dist(self, other: Goo) -> f64 {
        let Goo(x1, y1) = self;
        let Goo(x2, y2) = other;
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    }
}

// newtype
// struct Kilometers(f64);
// struct Miles(f64);

// Zero Sized Type
struct Tag;

// Type tags
struct Kilometers;
struct Miles;

struct Distance<T> {
    amount: f64,
    metric: T,
}

// Enumerations
enum Shape {
    Circle,
    Square,
}

struct Circle {
    center: Point,
    radius: f64,
}

struct Square {
    bottom_left: Point,
    top_right: Point,
}

impl Shape {
    fn circle(center: Point, radius: f64) -> Circle {
        Circle { center, radius }
    }

    fn area(shape: Shape) -> f64 {
        match shape {
            Shape::Circle  => {
                // let radius = Shape::Circle { radius, .. };
                // std::f64::consts::PI * radius * radius
                // unimplemented!()
                todo!()
            }
            Shape::Square => {
                unimplemented!()
            }
        }
    }
}