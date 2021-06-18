#[derive(Clone, Copy)]
struct Point {x: i32, y: i32}

fn main() {
    let c = 'q';

    let ref_c2 =&c;
    let ref ref_c1 = c;

    println!("c1: {}, c2: {}, ref_c1 equals ref_c2: {}", ref_c1, ref_c2 ,ref_c1 == ref_c2);

    let point = Point {x:0 , y: 1};

    let __copy_x = {

        let Point{x: ref __x, y: _} = point;
        *__x
    };
    
}
