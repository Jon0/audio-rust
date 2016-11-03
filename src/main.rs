use std::io;


fn printc(text: String, colour: u32) {
    println!("\x1b[1;{};1m{}", colour, text);
}


fn asknumber() -> u32 {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).expect("failed to read line");
    let number: u32 = match buf.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    return number;
}


fn sequence(start: u32, size: usize) {
    let mut grid = vec![0; size * size];
    let start: usize = start as usize;
    for i in 0..size*size {
        let number = start + i;
        grid[i] = number % 3;
    }
    for i in 0..size {
        for j in 0..size {
            print!("{}, ", grid[i * size + j])
        }
        println!("");
    }
}


fn main() {
    sequence(asknumber(), 8);
}
