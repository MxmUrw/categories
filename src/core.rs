
///////////////////////////////////////
// Generic

pub trait Unwrap<'a>
{
    type A: 'a;
    type F : Functor<'a, of<Self::A> = Self>;

    fn coe(self) -> <Self::F as Functor<'a>>::of<Self::A>;
    fn uncoe(x: <Self::F as Functor<'a>>::of<Self::A>) -> Self;
}

pub trait Functor<'a>
{
    type of<A: 'a>: Unwrap<'a, A = A, F = Self> + 'a;

    fn map<A,B,F>(f: F, a: Self::of<A>) -> Self::of<B>
        where F: Fn(A) -> B + 'a,
        A: 'a,
        B: 'a;

    // fn coe<A>(a: A) -> <Self::of<A> as Unwrap<'a>>::A;
    // fn uncoe<A>(a: <Self::of<A> as Unwrap<'a>>::A) -> A;
}

pub trait Monad<'a> : Functor<'a>
{
    fn pure<A: 'a>(a: A) -> Self::of<A>;
    fn bind<A,B,F>(a: Self::of<A>, f: F) -> Self::of<B>
        where F : Fn(A) -> Self::of<B>,
            A: 'a, B: 'a, F: 'a;
}

pub trait StrongMonad<'a> : Monad<'a>
{
    fn include<A,B>(a: Self::of<A>, b: B) -> Self::of<(A,B)>;

    fn merge<A: 'a, B: 'a>(ma: Self::of<A>, mb: Self::of<B>) -> Self::of<(A,B)>
    {
        Self::bind(Self::include(mb, ma), |(b, ma)| Self::include(ma, b))
    }
}


///////////////////////////////////////
// Type inference
pub trait MonadObject<'a> : Unwrap<'a>
{
    fn obind<MB : Unwrap<'a, F = Self::F>, F: Fn(Self::A) -> MB +'a>(self, f: F) -> MB;
}

impl<'a, MA: Unwrap<'a>> MonadObject<'a> for MA
where MA::F : Monad<'a>
{
    fn obind<MB : Unwrap<'a, F = MA::F>, Fun: Fn(Self::A) -> MB + 'a>(self, f: Fun) -> MB {
        let x = MB::F::bind::<Self::A, MB::A, _>(self, move |x : Self::A| {
            let y = f(x);
            y.coe()
            // <MA::F as Functor<'a>>::coe(y)
        });
        Unwrap::uncoe(x)
    }
}


///////////////////////////////////////
// Instances

pub struct SymOption {}

impl<'a, A: 'a> Unwrap<'a> for Option<A>
{
    type A = A;
    type F = SymOption;
    
    fn coe(self) -> <Self::F as Functor<'a>>::of<Self::A> {
        self
    }
    
    fn uncoe(x: <Self::F as Functor<'a>>::of<Self::A>) -> Self {
        x
    }
}

impl<'a> Functor<'a> for SymOption
{
    type of<A: 'a> = Option<A>;

    fn map<A,B,F>(f: F, a: Self::of<A>) -> Self::of<B>
        where F: Fn(A) -> B,
        A: 'a, B: 'a, F: 'a
    {
        a.map(f)
    }
}

impl<'a> Monad<'a> for SymOption
{
    fn pure<A: 'a>(a: A) -> Self::of<A> {
        Some(a)
    }

    fn bind<A: 'a, B: 'a ,F: 'a>(a: Self::of<A>, f: F) -> Self::of<B>
        where F : Fn(A) -> Self::of<B> 
    {
        a.and_then(f)
    }
}


pub fn append<A>(mut xs: Vec<A>, x: A) -> Vec<A> {
    xs.push(x);
    xs
}

// pub fn merge
// <M: StrongMonad, A, B>
// (ma: M::Apply<A>, mb: M::Apply<B>) -> M::Apply<(A,B)>
// {
//     M::bind(M::include(mb, ma), |(b, ma)| M::include(ma, b))
// }

pub fn mapm
<'a, M: StrongMonad<'a>, A: Clone, B: Clone, F: Fn(A) -> M::of<B>>
(f: F, xs: Vec<A>) -> M::of<Vec<B>>
{
    xs.into_iter().map(f).fold(M::pure(Vec::new()), |xs, x| M::bind(M::merge(xs,x), |(xs,x)| M::pure(append(xs, x))))
}



////////////////////////////////////////
// helpers

