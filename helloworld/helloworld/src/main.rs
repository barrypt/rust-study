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
    let gg = greet_world(3);

    match gg {
        Ok(t) => println!("{}", t),
        Err(e) => println!("{}", e),
    }
}
