mod mc;

mod sound {
    pub mod instrument {
        pub fn clarinet() {
            super::voice::hello();
        }
        pub mod inner {
            pub fn clarinet() {
                super::super::voice::hello();
            }
        }
    }

    pub mod voice {
        pub fn hello() {
            println!("hello");
        }
    }
}

fn main() {
    create_function!(foo);
    
    let _t =crate::sound::instrument::clarinet();

    println!("{:?}", _t);
    foo();
}