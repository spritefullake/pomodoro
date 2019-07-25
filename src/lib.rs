#[macro_export]
macro_rules! map_commands {
    ($( $command:ident : $value:ident ),+) => {
        let map = HashMap::new();
        $( 
            map.insert($command, $value);
        ),+
        map
    };
}