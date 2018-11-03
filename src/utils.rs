macro_rules! unwrap_or_exit {
    ( $e:expr, $m:expr ) => {
        $e.unwrap_or_else(|err| {
            error!("{}: {:?}", $m, err);
            std::process::exit(0);
        });
    }
}