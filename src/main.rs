
fn main() {
    let window_width = 200;
    let window_height = 100;

    println!("P3\n{} {}\n255", window_width, window_height);
    for j in (0..window_height).rev() {
        for i in 0..window_width {
            // as is typecasting 
            let r = i as f32 / window_width as f32;
            let g = j as f32 / window_height as f32;
            let b = 0.2 as f32;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}