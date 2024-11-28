fn main(){

    //immutable variable by default
    let name = "yashas";
     //  name = "mili " // error because name is immutable

    //mutable variable 
    let mut age = 30 ;
    age = 32; // no error because age is mutable
 
     // constant variable
    const  MAX_POINT: u32 = 100_000;
   

     // print the variable 
     println!("name : {} , age :{} , maximum point : {}" ,name , age , MAX_POINT);
}