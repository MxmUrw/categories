

// macro_rules! prepend {
//     (item={ $item:tt } acc={ $op:tt [ $($head:tt)* ] $($tail:tt)* }) => {
//          $op [ $($head)* $item ] $($tail)* 
//     };
//     (item={ $item:tt } acc={ $($current:tt)+ }) => {
//         $($current)+ $item
//     };
// }

// macro_rules! prepend_and_parse_type {
//     (
//         item={ $item:tt }
//         into={ $op:tt [ $($head:tt)* ] $($tail:tt)* }
//         forward_tail={ $($forward_tail:tt)* }
//     ) => {

//          $op [ $($head)* $item ] $($tail)* 
//     };

//     (
//         item={ $item:tt }
//         into={ $($current:tt)+ } 
//         forward_tail={ $($forward_tail:tt)* }
//     ) => {
//         parse_type(current={ $($current)+ $item } tail={ $($forward_tail)* } )  
//     };
// }

enum Either<A,B> {
    Left(A),
    Right(B)
}

// macro_rules! single_op {
//     (op=+ arg0={$($arg0:tt)*} arg1={$($arg1:tt)*}) => {
//         Either<
        
//     };
// }

// macro_rules! eval_op_impl {
//     ({$($acc:tt)*} + [$($arg0:tt)*] $($args:tt)*) => {
//         eval_op_impl!({Either<$($arg0)*,$($acc)*>} + $($args)*)
//     };

//     ({$($acc:tt)*} $op:tt ) => {
//         $($acc)*
//     }
// }





macro_rules! eval_op {
    ($op:tt [$($arg0:tt)*]) => {
        dispatch_operand!($($arg0)*)
    };

    (+ [$($head:tt)*] $($tail:tt)+) => {
        Either<dispatch_operand!($($head)*), eval_op!(+ $($tail)+ )>
        
    };

    (* $( [ $($arg:tt)* ] )+ ) => {

        // a big tuple with all args
        (
            $(
                dispatch_operand!($($arg)*)
            ),+
        )

    };
}

macro_rules! dispatch_operand {
    // left-over half-parsed nodes are wrapped in `{}`
    ({ $op:tt $($operand:tt)+ }) => {
        eval_op_inv!( $op input={$($operand)+} result={} )
    };

    // all other tokens are unparsed
    ($($tokens:tt)+ ) => {
        parse_type_leaf!(input={ $($tokens)+ } result={})
    };
}

macro_rules! eval_op_inv {
    (
        $op:tt
        input={ [$($arg0:tt)*] $($args:tt)* }
        result={ $($result:tt)* } 
    ) => {
        eval_op_inv!( $op input={$($args)*} result={ [ $($arg0)* ] $($result)*  })
    };

    (
        $op:tt
        input={}
        result={ $($result:tt)* } 
    ) => {
        eval_op!( $op $($result)*)
    };
}


macro_rules! parse_type_plus {
    // + / +
    (current={ + [$($acc0:tt)*] $($accs:tt)* } tail={ + $($tail:tt)* } ) => {

        parse_type_plus!
        (
            current={ + [] [$($acc0)*] $($accs)* }
            tail={ $($tail)* }
        )

    };

    // push head 
    (current={ $op:tt [$($acc0:tt)*] $($accs:tt)* } tail={ $head:tt $($tail:tt)* } ) => {

        parse_type_plus!
        (
            current={ $op [$($acc0)* $head] $($accs)* }
            tail={ $($tail)* }
        )

    };

    (current={ $op:tt $($accs:tt)* } tail={} ) => {
        eval_op_inv!($op input={$($accs)*} result={})
    };
}

macro_rules! parse_type_mult {
    // * / *
    (current={ * [$($acc0:tt)*] $($accs:tt)* } tail={ * $($tail:tt)* } ) => {

        parse_type_mult!
        (
            current={ * [] [$($acc0)*] $($accs)* }
            tail={ $($tail)* }
        )

    };

    // * / +
    (current={ * [$($acc0:tt)*] $($accs:tt)* } tail={ + $($tail:tt)* } ) => {

        parse_type_plus!
        (
            current={ + [] [ { * [$($acc0)*] $($accs)* } ] }
            tail={ $($tail)* }
        )

    };

    // push head 
    (current={ $op:tt [$($acc0:tt)*] $($accs:tt)* } tail={ $head:tt $($tail:tt)* } ) => {

        parse_type_mult!
        (
            current={ $op [$($acc0)* $head] $($accs)* }
            tail={ $($tail)* }
        )

    };

    (current={ $op:tt $($accs:tt)* } tail={} ) => {
        eval_op_inv!($op input={$($accs)*} result={})
    };
}


