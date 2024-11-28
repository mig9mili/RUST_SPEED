fn greet(name: &str){
    println!(" hello {}", name);

}

fn add(a:i32 , b:i32) -> i32{
    a+b // no semicolon means retun this value 
}

// multiple return values
fn get_number() -> (i32 , i32){

(42 , 7)

}

fn main(){
    greet("yashas");
    let sum =  add(4,5);
    let (x,y) = get_number();

    println!("sum is {} numbert are : {} {}" , sum , x, y);

}