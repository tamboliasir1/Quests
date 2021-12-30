fn main() {
    let mut x=1;
    //infinite loop
    loop{
        x = x*2;
        if x>5000{
            break;
        }
        println!("value:{}",x);
    }

    //while loop
    let mut y =1;
    while y>5000 {
        y=y*2;
        println!("value:{}",x);
    }

    //for loop
    for x in 0..9{
        println!("value:{}",x);
    }
    for x in 0..=9{
        println!("value:{}",x);
    }

    let a = [1,2,3];
    for x in a{
        println!("value:{}",x)
    }

    //match /switch case
    let z =2;

    match z{
        1 => println!("value:{}",z),
        2 => println!("value:{}",z),
        _ =>println!("value:{}",z)
    }

    let b = true;
    let c = false;

    match(b,c){
        (true,true)=>println!("x and y are true"),
        (false,false)=>println!("x and y are false"),
        (true,false)=>println!("x is true and y is false"),
        (false,true)=>println!("x is false and y is true")
    }
}
