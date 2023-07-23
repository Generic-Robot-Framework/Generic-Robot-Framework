#[macro_export]
macro_rules! main_loop {
    ($node: expr, $main: expr) => {
        while $node.ok {
            $main()
        }
    }
}