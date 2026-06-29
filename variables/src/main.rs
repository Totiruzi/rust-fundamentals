use std::io;

fn main() {
   let mut x = 5;
   println!("The value of x is: {x}");
   x = 6;
   println!("The value of x is: {x}");

   const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

   // Shadowing a variable
   // this would have been a function name fn variableShadowing(x) 
   {
      let x = x * 2;
      println!("the value of x in the inner scope is x:  {x}");
   }

   println!("The value of x in the Outer scope is x: {x}");


   // Arrays
   fn array_indexing() {
      // Invalid Array Index

      let a = [1, 2, 3, 4, 5];

      println!("Please enter an array index");

      let mut index = String::new();

      io::stdin()
         .read_line(&mut index)
         .expect("Failed to read line 👽");

         let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

      let element = a[index];

      println!("The value of the element at index {index} is: {element}!")
   }

   // The program "array_indexing" demonstrates what happens when trying to index an 
   // out of bound element. Entering an index between 0 & 4 won't break the program but
   // 5 and above 
   // Uncomment the code below to allow it run.
   // array_indexing();
}
