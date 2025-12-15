mod generic_type_in_function {
    pub fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
}

mod generic_type_in_struct {
    pub struct Point<T> { x: T, y: T, }

    impl<T> Point<T> {
        pub fn new(x: T, y: T) -> Point<T> {
            Point { x, y }
        }

        pub fn x(&self) -> &T {
            &self.x
        }

        pub fn y(&self) -> &T {
            &self.y
        }
    }
}




fn main() {
    // ********** generic type in function ********** //
    // the function works for Vec<i32>
    let list = vec![34, 50, 25, 100, 65];
    let result = generic_type_in_function::largest(&list);
    print!("The largest number is: {}", result);

    // as well as for Vec<char>
    let list = vec!["a", "c", "&", "="];
    let result = generic_type_in_function::largest(&list);
    println!("\nThe largest char is: {}", result);


    // ********** generic type in struct ********** //
    // both integer
    let p1 = generic_type_in_struct::Point::new(5, 10);
    println!("Point p1: ({}, {})", p1.x(), p1.y());

    // both float
    let p2 = generic_type_in_struct::Point::new(1.0, 4.0);
    println!("Point p2: ({}, {})", p2.x(), p2.y());

    // different types will not work
    // let this_wont_work = generic_type_in_struct::Point::new(5, 4.0);
    // println!("Point p3: ({}, {})", this_wont_work.x(), this
}
