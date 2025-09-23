let freezingtemp:f64 = 32;

fn farenheit_to_celsius(f:f64) -> f64
{
    	
return (f − 32) × (5/9); 


}

fn celsius_to_farenheit(c: f64) -> f64
{
    return (c × 9/5) + 32;
}

fn main() {
 let firsttemp:i32 mut = 75;
 let mut counter = 5;

 farenheit_to_celsius(firsttemp);

 while counter < 5
 {
    firsttemp += 1;
    farenheit_to_celsius(firsttemp);
    counter+=1;
 }

    
}
