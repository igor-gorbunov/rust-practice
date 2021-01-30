fn main()
{
    println!("Fibonacci number for 48 = {}", fibonacci(48));
}

fn fibonacci(nm: u64) -> u64
{
    if nm == 0
    {
        return 0;
    }
    else if nm == 1
    {
        return 1;
    }
    else
    {
        return fibonacci(nm - 1) + fibonacci(nm - 2);
    }
}

