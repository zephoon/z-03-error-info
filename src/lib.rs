#[macro_export]
macro_rules! my_vec {
    () => { Vec::new() };
    ($elem:expr;$n:expr) => { Vec::from_elem($elem, $n) };
    ($($x:expr), +) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}