struct A {
    a: i32
}

struct AA {
    aa: i64,
}

struct B(A, AA);

impl A {
    fn f(&self) {
        println!("{}", &self.a);
    }
}



fn main() {
    let a = A { a: 1 };
    let aa = AA { aa: 2};
    let b = B(a, aa);

    b.0.f();
    println!("{}", b.1.aa)
}
