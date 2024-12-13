
trait Trait
{
    type T<X>;

    fn comp<Head: FnOnce(u8) -> u8>(head: Head, tail: Self::T<u8>) -> Self::T<Head>;
}

// struct SymCompU8 {}

// impl Trait for SymCompU8
// {
//     type T<X> = impl FnOnce(u8) -> u8;
    
//     fn comp<Head: FnOnce(u8) -> u8>(head: Head, tail: Self::T<u8>) -> Self::T<Head> {
//         |a| head(tail(a))
//     }

// }



const fn run(a: u8, b: u8) -> u8 {
    a + b
}

// const fn unpack(a: Box<u8>) -> u8 {
//     *a
// }
