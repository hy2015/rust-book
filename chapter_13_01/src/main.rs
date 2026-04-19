use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1; 

    add_one_v1(5);
    add_one_v2(5);
    add_one_v3(5);
    add_one_v4(5);

    let example_closure = |x| x;
    let s = example_closure(String::from("Hello"));
    // let n = example_closure(5); // This line would cause a compile-time

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list2);
    let mut borrows_mutably = || list2.push(7);
    borrows_mutably();
    println!("After calling closure: {:?}", list2);

    
    let list3 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list3);
    // The closure borrows list3, and the thread may outlive the main thread, so we need to move the ownership of list3 into the closure.
    thread::spawn(move || println!("From thread: {:?}", list3))  
        .join()
        .unwrap();

    let mut list4 = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list4.sort_by_key(|r| r.width);
    println!("Sorted by width: {:#?}", list4);

//     let mut list5 = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12 },
//     ];
//     let mut sort_operations = vec![];
//     let value = String::from("closure called");

//     list5.sort_by_key(|r| {
//         sort_operations.push(value);
//         r.width
//     });
//     println!("Sorted by width: {:#?}", list5);


    let mut list6 = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    let mut num_sort_operations = 0;
    list6.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("Sorted by width: {:#?}, sorted {} times", list6, num_sort_operations);
}


fn add_one_v1(x: u32) -> u32 {
    x + 1
}