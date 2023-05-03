#[macro_export]
macro_rules! arcmutm {
    ($typename:path) => {
        Arc<Mutex<&mut $typename>>
    };

    ($typename:path, $lifetime:lifetime) => {
        Arc<Mutex<&$lifetime mut $typename>>
    };
}

#[macro_export]
macro_rules! new_arcmutm {
    ($value:expr) => {
        Arc::new(Mutex::new(&mut $value))
    };
}

#[macro_export]
macro_rules! arcmut {
    ($typename:path) => {
        Arc<Mutex<$typename>>
    };
}

#[macro_export]
macro_rules! new_arcmut {
    ($value:expr) => {
        Arc::new(Mutex::new($value))
    };
}

// Shamelessly stolen from https://users.rust-lang.org/t/how-to-store-async-function-pointer/38343/4
#[macro_export]
macro_rules! dyn_async {
    (
        $( #[$attr:meta] )* // includes doc strings
        $pub:vis
        async
        fn $fname:ident<$lt:lifetime> ( $($args:tt)* ) $(-> $Ret:ty)?
        {
            $($body:tt)*
        }
    ) => (
        $( #[$attr] )*
        #[allow(unused_parens)]
        $pub
        fn $fname<$lt> ( $($args)* ) -> ::std::pin::Pin<::std::boxed::Box<
            dyn ::std::future::Future<Output = ($($Ret)?)>
                + ::std::marker::Send + $lt
        >>
        {
            ::std::boxed::Box::pin(async move { $($body)* })
        }
    );
    (
        $( #[$attr:meta] )* // includes doc strings
        $pub:vis
        async
        fn $fname:ident( $($args:tt)* ) $(-> $Ret:ty)?
        {
            $($body:tt)*
        }
    ) => (
        $( #[$attr] )*
        #[allow(unused_parens)]
        $pub
        fn $fname( $($args)* ) -> ::std::pin::Pin<::std::boxed::Box<
            dyn ::std::future::Future<Output = ($($Ret)?)>
                + ::std::marker::Send
        >>
        {
            ::std::boxed::Box::pin(async move { $($body)* })
        }
    )
}
