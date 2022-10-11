fn main() {
    let _counter = 10; // prefixing with an underscore makes its existance without being used ignored.
                       // counter = 100; // asignment to immutable variable
    let mut mutable_counter = 100;
    println!("Counter value is  {}", mutable_counter);
    mutable_counter = 1000;
    println!("Counter value is  {}", mutable_counter);
    // wrapping the numbers
    // let _small_counter: u8 = 260; // this will be wrapped around. This will become 5 on runtime.

    // constants
    const PI: f64 = 3.14; // type annotation is required and it need to initialised statically.
                          // Its value can not be determined at runtime.
                          // moreover , SHadowing is not allowed.

    println!("Value of  this  constant is  :{} ", PI)
    // booleans :

    // scalar data types :
    // intergers,floats , booleans and characters.
    // vector data types.
    //
}
