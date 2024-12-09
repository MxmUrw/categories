
trait IsCategory 
{
    type Comp<F: BiFiber<Self>, G: BiFiber<Self, A=F::B>>;
    
    type Get<F: BiFiber<Self>>;

    fn get<F: BiFiber<Self>>(f: F) -> Self::Get<F>;

    fn comp<F: BiFiber<Self>, G: BiFiber<Self, A=F::B>>(f: F, g: G) -> Self::Comp<F, G>;
}

struct SymSet {}

trait BiFiber<Symbol: ?Sized>
{
    type A;
    type B;

    type C;

    fn get(self) -> Self::C;
}

// impl<A,B,X: Fn(A) -> B> BiFiber<SymSet> for X
// {

// }

trait Arrow<A,B>
{
    type C;
    fn get(self) -> Self::C;
}

impl<A,B, X: Fn(A) -> B> Arrow<A,B> for X
{
    type C = Self;

    fn get(self) -> Self::C {
        self
    }
}

// impl BiFiber<SymSet> for
/*

impl IsCategory for SymSet
{
    type Comp<F: BiFiber<Self>, G: BiFiber<Self, A=F::B>> = impl Fn(F::A) -> G::B;

    fn comp<F: BiFiber<Self>, G: BiFiber<Self, A=F::B>>(f: F, g: G) -> Self::Comp<F, G> {
        todo!()
    }
    
    type Get<F: BiFiber<Self>> = impl Fn(F::A) -> F::B;
    
    fn get<F: BiFiber<Self>>(f: F) -> Self::Get<F> {
        todo!()
    }
}

 */