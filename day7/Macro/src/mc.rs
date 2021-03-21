
pub mod mc
{
    #[macro_export]
    macro_rules! create_function {
        ($func_name:ident) => {
            
            fn $func_name() {
                println!("function {:?} is called", stringify!(func_name));
            }
            
        };
    }

    fn one_addo_one()->i32
    {
        return 2;
    }
}