
use crate::type_macros::*;

////////////////////////////////////////
// helpers


#[macro_export]
macro_rules! mdo {
    (return $($rest:tt)+) => {
        crate::core::opure($($rest)+)
    };
    ($expr:expr) => {$expr};
    (let $var:ident = $expr:expr; $($expr2:tt)+ ) => {
        {
            let $var = $expr;   
            mdo!($($expr2)+)
        }
    };
    ($var:ident <= $expr:expr; $($expr2:tt)+) => {
        $expr.obind(move |$var| mdo!($($expr2)+))
    };
}


//-- definitions
#[macro_export]
macro_rules! define {

    ($fn_name:ident : $type:ty = $($tokens:tt)+) => {

        #[allow(unused)]
        fn $fn_name() -> $type {
            define_expr!($($tokens)+)
        }

    };
}

#[macro_export]
macro_rules! define2 {
    // ($fn_name:ident : $($tokens:tt)+) => {
    //     type_and_term!{define2callback, {args= {name=$fn_name} } [] $($tokens)+}
    // };

    ($callback:ident, $fn_name:ident [$($params:tt)*] for . $(($var:ident : $($type:tt)+))* : $($tokens:tt)+) => {
        type_and_term!{$callback, {args= {name=$fn_name} {direct_vars= $(($var : $($type)+))*} {params=$($params)*} } [] $($tokens)+}
    };

    ($callback:ident, $fn_name:ident [$($params:tt)*] for $param:ident $($rest:tt)+) => {
        define2!{$callback, $fn_name [ $($params)* $param, ] for $($rest)+}
    };

    ($callback:ident, $fn_name:ident [$($params:tt)*] for $param:lifetime $($rest:tt)+) => {
        define2!{$callback, $fn_name [ $($params)* $param, ] for $($rest)+}
    };

    ($callback:ident, $fn_name:ident [$($params:tt)*] for ($param_name:ident $($param_names:ident)* : $($param:tt)+) $($rest:tt)+) => {
        define2!{$callback, $fn_name [ $($params)* $param_name : $($param)+, ] for ($($param_names)* : $($param)+) $($rest)+}
    };

    ($callback:ident, $fn_name:ident [$($params:tt)*] for (: $($param:tt)+) $($rest:tt)+) => {
        define2!{$callback, $fn_name [ $($params)* ] for $($rest)+}
    };

    ($callback:ident, $fn_name:ident for $($rest:tt)+) => {
        define2!{$callback, $fn_name [] for $($rest)+}
    };

    ($callback:ident, $fn_name:ident $(($var:ident : $($type:tt)+))* : $($tokens:tt)+) => {
        type_and_term!{$callback, {args= {name=$fn_name} {direct_vars= $(($var : $($type)+))*} {params=} } [] $($tokens)+}
    };
}

#[macro_export]
macro_rules! declare2callback {

    ({name = $fn_name:ident} 
        {direct_vars= $(($var_name:ident : $($var_type:tt)+))*}
        {params = $($params:tt)*}
        {type = $($type:tt)+} 
        {expr = ()}) => {

        #[allow(unused)]
        fn $fn_name<$($params)*>
        (
            $(
                $var_name : make_type_top!($($var_type)+)
            ),*

        ) -> make_type_top!($($type)+);
    };
}


#[macro_export]
macro_rules! define2callback {

    ({name = $fn_name:ident} 
        {direct_vars= $(($var_name:ident : $($var_type:tt)+))*}
        {params = $($params:tt)*}
        {type = $($type:tt)+} 
        {expr = $($expr:tt)+}) => {

        #[allow(unused)]
        fn $fn_name<$($params)*>
        (
            $(
                $var_name : make_type_top!($($var_type)+)
            ),*

        ) -> make_type_top!($($type)+) {
            define_expr!($($expr)+)
        }

    };

    ({name = $fn_name:ident}
        {direct_vars= $(($var_name:ident : $($var_type:tt)+))*}
        {params = $($params:tt)*}
         {type = $($type:tt)+} 
         {exprs= $([$x:pat => $($expr:tt)+])* }
        ) 
        => {

        #[allow(unused)]
        fn $fn_name<$($params)*>
        (
            $(
                $var_name : make_type_top!($($var_type)+)
            ),*

        ) -> make_type_top!($($type)+) {
            move |a| match a
            {
                $($x => define_expr!($($expr)+)),*
            }
        }
    };
}

#[macro_export]
macro_rules! define_many {
    ($([ $($def:tt)+ ])*) => {
        $(
            $crate::define2!{define2callback, $($def)+}
        )*
        // call_many!{define, [] $($def)+}
    };
}

#[macro_export]
macro_rules! declare_many {
    ($([ $($def:tt)+ ])*) => {
        $(
            $crate::define2!{declare2callback, $($def)+}
        )*
        // call_many!{define, [] $($def)+}
    };
}


