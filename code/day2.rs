use std::io::{self, Write};

fn main() {
    // f_to_c();
    // fibonacci();
    twelve_days_of_christmas();
}

fn f_to_c() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let f: f64 = loop {
        print!("°F = ");
        stdout.flush().expect("Failed to flush stdout");

        let mut inp = String::new();

        stdin.read_line(&mut inp).expect("Failed to read line");

        break match inp.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };
    };

    println!("°C = {}", (f - 32.) * 5. / 9.);
}

fn fibonacci() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let n: i32 = loop {
        print!("n = ");
        stdout.flush().expect("Failed to flush stdout");

        let mut inp = String::new();

        stdin.read_line(&mut inp).expect("Failed to read line");

        break match inp.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };
    };

    if n == 0 || n == 1 {
        println!("{}", n);
    }

    let mut previous: u32 = 0;
    let mut current: u32 = 1;

    for _ in 2..=n {
        let calc = match previous.checked_add(current) {
            Some(v) => v,
            None => {
                println!("\nOverflow!");
                break;
            }
        };

        previous = current;
        current = calc;
        print!("{} ", calc);
    }

    println!();
}

fn twelve_days_of_christmas() {
    let lyrics = [
        "Twelve lords-a-leaping",
        "Eleven ladies dancing",
        "Ten pipers piping",
        "Nine drummers drumming",
        "Eight maids-a-milking!",
        "Seven swans-a-swimming!",
        "Six geese-a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
        "And a partridge in a pear tree\n",
    ];

    for i in 1..=12 {
        println!(
            "On the {} day of Christmas",
            match i {
                1 => "first",
                2 => "second",
                3 => "third",
                4 => "fourth",
                5 => "fifth",
                6 => "sixth",
                7 => "seventh",
                8 => "eighth",
                9 => "ninth",
                10 => "tenth",
                11 => "eleventh",
                12 => "twelfth",
                _ => unreachable!()
            }
        );
        println!("My true love gave to me");
        if i == 1 {
            println!("A partridge in a pear tree\n");
            continue;
        }

        let diff = 12 - i;

        for i in 0..i {
            println!("{}", lyrics[diff + i]);
        }
    }
}