

use crate::*;

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

pub trait Applicative<'a> : Functor<'a>
{
    fn pure<A: 'a>(a: A) -> Self::of<A>;
    fn funmap<A: 'a, B: 'a>(f: Self::of<impl Fn(A) -> B + 'a + 'static + Copy>) -> impl Fn(Self::of<A>) -> Self::of<B>;
}

pub trait Monad<'a> : Applicative<'a>
{
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

pub trait Traversable<'a> : Functor<'a>
{
    fn foldr<A: 'a, B: 'a>(f: impl Fn(A,B) -> B, b: B, ta: Self::of<A>) -> B;

    declare_many!
    {
        // [ foldr for (A B : 'a). (f : (A , B) -> B) (b : B) (ta : Self[A]) : B = ()]
        [ sequence for (A : 'a) (F : Applicative<'a>). (xs : Self[F[A]]) : F[Self[A]] = () ]
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

impl<'a> Applicative<'a> for SymOption
{
    define_many!
    {
        [ pure for (A : 'a). (a : A) : Self[A]
        = Some(a)
        ]

        [ funmap for (A B : 'a). (f: Self[A -> B]) : Self[A] -> Self[B]
        | Some(a) => f.map(move |f| f(a))
        | None => None
        ]
    }
    
    // fn funmap<A: 'a, B: 'a>(f: Self::of<impl Fn(A) -> B + 'a>, aa: Self::of<A>) -> Self::of<B> {
    //     f.map(|f| f(aa))
    // }
}

impl<'a> Monad<'a> for SymOption
{

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


// Vector
pub struct SymVec {}

impl<'a, A: 'a> Unwrap<'a> for Vec<A>
{
    type A = A;
    type F = SymVec;
    
    fn coe(self) -> <Self::F as Functor<'a>>::of<Self::A> {
        self
    }
    
    fn uncoe(x: <Self::F as Functor<'a>>::of<Self::A>) -> Self {
        x
    }
}


impl<'a> Functor<'a> for SymVec
{
    type of<A: 'a> = Vec<A>;

    define_many!
    {
        [ map for (A B : 'a) (F : Fn(A) -> B + 'a). (f : F) (a: Self[A]) : Self[B]
        = a.into_iter().map(f).collect()
        ]
    }
}

impl<'a> Traversable<'a> for SymVec
{

    define_many!
    {
        [ sequence for (A : 'a) (F : Applicative<'a>). (xs : Self[F[A]]) : F[Self[A]]
        = todo!() // Self::foldr(|a,b| F::funmap(F::map(|aa| |bb| append(bb, a), a))(b), F::pure(Vec::new()), xs)
        ]
    }
    
    fn foldr<A: 'a, B: 'a>(f: impl Fn(A,B) -> B, b: B, ta: Self::of<A>) -> B {
        todo!()
    }
}






mod test
{
    use crate::{core::append, *};

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

        [ mybind for 'a A B (M : Monad<'a>). (f : A -> M[B]) (x : M[A]) : M[B] = do
            x <= x;
            y <= f(x);
            M::pure(y)
        ]

        [ mycomp for 'a (A B C : 'a) (M : Monad<'a>). 
            (f : A -> M[B])
            (g : B -> M[C])
               : A -> M[C]
            = a -> do b <= f(a); g(b)
        ]
        
        [ mapM for 'a (A B : 'a + Copy) (M : Monad<'a>) (As : Iterator<Item=A>). (f : A -> M[B]) (xs: As) : M[Vec B]
        = {
            let mut res = M::pure(Vec::new());
            for x in xs {
                res = mdo!{
                    res <= res;
                    y <= f(x);
                    M::pure(append(res.clone(), y))
                };
            }
            res
        }
        ]

    }

    #[test]
    pub fn mytest() {
        print!("{}", double2()(2));
        println!("{:?}", bind(|a| Some(a + 1), Some(7)))
        // forward!(test_eq, []=);
    }

}

