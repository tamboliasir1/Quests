fn main() {
    let x:i32 = 1; //signed 32 bit integer
    println!("Value if x is {}",x);

    //type inerence = auomatically detects type of variable
    let y = 2;
    println!("Value if y is {}",y);

    //y= "hello world"; this will give error because we have assigned integer hence we cannot assign string. 
    //It is called static types

    //unsigned 8 bit integer that means integer upto 256 can be used
    let z:u8 = 200;
    println!("Value if z is {}",z);

    //float either f32 or f64 can be used. f64 is default
    let a:f64 = 3.14;
    println!("Value if a is {}",a);

    //boolean
    let b:bool = true;
    println!("Value if b is {}",b);

    //composite types
    //tuples = set of diffrent types of values
    let c= (5,"hello world",true);
    println!("First two values are {} {}",c.0,c.1);

    //second option for reading
    //destructuring
    let (val_one,val_two,_) = c;
    println!("val one and val two are {} {}",val_one,val_two);

    //Arrays = set of similar types of values
    let d = [0,1,2];
    println!("arrays:{}",d[0]);

    //for same value multiple time
    //let e:[i32;10] = [0;10]; //for decalring type
    //Add mut keyword if you want to change array elements
    let e = [0;10];
    println!("arrays:{:?}",e) //special format for printing entire array



}
