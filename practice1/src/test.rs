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

    fn sum_vec(v: &Vec<i32>) -> i32 {
        return v.iter().fold(0, |a, &b| a+b );
    }

    fn sum_two_vec(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        // Do stuff with 'v1' and 'v2'
        let s1=sum_vec(v1);
        let s2=sum_vec(v2);
        s1+s2
    }
    println!();
    for (index, value) in (5..10).enumerate() {
        println!("index={} and value={}",index,value);
    }
    //print_all_odds(10,12);
    let v1=vec![1,2,3];
    let v2=vec![4,5,6];

    let answer=sum_two_vec(&v1,&v2);
    println!("Answer: {}", answer);
    get_second_element(&v1);
}

fn change_truth(x: bool) -> bool {
    !x
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

fn get_second_element(v: &Vec<i32>) {
    match v.get(1) {
        Some(x) => println!("Item 1 in the given vector is {}",x),
        None    => println!("Sorry, this vector isn't long enough")
    }
}
