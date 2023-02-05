use rand::Rng;

fn main() {
    println!("猜数游戏！");
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("神秘数字是：{}", secret_number);
    loop {
        println!("猜测一个数");
        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess).expect("无法读取行");

        println!("猜测的数是：{}", guess);

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number)
        {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("Tou win!");
                break;
            }
        }
    }
}