macro_rules! parse_type_leaf {
    // encounter + operator
    (input={ + $($input:tt)* } result={ $($result:tt)* }) => {
        parse_type_plus!(current={ + [] [ $($result)* ] } tail={ $($input)* })
    };

    // encounter * operator
    (input={ * $($input:tt)* } result={ $($result:tt)* }) => {
        parse_type_mult!(current={ * [] [ $($result)* ] } tail={ $($input)* })
    };

    // take other token
    (input={ $input_head:tt $($input_tail:tt)* } result={ $($result:tt)* }) => {
        parse_type_leaf!(input={ $($input_tail)* } result={ $($result)* $input_head })
    };

    // done
    (input={} result={ $($result:tt)* }) => {
        eval_type_leaf!( $($result)* )
    };
}

macro_rules! eval_type_leaf {
    // ($single:tt) => { $single };
    // ($head:tt $($tail:tt)+) => { $head<$($tail),+> }
    ($($tail:tt)*) => { process_token_trees!( input={ $($tail)* } result={}) }
}

macro_rules! process_token_trees {

    (input={ $head_functor:tt[$($head_args:tt)+] $($tail:tt)*} result={$($result:tt)*} ) => {
        process_token_trees!(
            input={$($tail)*}
            result={ $($result)* (functor=($head_functor)($($head_args)+)) }
        )
    };

    (input={ $head:tt $($tail:tt)*} result={$($result:tt)*} ) => {

        process_token_trees!(
            input={$($tail)*}
            result={ $($result)* (token=($head)) }
        )
        
    };

    // (input={ ( $($head:tt)* ) $($tail:tt)*} result={$($result:tt)*} ) => {

    //     process_token_trees!(
    //         input={$($tail)*}
    //         result={ $($result)* (parens=( $($head)* )) }
    //     )
        
    // };

    (input={} result={$($result:tt)*} ) => {
        process_type_applications!($($result)*)
    };
}

// macro_rules! process_token_trees_eval {
//     ( $( ($category:tt = $($value:tt)* ) )* ) => { $( ( process_token_trees_single_eval!($category = $($value)* ) ) )*  }
// }

macro_rules! expand_tt {
    (token=(($($value:tt)*))) => { eval_type!($($value)*) };
    (token=($token:tt)) => { $token };
    (functor=($head:tt)($($args:tt)*)) => { $head::of<eval_type!($($args)*)> }
}

macro_rules! process_type_applications {
    (( $($single:tt)+ )) => { expand_tt!($($single)+) };
    ( (token=($token:tt)) $( ( $($tail:tt)+ ) )* ) => { $token<$( expand_tt!($($tail)+) ),*> };
    ( ($($head:tt)+) $( ( $($tail:tt)+ ) )* ) => { expand_tt!($($head)+)<$( expand_tt!($($tail)+) ),*> };
}

macro_rules! eval_type {
    ($($tokens:tt)*) => {
        parse_type_leaf!(input={ $($tokens)* } result={})
    };
}

macro_rules! assert_type_eq {
    ($expr:ty, $expr2:ty) => {
        let _ = |a: $expr| -> $expr2 {a};
    };
}


mod test
{

    use crate::type_macros::Either;

    fn testf()
    {
        // let x: eval_type!(i8 + i8) = Either::Right(-1i8);
        let x: eval_type!(Option i8 + i8) = Either::Left(Some(0i8));

        assert_type_eq!(
            eval_type!(Option i8 + i8),
            Either<Option<i8>, i8>
        );

        assert_type_eq!(
            eval_type!(i8 * i8),
            (i8, i8)
        );

        assert_type_eq!(
            eval_type!(i8 * i8 * i8),
            (i8, i8, i8)
        );

        assert_type_eq!(
            eval_type!(i8 * i8 + i8),
            Either<(i8, i8), i8>
        );

        assert_type_eq!(
            eval_type!(i8 + i8 * i32 + i64),
            Either<i8, Either<(i8, i32), i64>>
        );

        assert_type_eq!(
            eval_type!(Option i8 + Option i64),
            Either<Option<i8>, Option<i64>>
        );

        assert_type_eq!(
            eval_type!(Option (Option i8)),
            Option<Option<i8>>
        );

    }

}



