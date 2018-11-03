macro_rules! unwrap_or_exit {
    ( $e:expr, $m:expr ) => {
        $e.unwrap_or_else(|err| {
            eprintln!("{}: {:?}", $m, err);
            std::process::exit(0);
        });
    }
}

macro_rules! exit {
    ( $m:expr ) => { 
        eprintln!("{}", $m); 
        std::process::exit(0); 
    } 
}