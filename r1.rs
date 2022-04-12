use std::collections::HashMap;


fn vecs(){
 let mut v: Vec<i32> = Vec::new();
 let _v1 = vec![1, 2, 3];
 //let mut v2 = Vec::new(); //should declare the type
 

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{}", v[1]);
    
   {
     let v = vec![1, 2, 3, 4];
      // do stuff with v
      for i in v{
          println!("{}", i);
      }
    } // <- v goes out of scope and is freed here
    
  //let v = vec![9,8,7,6,1, 2, 3, 4, 5];
  //let third: &i32 = &v[2];
  let mut third: &i32 = &v[2];
  //let mut f = third;
  //*f+=10;
  //println!("The third element is {}", f);
  
  match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
  }
    //let v = vec![1, 2, 3, 4, 5];

    //let _does_not_exist = &v[100];
    //let _does2 = v.get(100);
    
    //let mut v = vec![1, 2, 3, 4, 5];
    let mut first = &v[0];

    //v.push(6);

    println!("The first element is: {}", first);
   let mut v2 = vec![100, 32, 57];
    for i in &v2 {
        println!("{}", i);
    }
  let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
  enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}


#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn ll() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}",list);
    println!("==========");
}



fn main() {
    ll();
    vecs();
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
let a = ["Guz", "Pishka", "Ui"];
//for number in (1..4).rev() {
//let a = a.rev();
for i in a.iter().rev() {
println!("{}!", i);
}
println!("LIFTOFF!!!");

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut _scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
        

   //println!()
   /*dfdsfsd
   f...
   */
}