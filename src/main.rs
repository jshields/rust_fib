mod fib;

fn main() {
    // NOTE: n is zero indexed so "0th" fib in the sequence is the first number on the series depending on how you look at it.
    // This seems to be normal based on other examples.
    // Answers checked using: https://www.calculatorsoup.com/calculators/discretemathematics/fibonacci-calculator.php

    // WARN: panic due to addition overflow with larger fib_to values.
    let fib_to_10: u32 = 10;
    let fib_result1: u32 = fib::fib_n(fib_to_10);

    // FIXME: format fib_to properly for 1st, 2nd, 3rd etc.
    println!("The Fibonacci sequence up to {fib_to_10} is {fib_result1}!");

    let fib_to_5: u32 = 5;
    let fib_result2: u32 = fib::fib_n(fib_to_5);
    println!("The Fibonacci sequence up to {fib_to_5} is {fib_result2}!");
}
