use std::io;

fn tutorials() {
    let mut x: i32 = 10; // By default the variables are immutable
    println!("{}", x);

    x = 20;
    println!("{}", x);
    let mut y: f32 = 64.0;
    println!("{}", y);

    let mut value = 0b1111_0101u8;
    println!("{}", value);
    println!("{:08b}", value);

    value = !value;
    println!("{:08b}", value);

    value = value << 4;
    println!("{:08b}", value);

    value = value >> 2;
    println!("{:08b}", value);

    let a = false;
    println!("{}", a);

    let finger = '\u{261D}';
    println!("{}", finger);

    let mut letters = ['a', 'b', 'c', 'd'];
    println!("{}", letters[0]);
    letters[3] = 'D';
    println!("{}", letters[3]);
    let numbers: [i32; 5];
    numbers = [0; 5];
    println!("{}", numbers[4]);

    let index: usize = numbers.len();
    println!("{}", index);
    println!("{}", numbers[index - 1]);

    let parking_lot = [[1, 2, 3], [4, 5, 6]];
    let number = parking_lot[0][1];
    println!("{}", number);

    let mut tu = (1, 'C', 3.4);
    println!("{}", tu.0);

    tu.0 += 23;
    println!("{}", tu.0);

    say_hello();

    let xs = squre(10);
    println!("{}", xs);

    x = 3;

    y = 5.0;
    if x == 3 {
        println!("{}", x)
    }

    if x > 0 && y < 10.0 {
        println!("{}", y)
    }

    let mut count = 1;
    let re = loop {
        count += 1;
        println!("{}", count);

        if count == 5 {
            break count * 10;
        }
    };

    println!("{}", re);

    let hello = ['h', 'e', 'l', 'l', 'o'];

    for item in hello.iter() {
        println!("{}", item);
    }

    for (item, _charter) in hello.iter().enumerate() {
        println!("{} {}", item, _charter);
    }

    let planet = "Earth";
    println!("planet is  {}", planet);

    let planet = "Mars"; // shadow variable
    println!("planet is  {}", planet);

    let message = String::from("Earth");
    println!("{}", message);

    let outer_planet: String;

    {
        let mut inner_planet = String::from("Mercury");
        outer_planet = inner_planet.clone();
        inner_planet.clear();

        println!("inner planet is {}", inner_planet);
    }

    println!("outer planet is {}", outer_planet);

    let mut rocket_fuel = String::from("RP-1");
    process_fuel(&mut rocket_fuel);
    println!("{}", rocket_fuel);
}

fn process_fuel(propallant: &mut String) {
    println!("{}", propallant);
    propallant.push_str(" is changed");
}

fn say_hello() {
    println!("Hello");
}

fn squre(x: i32) -> i32 {
    return x * x;
}

fn main() {
    let mut buffer = String::new();
    println!("Enter a message: ");
    io::stdin().read_line(&mut buffer);
    println!("buffer is  {}", buffer);

    let number: i32 = buffer.trim().parse().unwrap();
    println!("number + 1 is {}", number + 1);
}