macro_rules! mdo {
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
macro_rules! define {

    ($fn_name:ident : $type:ty = $($tokens:tt)+) => {

        #[allow(unused)]
        fn $fn_name() -> $type {
            define_expr!($($tokens)+)
        }

    };
}

macro_rules! define2 {
    // ($fn_name:ident : $($tokens:tt)+) => {
    //     type_and_term!{define2callback, {args= {name=$fn_name} } [] $($tokens)+}
    // };

    ($fn_name:ident $(($var:ident : $($type:tt)+))* : $($tokens:tt)+) => {
        type_and_term!{define2callback, {args= {name=$fn_name} {direct_vars= $(($var : $($type)+))*} } [] $($tokens)+}
    };
}

macro_rules! define2callback {

    ({name = $fn_name:ident} 
        {direct_vars= $(($var_name:ident : $($var_type:tt)+))*}
        {type = $($type:tt)+} {expr = $($expr:tt)+}) => {

        #[allow(unused)]
        fn $fn_name(
            $(
                $var_name : make_type_top!($($var_type)+)
            ),*

        ) -> make_type_top!($($type)+) {
            define_expr!($($expr)+)
        }

    };

    ({name = $fn_name:ident}
        {direct_vars= $(($var_name:ident : $($var_type:tt)+))*}
         {type = $($type:tt)+} 
         {exprs= $([$x:pat => $($expr:tt)+])* }
        ) 
        => {

        #[allow(unused)]
        fn $fn_name(
            $(
                $var_name : make_type_top!($($var_type)+)
            ),*

        ) -> make_type_top!($($type)+) {
            |a| match a
            {
                $($x => define_expr!($($expr)+)),*
            }
        }
    };
}

macro_rules! define_many {
    ($([ $($def:tt)+ ])*) => {
        $(
            define2!{$($def)+}
        )*

        // call_many!{define, [] $($def)+}
    };
}

// macro_rules! define_many {
//     ([$($def:tt)+] $($defs:tt)*) => {
//     // ($($def:tt)+) => {
//         define!{$($def)+}
//         define!{$($defs)+}
//         // define_many!($($defs:tt)+)
//     };
// }


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

macro_rules! make_type {
    ($t1:tt -> $($t2:tt)+) => {
        impl Fn(make_type!($t1)) -> make_type!( $($t2)+ )
    };
    ($t1:ident $t2:ident) => {
        $t1<$t2>
    };
    ($type:ty) => {
        $type
    };
}

macro_rules! make_type_top {
    ($head:ident $($arg:tt)*) => {
        make_type_gen!({head = $head} {args=} $($arg)*)
    }
}

macro_rules! make_type_gen {
    // ($t1:tt -> $($t2:tt)+) => {
    //     impl Fn(make_type!($t1)) -> make_type!( $($t2)+ )
    // };
    // ({head=$head:ident} {args=} -> $($rest:tt)+) => {
    //     impl Fn($head) -> make_type_top!($($rest)+)
    // };
    ({head=$head:ident} {args=$($arg:ident)*} -> $($rest:tt)+) => {
        impl Fn($head$(<$arg>),*) -> make_type_top!($($rest)+)
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






mod test
{
    use super::{Monad, MonadObject};

    pub fn my_fun
    <'a, M: Monad<'a>, A: 'a, B: 'a, C: 'a>
    (f: impl Fn(A) -> M::of<B> + 'a, g: impl Fn(B) -> M::of<C> + 'a) -> impl FnOnce(A) -> M::of<C>
    {
        move |a| mdo!(
            aa <= f(a);
            bb <= g(aa);
            M::pure(bb)
        )
    }

    define!
    {
        double : impl Fn(i8) -> i8 = a -> a * 2
    }


    define!
    {
        triple : impl Fn(i8) -> i8 = do |a| a * 3
    }

    define_many!
    {
        [ double2 : i8 -> i8
        | a => a * 2
        ]

        [ prod : i8 -> i8 -> i8
        = a -> b -> a * b
        ]

        [ test : Option u8 -> u8
        | Some(a) => 1
        | None => 2
        ]

        [ sum (a: u8) (b: u8) : u8
        = a + b
        ]

        [ bind (f : u8 -> Option u8) (x : Option u8) : Option u8 = do
           x <= x;
           y <= f(x);
           Some(y)
        ]

    }

    #[test]
    pub fn mytest() {
        print!("{}", double2()(2));
        println!("{:?}", bind(|a| Some(a + 1), Some(7)))
        // forward!(test_eq, []=);
    }

}