#[macro_export]
macro_rules! define_expr {
    ($x:ident -> $($expr:tt)+) => {
        move |$x| define_expr!($($expr)+)
    };
    (do $($expr:tt)+) => {
        mdo!($($expr)+)
    };
    ($expr:expr) => {
        $expr
    };
}

#[macro_export]
macro_rules! make_type {
    ($t1:tt -> $($t2:tt)+) => {
        impl Fn(make_type!($t1)) -> make_type!( $($t2)+ ) + 'static
    };
    ($t1:ident $t2:ident) => {
        $t1<$t2>
    };
    ($type:ty) => {
        $type
    };
}

#[macro_export]
macro_rules! make_type_top {
    ($head:ident [$($arg:tt)*]) => {
        $head::of<make_type_top!($($arg)*)>
    };

    ($head:ident $($arg:tt)*) => {
        make_type_gen!({head = $head} {args=} $($arg)*)
    }

    // (($head:tt) $($arg:tt)*) => {
    //     make_type_gen!({head = $head} {args=} $($arg)*)
    // }
}

#[macro_export]
macro_rules! make_type_gen {
    // ($t1:tt -> $($t2:tt)+) => {
    //     impl Fn(make_type!($t1)) -> make_type!( $($t2)+ )
    // };
    // ({head=$head:ident} {args=} -> $($rest:tt)+) => {
    //     impl Fn($head) -> make_type_top!($($rest)+)
    // };
    ({head=$head:ident} {args=$($arg:ident)*} -> $($rest:tt)+) => {
        impl Fn($head$(<$arg>),*) -> make_type_top!($($rest)+) + 'static + Copy
    };
    ({head=$head:ident} {args=$($arg:ident)*} [$($inside:tt)*] -> $($rest:tt)+) => {
        impl Fn($head$(<$arg>),*::of<make_type_top!($($inside)+)>) -> make_type_top!($($rest)+) + 'static + Copy
    };
    ({head=$head:ident} {args=$($arg:ident)*} $cur:ident $($rest:tt)*) => {
        make_type_gen!({head=$head} {args=$($arg)* $cur} $($rest)*)
    };
    // ({head=$head:ident} {args=} ) => {
    //     $head
    // };
    ({head=$head:ident} {args=$($arg:ident)*} ) => {
        $head$(<$arg>),*
    };
    ($type:ty) => {
        $type
    };
}

#[macro_export]
macro_rules! type_and_term {

    ($callback:ident, {args=$({$($args:tt)+})+} [$($type:tt)*] | $x:pat => $($expr:tt)+) => {
        type_and_term!{$callback, {args=$({$($args)+})+} {type=$($type)*} {exprs=} [$x =>] $($expr)+ }
    };

    ($callback:ident,
        {args=$({$($args:tt)+})+}
        {type=$($type:tt)*}
        { exprs= $([$x:pat => $($expr:tt)+])* }
        [$cur_x:pat => $($cur_expr:tt)*]
        | $y:pat => $($tail:tt)+
    ) =>
    {
        type_and_term!{$callback, {args=$({$($args)+})+} {type=$($type)*} { exprs= $([$x => $($expr)+])* [$cur_x => $($cur_expr)+] } [$y =>] $($tail)+ }
    };

    ($callback:ident,
        {args=$({$($args:tt)+})+}
        {type=$($type:tt)*}
        { exprs= $([$x:pat => $($expr:tt)+])* }
        [$cur_x:pat => $($cur_expr:tt)+]
        ) =>
    {
        $callback!{$({$($args)+})+ {type=$($type)*} { exprs= $([$x => $($expr)+])* [$cur_x => $($cur_expr)+] } }
    };

    ($callback:ident,
        {args=$({$($args:tt)+})+}
        {type=$($type:tt)*}
        { exprs= $([$x:pat => $($expr:tt)+])* }
        [$cur_x:pat => $($cur_expr:tt)*]
        $head:tt $($tail:tt)*) =>
    {
        type_and_term!{$callback, {args=$({$($args)+})+} {type=$($type)*} { exprs= $([$x => $($expr)+])* } [$cur_x => $($cur_expr)* $head] $($tail)* }
    };

    ($callback:ident, {args=$({$($args:tt)+})+} [$($type:tt)*] = $($expr:tt)+) => {
        $callback!{$({$($args)+})+ {type=$($type)*} {expr= $($expr)+} }
    };

    // ($callback:ident, $({$($args:tt)+})+ {type=$($type:tt)*} [$x:pat $($expr:tt)+] | $x:pat => $($expr:tt)+) => {
    // };

    ($callback:ident, {args=$({$($args:tt)+})+} [$($type:tt)*] $type0:tt $($expr:tt)+) => {
        type_and_term!{$callback, {args=$({$($args)+})+} [$($type)* $type0] $($expr)+}
    };
}



