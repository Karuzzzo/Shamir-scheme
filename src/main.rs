use std::env;
use std::io;
use std::io::Write;
extern crate bigint;
use bigint::uint::U256;
//this is needed for calculating and containing powers, before modulation
use bigint::uint::U512;
//our X coordinate doesnt need to be big, it lies in range 1..100, 
//and also represent number of key
struct Point{       
    x:u32,
    y:U256,
}
/*                      WARNING
This code doesn`t handle most of the errors that could happen,
so all input data must be correct. */
fn main(){
    //gathering input data from launch
    let args: Vec<String> = env::args().collect();
    io::stdout().flush().expect("error on flush");

    //selecting mode of running
    if select_mode(&args[2]) == 1{
        println!("you selected split mode!");

        split();
    } else if  select_mode(&args[2]) == 2{
        println!("you selected recover mode!");

        recover();
    } else {
        println!("you typed {}, not {} or {}", &args[2] as &str, "split", "recover");

        panic!("wrong command input!");
    }
}
fn select_mode(input: &str) -> u8{
    match input {
        "split" => 1,
        "recover" => 2,
        _ => 3,
    }
}
//down here all functions required for splitting secret key
fn split(){
    let (secret_key_hex, pub_keys, to_restore) = read_split_data();
    let secret_key :U256;

    secret_key =decode_from_hexstr_to_u256(secret_key_hex);

    let koeffs :Vec<U256> = generate_koeffs(to_restore);
    println!("generated polynom with power of {}", koeffs.len());
    io::stdout().flush().expect("error on flush");

    let mut output_number :U512;
    let mut counter :u32 = 1;
    while counter <= pub_keys {
        output_number = U512::from(secret_key.clone());
        let mut power :u32 =1;
        for k in koeffs.to_owned(){
            //this is the operation, i imported U512 for. I hope all operations down here are correct.
            let a :U512 = U512::from(k) * 
            (U512::from(counter).pow(U512::from(power)));

            output_number = output_number + a;
            power = power+1;
        }
        //forming point structure to send it to output
        let out: Point = Point {x: counter, y:get_mod(output_number)};

        split_output(out);
        counter+=1;
    }
}

//this is big function down here, it gets all data for split and returns it
fn read_split_data()-> (String, u32, u32){
    let secret_key :String;

    print!("Type in secret key: ");
    io::stdout().flush().expect("error on flush");
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            input.pop();    //removing EOF or idk what it is

            println!("{} bytes read, decoding..", n-1);
            secret_key = input.to_owned();
        }
        Err(error) => panic!("wrong input, error: {}", error) 
    }
    
    //total number of keys in stdout
    let pub_keys: u32;
    //number of keys to restore back private key
    let to_restore: u32;

    print!("Type in number of keys, and number of them, needed to restore private key: ");
    io::stdout().flush().expect("error on flush");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {            
            let (_pub_keys, _to_restore) = parse_numbers(&mut input);
            pub_keys = _pub_keys;
            to_restore = _to_restore;
        }
        Err(error) => panic!("wrong input, error: {}", error) 
    }

    (secret_key, pub_keys, to_restore)
}
fn parse_numbers(input:&mut String)-> (u32, u32){
    input.pop();    //removing EOF or idk what it is
    let vec:Vec<&str> = input.split(' ').collect();

    (vec[0].parse::<u32>().unwrap(), vec[1].parse::<u32>().unwrap())
}
//create random koeffs for polynom
fn generate_koeffs(mut to_restore: u32) -> Vec<U256>{
    let mut koeffs: Vec<U256> = Vec::new(); //vector contains all polynomial koeffs 

    while to_restore > 1 {
        let mut random_koeff :String = rand::random::<u128>().to_string(); //we create u256 string, concatenating 2 u128
        random_koeff.push_str(&rand::random::<u128>().to_string());
        random_koeff.pop();         //cutting off EOF

        let output :U256 = U256::from_dec_str(&random_koeff).unwrap();     //im pretty sure there could be no errors
        //println!("Koeff {} is {}", to_restore, output);
        koeffs.push(output);
        to_restore -= 1;
    }

    koeffs
}
fn split_output(out: Point){
    let hex_y = encode_from_u256_to_hexstr(out.y);

    println!("{}k{}",out.x, hex_y);
}

