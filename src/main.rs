// struct Tweet {
//     username: String,
//     content: String,
// }
// struct NewsArticle {
//     author: String,
//     content: String,
// }

// pub trait Summary {
//     fn summarize(&self) -> String {
//         format!("Read more..")
//     }
// }

// struct Driver {
//     name: String,
//     age: u8,
// }

// impl Driver {
//     fn create_driver(name: String, age: u8) -> Driver {
//         Driver { name, age }
//     }
// }

// pub trait DriverType {
//     fn carDriver(&self) -> String;
//     fn truckDriver(&self) -> String;
// }

// impl DriverType for Driver {
//     fn carDriver(&self) -> String {
//         format!("name: {}, age: {}, Liscence: LTV", self.name, self.age)
//     }

//     fn truckDriver(&self) -> String {
//         format!("name: {}, age: {}, Liscence: HTV", self.name, self.age)
//     }
// }

// fn main() {
//     let carD = Driver::create_driver("zain".to_string(), 26);
//     let truckD = Driver::create_driver("Noman".to_string(), 25);

//     println!("{}", carD.carDriver());
//     println!("{}", truckD.truckDriver());
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{} by {}", self.content, self.username)
//     }
// }

// impl Summary for NewsArticle {}
// fn main() {
//     let twt = Tweet {
//         username: "Zainbawa".to_string(),
//         content: "this is the tweet by ".to_string(),
//     };
//     let NA = NewsArticle {
//         author: "Zainbawa".to_string(),
//         content: "this is the article by ".to_string(),
//     };
//     println!("Tweet: {}", twt.summarize());
//     println!("News Article: {}", NA.summarize());
// }
// #[derive(Debug)]
// struct plane {
//     height: u32,
//     width: u32,
//     seats: i32,
// }

// impl plane {
//     fn seats(&self) -> i32 {
//         self.seats
//     }

//     fn small_plane(height: u32, width: u32, seats: i32) -> plane {
//         plane {
//             height,
//             width,
//             seats,
//         }
//     }
// }

// fn main() {
//     let p = plane::small_plane(40, 50, 60);

//     println!("plane details: {:#?}", p);

//     println!("plane seats: {}", p.seats());
// }

// #[derive(Debug)]
// struct Student {
//     name: String,
//     age: u8,
//     address: String,
// }

// impl Student {
//     pub fn create_student(name: String, age: u8, address: String) -> Student {
//         Student { name, age, address }
//     }
//     fn view_student(&self) -> String {
//         let info_student = format!(
//             "Name: {}\nAddress: {}\n , Age:{}",
//             self.name, self.address, self.age
//         );
//         info_student
//     }
// }

// fn main() {
//     let student1 = Student::create_student("Zain".to_string(), 25, "zainulabideenbawa".to_string());

//     println!("{:?}", student1);
//     println!("{}", student1.view_student());

//     let student2 = Student {
//         name: "zain".to_string(),
//         age: 25,
//         address: "zainulabideenbawa".to_string(),
//     };

//     println!("student2 : {:?}", student2)
// }

// #[derive(Debug)]
// struct Student {
//     name: String,
//     age: u8,
//     address: String,
// }

// impl Student {
//     pub fn create_student(name: String, age: u8, address: String) -> Student {
//         Student { name, age, address }
//     }
// }

// fn main() {
//     let student1 = Student::create_student("Zain".to_string(), 25, "zainulabideenbawa".to_string());

//     println!("{:?}", student1);
//     println!("{}", student1.info());

//     let student2 = Student {
//         name: "zain".to_string(),
//         age: 25,
//         address: "zainulabideenbawa".to_string(),
//     };

//     println!("student2 : {:?}", student2);
//     println!("second trait : {}", student2.only_first_name())
// }
// pub trait information {
//     fn info(&self) -> String;
//     fn only_first_name(&self) -> String;
// }

// impl information for Student {
//     fn info(&self) -> String {
//         let info_student = format!(
//             "Name: {}\nAddress: {}\n , Age:{}",
//             self.name, self.address, self.age
//         );
//         info_student
//     }

//     fn only_first_name(&self) -> String {
//         format!("{}", self.name)
//     }
// }
// struct point<T> {
//     x: T,
//     y: T,
// }

// impl<T> point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl point<f32> {
//     fn distance(&self) -> f32 {
//         self.x.po
//     }
// }

// fn main() {
//     let p = point { x: 5, y: 6 };
// }

fn Largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = Largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = Largest(&char_list);
    println!("The largest char is {}", result);
}

// use std::fmt::Display;

// struct Tweet {
//     username: String,
//     content: String,
// }

// struct NewsArticle {
//     author: String,
//     content: String,
// }

// pub trait Summary {
//     fn summarize(&self) -> String {
//         format!(
//             "this is default Implementation. {}",
//             self.summarize_author()
//         )
//     }

//     fn summarize_author(&self) -> String;
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("@{} Tweeted{}", self.username, self.content)
//     }
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.username)
//     }
// }

// impl Summary for NewsArticle {
//     // fn summarize(&self) -> String {
//     //     format!("@{} Tweeted{}", self.author, self.content)
//     // }

//     fn summarize_author(&self) -> String {
//         format!("@{}", self.author)
//     }
// }

// fn notify(item: impl Summary, item1: impl Summary) {
//     //implementation trait syntax
//     println!("{},\n {}", item.summarize(), item1.summarize())
// }

// fn notify1<T: Summary, U: Summary>(item: T, item1: U) {
//     println!("{}\n{}", item.summarize(), item1.summarize());
// }

// fn return_summary() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//     }
// }

// fn main() {
//     let tweet1 = Tweet {
//         username: "ZainBawa ".to_string(),
//         content: "Its Cold Outside".to_string(),
//     };

//     let news = NewsArticle {
//         author: "ZainBawa".to_string(),
//         content: "12 Rules of Life".to_string(),
//     };

//     // notify(tweet1, news);
//     notify1(tweet1, news);

//     println!("{:?}", return_summary().summarize());

//     // println!("{}", tweet1.summarize());
//     // println!("{}", news.summarize());
// }
