fn greet_world(a : int32) -> Result<string,bool> {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
if(a==1)
   Ok("123")
   else
    Err(false)
}

fn main() {
    let mut v = 0;
    for i in 1..10 {
        v = if i == 9 { continue } else { i }
    }
    println!("{}", v);
    greet_world();
}
