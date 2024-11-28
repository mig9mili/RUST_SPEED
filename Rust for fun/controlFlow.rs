fn main(){

    //if -else
    let number  = 3;
    if number < 5{
        println!("less than 5")
    }
    else{
        println!("5 or greater")
    }

    // match
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        _=> println!("somerthing else"),
    }
}