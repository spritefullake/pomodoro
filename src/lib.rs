#[macro_export]
macro_rules! map_commands {
    ($( $command:ident = $type:ty ; $value:ident),+) => {
        {
        let mut map = std::collections::HashMap::new();
        $( 
            map.insert(stringify!($command), stringify!($value::$type));
            println!("{}",stringify!($type));

        ),+

        
        map
    }
    };
}
