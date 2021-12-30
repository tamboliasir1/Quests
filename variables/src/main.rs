fn main() {
    let x= 1;

    println!("value of x is {}",x);

    //x=2 will give error because we have not used mut keyword so it is immutable

    let mut y =1;
    println!("value of y is {}",y);

    y=2;
    println!("reassigned value of y is {}",y);
}
