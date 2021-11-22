fn greet_world() {
    // !はマクロ
    println!("Hello world");
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー ワールド";
    let regions = [southern_germany, japan];

    //&でregionを借りる
    for region in regions.iter() {
        println!("{}", &region);
    }
}


fn main() {
    greet_world();
}
