
///////////////////////////////////////
// Generic

pub trait Unwrap<'a>
{
    type A: 'a;
    type F : Functor<'a, of<Self::A> = Self>;
}

pub trait Functor<'a>
{
    type of<A: 'a>: Unwrap<'a, A = A, F = Self> + 'a;

    fn map<A,B,F>(f: F, a: Self::of<A>) -> Self::of<B>
        where F: Fn(A) -> B + 'a,
        A: 'a,
        B: 'a;
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
// Instances

pub struct SymOption {}

impl<'a, A: 'a> Unwrap<'a> for Option<A>
{
    type A = A;
    type F = SymOption;
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
