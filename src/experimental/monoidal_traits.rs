

// trait HList
// {
// }


trait Monoid
{
    fn one() -> Self;
    fn mult(x: Self, y: Self) -> Self;
}

trait Convert
{
    type Target;
}

trait Trait
{
    type T;
    type Conv<X: Convert<Target = Self::T>>;

    // fn construct<Other: Trait<T = Self::T>>(t: Other::T) -> u8;
    fn construct<X: Convert<Target = Self::T>>(t: Self::Conv<X>) -> u8;
}

struct SymMonoid;

/*
impl Trait for SymMonoid
{
    type T = impl Monoid;
    
    type Conv<X: Convert<Target = Self::T>> = X;
    
    fn construct<X: Convert<Target = Self::T>>(t: Self::Conv<X>) -> u8 {
        todo!()
    }
    
    // fn construct<X: Convert<Target = Self::T>>(t: X) -> u8 {
    //     todo!()
    // }
    // type T<X: Convert<Target = Self::T<X>>> = impl Monoid;
    
    // fn construct<X: Convert<Target = Self::T<X>>>(t: X) -> u8 {
    //     todo!()
    // }
    // type T<X: Into<Self::T<X>>> = impl Monoid;
    
    // fn construct<X: Into<Self::T<X>>>(t: X) -> u8 {
    //     todo!()
    // }
    // type T = impl Monoid;
    
    // fn construct(t: impl Trait<T = Self::T>) -> u8 {
    //     0
    // }
    
    // fn construct<Other: Trait<T = Self::T>>(t: Other::T) -> u8 {
    //     0
    // }

}

 */





