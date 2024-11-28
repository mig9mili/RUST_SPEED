fn main(){
    // Numbers
    let number : i8 = 2;
    let float_number : f64 = 3.14;
    let float_number_2 : f32 = 4.757;

    //Boolean 
    let is_active : bool = true;

    //Character
    let character : char = 'A';

    //string slice
    let text : &str = "hello mili"  ;// String slice - immutable, fixed
   // text.push_str(" world");  // This would ERROR - can't modify a slice


   // String object - growable, owned
    let mut string1= String::from("hello") ;// string object 
    string1.push_str(" world");

    println!("{}", string1);
    println!("{}" , text);

    // array 
    let array = [1,2,3,4,5,6];
 
     // vector //growable array
     let mut vector =  vec![2,3,4];
     vector.push(5);

     //tuple
     let person = ("yashas" , 30);
     println!("name :{} , age : {}" , person.0 , person.1);

    
}