




// the problem is that when i create an impl for a "category" trait,
// then I cannot define the underlying impl type by having a function create : X -> X
// because I cannot bound the X to be the impl type itself.
//
// I want to allow the associated type of a trait contain arbitrary compositions of functions
// this means that I need a compositor trait which fulfills this purpose
//
// now the next thing is that if i have
//
// type Compose<A,B> = impl Monoid
//
// then I want A and B to be Monoids as well. But for that I require two type parameters.
//
// Thus I need at least a generic Compose...
//
// A compose has to take an arbitrary list of types.
//
// What I need is an existential type which hides the input data of the impl

const fn comb(xs: &[u8]) -> u8 {
    let mut y = 0;
    // for x in xs {
    //     y += x
    // }
    y
}


