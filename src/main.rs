#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

use std::ops::Add;

fn say_hello() {
    println!("Hello");
}

fn get_2(x: i32) -> (i32, i32) {
    // println!("{} + {} = {}", x, y, x+y);
    return (x + 1, x + 2);
}

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn main() {
    /*
    println!("What is your name? ");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input.");
    println!("Hello, {}! {}!", name.trim_end(), greeting);

    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number.");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

    // Unsigned integer(u), Signed(s)

    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max u128 : {}", u128::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);

    // let is_true = true;
    let num_1: f32 = 1.11111111111111111;
    println!("f32 : {}", num_1 + 0.11111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.1111111111111);

    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    let random_num: i32 = rand::thread_rng().gen_range((1..101));
    println!("Random : {}", random_num);

    loop {
        println!("Enter your birthday: ");
        let mut age = String::new();
        io::stdin()
            .read_line(&mut age)
            .expect("Didn't Receive Input.");
        let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number.");
        if (age >= 1) && (age <= 18) {
            println!("Important Birthday");
        } else if (age == 21) || (age == 50) {
            println!("Important Birthday");
        } else if age >= 65 {
            println!("Important Birthday");
        } else {
            println!("Not an Important Birthday");
        }
    } */
    /*
        let mut my_age: i32 = 47;
        let can_vote = if my_age >= 18 { true } else { false };
        println!("Can Vote: {}", can_vote);


        let age2: i32 = 8;
        match age2 {
            1..=18 => println!("Important Birthday"),
            21 | 50 => println!("Important Birthday"),
            65..=i32::MAX => println!("Important Birthday"),
            _ => println!("Not an Important Birthday"),
        };


    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    };

    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("1st: {}", arr_1[0]);
    println!("Length: {}", arr_1.len());

    let mut loop_idx = 0;

    loop {
        if arr_1[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_1[loop_idx] == 9 {
            break;
        }
        println!("Val : {}", arr_1[loop_idx]);
        loop_idx += 1;
    }*/
    /*
            let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
            let mut loop_idx = 0;
            while loop_idx < arr_2.len() {
                println!("Arr: {}", arr_2[loop_idx]);
                loop_idx += 1;
            }

            for val in arr_2.iter() {
                println!("Val : {}", val);
            }

            let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
            println!("Name : {}", my_tuple.1);
            let (v1, v2, v3) = my_tuple;
            println!("Age: {}", v1);


        let mut st1 = String::new();
        st1.push('A');
        st1.push_str(" word");
        for word in st1.split_whitespace() {
            println!("{}", word);
        }
        let st2 = st1.replace("A", "Another");
        println!("{}", st2);

        let st3 = String::from("x r t b h k k a m c");
        let mut v1: Vec<char> = st3.chars().collect();
        v1.sort();
        v1.dedup();
        for char in v1 {
            println!("{}", char);
        }
        let st4: &str = "Random string";
        let mut st5: String = st4.to_string();
        println!("{}", st5);
        let st6 = &st5[0..6];
        println!("String length : {}", st6.len());
        st5.clear();
        let st6 = String::from("Just some");
        let st7 = String::from("Just some");
        let st8 = st6 + &st7;
        for char in st8.bytes() {
            println!("{}", char);
        }

        enum Day {
            Monday,
            Tuesday,
            Wednesday,
            Thursday,
            Friday,
            Saturday,
            Sunday,
        }

        impl Day {
            fn is_weekend(&self) -> bool {
                match self {
                    Day::Saturday | Day::Sunday => true,
                    _ => false,
                }
            }
        }
        let today: Day = Day::Monday;
        match today {
            Day::Monday => println!("Everyone hates Monday"),
            Day::Tuesday => println!("Donut day"),
            Day::Wednesday => println!("Hump day"),
            Day::Thursday => println!("Pay day"),
            Day::Friday => println!("Almost Weekend"),
            Day::Saturday => println!("Weekend"),
            Day::Sunday => println!("Weekend"),
        }

        println!("Is today the weekend {}", today.is_weekend());

    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("1st : {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("Mo 2nd value"),
    }

    for i in &mut vec2 {
        *i *= 2;
    }

    for i in &vec2 {
        println!("{}", i);
    }

    println!("Vec Length {}", vec2.len());
    println!("Pop :{:?}", vec2.pop());

    say_hello();

    println!("{}", get_sum(5, 4));
    let (val_1, val_2) = get_2(3);
    println!("Nums : {} {}", val_1, val_2);*/

    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.2 + 4.6 = {}", get_sum_gen(5.2, 4.6));
    println!("5 + 4 = {}", get_sum_gen(5, 4));
}
