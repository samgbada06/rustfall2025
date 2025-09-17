fn add (x:i32,y:i32){
    println!("{}",x+y)
}

fn fahrenheit_to_celsius(f: f64) -> f64
{
    let cel = (f-32.0) * (0.5555556);
    return cel
}

fn celsius_to_fahrenheit(c: f64) -> f64
{
    let cel = (f * 0.5555556) + 32.0;
    return cel
}

fn main() {
    
   println!("{}",fahrenheit_to_celsius())
}
