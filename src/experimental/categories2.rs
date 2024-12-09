
/*
trait IsCategory 
{
    type Comp<A,B,C,F: Arrow<Self,A,B>, G: Arrow<Self, B, C>>;
    fn comp<A,B,C,F: Arrow<Self,A,B>, G: Arrow<Self, B, C>>(f: F, g: G) -> Self::Comp<A,B,C, F, G>;

    type Hom<A,B,X: Arrow<Self, A,B>>;
    
    // type Get<F: BiFiber<Self>>;
    // fn get<F: BiFiber<Self>>(f: F) -> Self::Get<F>;

}

struct SymSet {}

 */
/*

trait Arrow<Cat: ?Sized + IsCategory, A,B>
{
    type C;
    fn get(self) -> Self::C;
    fn comp<C>(self, other: impl Arrow<Cat, B, C, C = impl Fn(B) -> C>) -> impl Arrow<Cat, A,C>;
    fn comp2<C, X: Arrow<Cat, B, C>>(self, other: Cat::Hom<B,C,X>) -> impl Arrow<Cat, A,C>;
}

impl<A,B, Y: FnOnce(A) -> B> Arrow<SymSet, A,B> for Y
{
    type C = impl FnOnce(A) -> B;

    fn get(self) -> Self::C {
        self
    }
    
    fn comp<C>(self, other: impl Arrow<SymSet, B, C, C = impl FnOnce(B) -> C>) -> impl Arrow<SymSet, A,C> {
        |a| Arrow::get(other)(self(a))
    }
    
    fn comp2<C, X: Arrow<SymSet, B, C>>(self, other: <SymSet as IsCategory>::Hom<B,C,X>) -> impl Arrow<SymSet, A,C> {
        // |a| Arrow::get(other)(self(a))
        todo!()
    }
    
    // fn comp<C>(self, other: impl Arrow<SymSet, B, C>) -> impl Arrow<SymSet, A,C> {
    //     // |a| Arrow::get(other))(self(a))
    // }
}


/*
impl IsCategory for SymSet
{
    type Comp<A,B,C,F: Arrow<Self,A,B>, G: Arrow<Self, B, C>> = impl FnOnce(A) -> C;

    
    fn comp<A,B,C,F: Arrow<Self,A,B>, G: Arrow<Self, B, C>>(f: F, g: G) -> Self::Comp<A,B,C, F, G> {
        todo!()
        // f.comp(g)
        // let ff = F::get(f);
        // let gg = G::get(g);
        // |a| gg(ff(a))
    }

    type Hom<A,B,X: Arrow<Self, A,B>> = X::C;
    
    // type Comp<F: BiFiber<Self>, G: BiFiber<Self, A=F::B>> = impl Fn(F::A) -> G::B;

    // fn comp<F: BiFiber<Self>, G: BiFiber<Self, A=F::B>>(f: F, g: G) -> Self::Comp<F, G> {
    //     todo!()
    // }
    
    // type Get<F: BiFiber<Self>> = impl Fn(F::A) -> F::B;
    
    // fn get<F: BiFiber<Self>>(f: F) -> Self::Get<F> {
    //     todo!()
    // }
}

 */



 */