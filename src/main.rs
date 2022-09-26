use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 맞춰보자!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let random_char = rand::random::<char>();
    
    loop {
        println!("정답이라고 생각하는 숫자를 입력하세요. {}", random_char);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("입력한 값을 읽지 못했습니다. 뭘 쓴거야 이 멍청아!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("입력한 값: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 작습니다!"),
            Ordering::Greater => println!("입력한 숫자가 큽니다!"),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
    
    }
    
    
}
