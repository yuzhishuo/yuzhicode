use std::ops::Add;

#[derive(Debug)]
struct Point <T : Add<T, Output = T>> 
{
    x: T,
    y: T,
}

impl<T : Add<T, Output = T>>  Add for Point<T>{
    type Output =Point<T>;

    fn add(self, p: Point<T> ) -> Point<T>
    {

        Point{
            x: self.x + p.x,
            y: self.y + p.y
        }
    }


}

fn main() {

    let p1 = Point{x: 1.0f32, y: 2.0f32};
    let p2 = Point{x: 1.0f32, y: 2.0f32};

    let p3 = p1+p2;
    println!("{:?}", p3);
}
