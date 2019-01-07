fn main(){
    let x : i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    print_number(x);
    print_number(add_one(x));
    let a=[0,1,2,3,4];
    for i in 0..5 {
        print!("{} ",a[i]);
    }
    println!();
    for (index, value) in (5..10).enumerate() {
        println!("index={} and value={}",index,value);
    }
    print_all_odds(10,12);
}

fn print_number(x: i32) {
    println!("Val is {}", x);
}

fn add_one(x: i32) -> i32 {
    x+1
}

fn print_all_odds(x: i32,y: i32) {
    'outer: for x in 0..x {
        'inner: for y in 0..y {
            if x%2==0 {continue 'outer; }
            if y%2==0 {continue 'inner; }
            println!("x: {}, y: {}", x, y);
        }
    }
}
