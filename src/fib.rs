pub fn fib_n(n: u32) -> u32 {

    // return early for base cases - this is needed because the a,b shuffling starts with 2
    if n == 0 {
        return 0
    }

    if n == 1 {
        return 1
    }

    let mut a: u32 = 0;
    let mut b: u32 = 1;

    // we've started at the 2nd Fibonacci number because 0 and 1 have passed at this point in the sequence
    // the 2nd number in the sequence (first in the loop) is 1 (calculated by 0 + 1)
    let mut i: u32 = 2;
    while i <= n {
        // swap is needed to maintain the a,b sequence
        let old_b = b;

        b = a + b;  // fib calculation done here

        a = old_b;  // swap done, a,b now ready for next iteration
        i += 1;
    }

    b
}
