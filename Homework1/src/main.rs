const FREEZINGTEMP:f64 = 32.0;
const SECRETNUM:i32 = 20;

fn farenheit_to_celsius(mut f:f64) -> f64
{
    	
f = (f - 32.0) * (5.0/9.0); 
println!("{}", f);
return f;
}

fn celsius_to_farenheit(mut c: f64) -> f64
{
    c = (c * 9.0/5.0) + 32.0;
    println! ("{}", c);
    return c;
}

fn assignment1()
{
     let mut firsttemp:f64 = 75.0;
 let mut counter = 0;

 farenheit_to_celsius(firsttemp);

 while counter < 5
 {
    firsttemp += 1.0;
    farenheit_to_celsius(firsttemp);
    counter+=1;
 }
}

fn is_even(n: i32) ->bool
{
    if n % 2 == 0
    {
        return true;
    }
    else
    {
        return false;
    }
}

fn assignment2()
{
    let my_array: [i32;10] = [2,7,5,6,2,9,1,3,4,10];
    let mut counter:i32 = 0;

    while counter < my_array.len().try_into().unwrap()
    {
        if is_even(my_array[counter as usize]) == true
        {
            println!("true");
        }
        if my_array[counter as usize] % 3 == 0 && my_array[counter as usize] % 5 == 0
        {
            println!("FizzBuzz")
        }
        if my_array[counter as usize] % 3 == 0
        {
            println!("Fizz");
        }
        if my_array[counter as usize] % 5 == 0
        {
            println!("Buzz");
        }
        else
        {
            println!("false");
        }

        counter += 1;

    }

}

fn check_guess( guess:i32, secret: i32) -> i32
{
    if guess == secret
    {
          return 0;
    }
    if guess < secret
    {
        return -1;
    }
    else
    {
        return 1;
    }

}
fn assignment3 ()
{
    let user_guesses: [i32;5] = [15,3,5,23,20];
    let mut counter = 0;
    let mut guessresult:i32;

    while counter < user_guesses.len()
    {   
        guessresult = check_guess(user_guesses[counter as usize], SECRETNUM);
        if guessresult == 1
        {
            println!("Guess {} is too high!",user_guesses[counter]);
            counter += 1;

        }
        else if guessresult == -1
        {
            println!("Guess {} is too low!", user_guesses[counter]);
            counter += 1;
        }
        else if guessresult == 0
        {
            println!("You guessed correctly!");
            counter += 1;
            break;
        }
        else{
            counter += 1;
        }
    }
     println!("It took {} guesses",counter);


}



fn main() {

    assignment1();
    assignment2();
    assignment3();
    
}
