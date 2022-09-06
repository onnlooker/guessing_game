use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn celsius_fahrenheit_convert() {
    let mut degrees = String::new();

    let degrees: f32 = loop {
        println!("Please input degrees");

        io::stdin()
            .read_line(&mut degrees)
            .expect("Failed to read line");

        match degrees.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please type a number!");
                degrees.clear();
                continue;
            }
        };
    };

    let mut flag = String::new();

    let flag = loop {
        println!("Please input celsius(c)/fahrenheit(f)");

        io::stdin()
            .read_line(&mut flag)
            .expect("Failed to read line");

        match flag.trim() {
            // trim() 控制台输入后回车确认 flag 会包含回车
            "c" | "f" => break flag.trim(),
            _ => {
                flag.clear();
                continue;
            }
        }
    };

    println!("You input: {}{}", flag, degrees);

    // fahrenheit = 1.8 * celsius + 32;
    let (flag, degrees) = if flag == "f" {
        ("c", (degrees - 32.0) / 1.8)
    } else {
        ("f", 1.8 * degrees + 32.0)
    };

    println!("convert result: {}{}", flag, degrees)
}

fn fibonacci(n: i32) -> i32 {
    if n == 1 || n == 2 {
        1
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

fn fibonacci_helper() {
    let mut n = String::new();

    println!("Please input a number");

    let n: i32 = loop {
        io::stdin().read_line(&mut n).expect("Failed to read line");

        match n.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };
    };

    println!("n: {} fibonacci: {}", n, fibonacci(n))
}

fn print_the_twelve_days_of_christmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let list = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five gold rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtledoves",
        "partridge in a pear tree",
    ];

    for (index, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas, my true love sent to me", day);

        let mut j = index;
        while j > 0 {
            println!("{}", list[list.len() - 1 - j]);
            j -= 1;
        }

        println!(
            "{}{}",
            if index != 0 { "And a " } else { "A " },
            list[list.len() - 1]
        )
    }
}

fn main() {
    // guessing_game()
    // celsius_fahrenheit_convert();
    // fibonacci_helper();
    print_the_twelve_days_of_christmas();
}
