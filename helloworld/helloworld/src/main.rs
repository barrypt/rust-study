use std::vec;
use std::collections::HashMap;
fn greet_world(a: u32) -> Result<String, bool> {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
    if a == 1 {
        Ok("1230".to_string())
    } else {
        Err(false)
    }
}
fn main() {
    let mut v = 0;
    for i in 1..10 {
        v = if i == 9 { continue } else { i }
    }
    println!("{}", v);
    let gg = greet_world(1);
    let aaaa = gg.unwrap();
    println!("{}", aaaa);

    let mut ary = vec![4, 8, 6];
    ary.push(3);
    for (indexs, value) in ary.iter().enumerate() {
        println!("{0},{1}", indexs, value)
    }

    let mut scores = HashMap::new();

    scores.insert("1", "2");
    scores.insert("3", "5");

    let loopscore = &scores;
    for indexss in loopscore {
        println!("{},{}", indexss.0, indexss.1)
    }

    let mp1 = scores.get("1");
    let jj = mp1.is_some();
    if jj {
        match mp1 {
            None => print!("kong"),
            Some(b) => println!("1122{}", b),
        }
    }

    let arry1 = [1, 2, 3, 4, 5];
    println!("{}", arry1.len())
    // match gg {
    //  Ok(t) => println!("{}", t),
    //  Err(e) => println!("{}", e),
    // }
}
