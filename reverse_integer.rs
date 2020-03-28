

fn main(){
    let reversed = reverse(123);
    println!("{}", reversed);
    println!("Done!");
}

pub fn reverse(x: i32) -> i32 {
    let building = x.to_string().chars().rev().collect::<String>();
    return building.parse::<i32>().unwrap();
}
