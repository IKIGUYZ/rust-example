use std::mem;


fn learn_variable(){
    // Variables

    // Default Immutable
    let name = "Ho Ba Hung";
    let company = "DigiMed";

    // Mutable
    let mut age:i16 = 24; // i32 default
    let mut weight = 165.5; // f64 default
    println!("{} from {} company", name, company);
    println!("Before: {} age {} weight", age, weight);
    age = 25;
    weight = 165.5;
    println!("After: {} age {} weight", age, weight);

    const USER_LIMIT:i32 = 6;
    // reads first salary
    println!("Default decimals is {}", USER_LIMIT);

    let example_string = String::from("example_string");
    print_literal(example_string.as_str());
}

fn print_literal(data:&str ){
    println!("displaying string literal {}",data);
}

fn operators(){
    let mut a = 2 + 3 * 4;
    println!("{}", a);
    a = a + 1;
    a -= 2; // a = a - 2;
    println!("Remainder of {} / {} = {}", a, 3, (a % 3));
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

}

struct Point {
    x: f64,
    y: f64
}
   
fn origin() -> Point {
    return Point { x: 0.1, y: 0.0 };
}

pub fn stack_and_heap() {
    let p1 = origin();
    let p2 = Box::new(origin());
    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));
    let p3 = *p2;
    println!("p3 = {}", p3.x);
}

fn while_and_loop() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if(x == 64) {continue;}
        println!("{}", x);
    }

    let mut y = 1;
    loop {
        y *=2;
        println!("{}", y);
        if(y == 1024){ break;}
    }
}

fn for_loop() {
    for x in 1..10 {
        if(x == 3) {continue;}
        if(x == 9) {break;}
        println!("{}", x);
    }
    for(i, item) in (20..30).enumerate() {
        println!("item {} in {} index", item, i);
    }
}

fn match_statement() {
    let country_code = 44;
    let country = match country_code {
        44 => "UK",
        46 => "VN",
        1..=1000 => "unknown",
        _ => "invalid"
    };
    println!("the country with code {} is {}", country_code, country);
}

enum State {
    Locked,
    Failed,
    Unlocked
}

fn combination_lock() {
}
    
struct Line {
    start: Point,
    end: Point
}

fn learn_struct() {
    let p1 = Point{x: 2.2, y: 3.1};
    let p2 = Point{x: 3.0, y: 4.0};
    let line = Line{start: p1, end: p2};
    println!("Line start at x = {} and end at x = {}", line.start.x, line.end.x);
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), //tuple
    CustomColor { red: u8, yellow: u8, black: u8 }, // struct
}

fn learn_enum() {
    // let c:Color = Color::Blue;
    // let c:Color = Color::RgbColor(255, 0, 255);
    let c:Color = Color::CustomColor{red: 1, yellow: 2, black: 254};


    match c
    {
        Color::Red =>
        {
            println!("Red");
        },
        Color::Green =>
        {
            println!("Green");
        },
        Color::Blue =>
        {
            println!("Blue");
        },
        Color::RgbColor(r, g, b) => 
        {
            println!("rgb({},{},{})", r, g, b);
        },
        Color::CustomColor{red:_, yellow:_, black: 255} => {
            println!("black");
        }
        _ => (
            println!{"not match!"}
        )
    }
}

// union {

// }

fn learn_unions() {
    println!("Learn unions");
}

fn learn_option_t()
{
    let x = 3.0;
    let y = 0.0;

    // Option => Some(v) | None
    let result = if y != 0.0 {Some(x/y)} else {None};
    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Cannot divide by zero")z
    }

    if let Some(z) = result {
        println!("result = {}", z);
    }
    if let None = result {
        println!("Cannot divide by zero check by if");
    }
}

fn learn_array()
{
    Nzz
}


fn main() {
    // learn_variable();
    // operators();
    // stack_and_heap();
    // while_and_loop();
    // for_loop();
    // match_statement();
    // learn_struct();
    // learn_enum();
    // learn_option_t();
}