//this is part with all recovering functions
fn recover(){
    let str_vect = read_recover_data();
    let point_vect :Vec<Point> = parse_pub_keys(str_vect);
    let secret_key :U256 = interpolate(&point_vect);
    println!("your key is {:x}", secret_key);
}
fn read_recover_data()-> Vec<String>{
    let mut a:Vec<String> = Vec::new();

    println!("To restore private key, type in keys you have, one for line. When you typed all keys, press enter: ");
    io::stdout().flush().expect("error on flush");

    a.push("ToRemove".to_string());     //required kostyl, or there will be panic, vector is empty
    while a[a.len()-1].len() > 1 as usize{
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 1 {break}           //this happens when we press enter
                input.pop();                //removing EOF or idk what it is
                a.push(input.to_owned());
            }
            Err(error) => panic!("wrong input, error: {}", error) 
        }
    }
    a.swap_remove(0);   //removing element we used to enter cycle
    a
}
//we are taking strings from output, and create vector of points from it
fn parse_pub_keys(str_vect: Vec<String>)-> Vec<Point>{
    let mut out :Vec<Point> = Vec::new();
    for s in str_vect{
        let vec :Vec<&str> = s.split('k').collect();

        let _x = vec[0].parse::<u32>().unwrap();
        let _y = decode_from_hexstr_to_u256(vec[1].to_owned()); 

        let p:Point = Point {x: _x, y: _y};
        out.push(p);
    }
    out
}
fn interpolate(points: &Vec<Point>) -> U256{
    let mut secret :U512 = U512::zero();

    for p in points{
        let (num , den, neg) = multiply_lagrange(&points, p.x); //multiplying from Lagrange interpolation
        let x: U256 = U256::from(get_mod((U512::from(p.y) * num) / den));   //sum from Lagrange interpolation
        let a :U256 = if neg {get_opposite(x)} else {x};                    //cropping possibe negative koeffs

        secret = secret + U512::from(a);
    }
    get_mod(secret)
}
//from here we get num and den. We cant divide them here, 
//cause we will lose data if result of division is not integer 
//also, to avoid negative numbers, we must send flag if result is negative,
//and take opposite value in finite field later
fn multiply_lagrange(points: &Vec<Point>, current_number:u32)->(U512, U512, bool){
    let mut numerator :i32 = 1;
    let mut denominator :i32 = 1;

    for p in points{
        if p.x == current_number {continue;}
        numerator = numerator * (0 as i32 - p.x as i32);
        denominator = denominator * (current_number as i32 - p.x as i32);
    }

    let neg: bool = 
    if (denominator < 0) || (numerator < 0){        //if num or den negative, we setting the flag
        if (denominator < 0) && (numerator < 0) {   //if both negative, summary its positive, flag false
            denominator = denominator.abs();
            numerator = numerator.abs();
            false
        } else {      
            denominator = denominator.abs();
            numerator = numerator.abs();
            true
        }
    } else {false};

    (U512::from(numerator), U512::from(denominator), neg)
}

//down here is 4 functions, I used in all code, not in exact mode.
//there is no error control to simplify code, so all input data MUST be correct
fn decode_from_hexstr_to_u256(key_hex:String)->U256{
    let mut byte_vec: Vec<u8> = hex::decode(key_hex).unwrap();
    byte_vec.reverse();

    let mut output: [u8; 32] = [0; 32];
    let mut i:usize = 0;
    
    for a in byte_vec{
        output[31 - i] = a;
        i+=1;
    }

    U256::from(output)
}

fn encode_from_u256_to_hexstr(big_number:U256)-> String{
    let mut v :Vec<u8> = Vec::new();

    for i in 0 as usize..32 as usize {
        let b = big_number.byte(31-i);
        v.push(b);
    }
    hex::encode(v)
}

fn get_mod(input :U512)-> U256{
    //setting finite field 
    let s :String = "fffffffff44e0431043b044e043f04380442044c043f04380432043e044505fb".to_string();
    let modulo :U256 = decode_from_hexstr_to_u256(s);
    //returning cropped input
    let out = U256::from(input % U512::from(modulo));
    out
}

fn get_opposite(input :U256)-> U256{
    //setting finite field 
    let s :String = "fffffffff44e0431043b044e043f04380442044c043f04380432043e044505fb".to_string();
    let modulo :U256 = decode_from_hexstr_to_u256(s);
    //returning cropped input
    U256::from(modulo - input)
}
