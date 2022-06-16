pub fn verse(n: i32) -> String {
    if n == 0{
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy \
        some more, 99 bottles of beer on the wall.\n".to_owned();
    }
    format!("{} of beer on the wall, {} of beer.\nTake {} down and pass it around, \
    {} of beer on the wall.\n",get_str(n),get_str(n),get_it(n),get_str(n-1))
}

pub fn sing(start: i32, end: i32) -> String {
    let mut s = String::new();
    let mut index = start;
    while index >= end {
        s += verse(index).as_str();
        if index != end {
            s += "\n";
        }
        index -= 1;
    }
    s
}

fn get_str(n:i32)->String{
    match n {
        0=>"no more bottles".to_owned(),
        1=>format!("{} bottle",1),
        _=>format!("{} bottles",n),
    }
}
fn get_it(n:i32)->String{
    match n {
        1=>"it".to_owned(),
        _=>"one".to_owned(),
    }
}