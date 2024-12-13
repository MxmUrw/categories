use crate::{declare_many, define};
use crate::*;


trait Monoid<X : PartialEq + Copy>
{
    declare_many!
    {
        [ mult (a : X) (b: X) : X = () ]
        [ unit : X = () ]
    }

    // laws
    define_many!
    {
        [ unit_left (a : X) : bool
        = Self::mult(Self::unit(), a) == a 
        ]

        [ unit_right (a : X) : bool
        = Self::mult(a, Self::unit()) == a 
        ]

        [ assoc (a : X) (b : X) (c : X) : bool
        = Self::mult(Self::mult(a, b), c) == Self::mult(a, Self::mult(b, c)) 
        ]
    }
}

struct SymMonoid {}

impl Monoid<bool> for SymMonoid
{
    define_many!
    {
        [ mult (a : bool) (b : bool) : bool = a && b ]
        [ unit : bool = true ]
    }
}


struct FinMon<const size: u64> {}

impl<const size: u64> Monoid<u64> for FinMon<size>
{
    define_many!
    {
        [ mult (a : u64) (b : u64) : u64
        = { let res = a + b; if res >= size { res - size } else { res } }
        ]

        [ unit : u64 = 0 ]
    }
}

/*

#[cfg(kani)]
#[kani::proof]
fn test_monoid()
{
    const size: u64 = 11u64;

    let x = kani::any();
    kani::assume(x < size);

    let y = kani::any();
    kani::assume(y < size);

    assert!(FinMon::<11>::mult(x, y) < size);
}

#[cfg(kani)]
#[kani::proof]
fn test_monoid_laws() {

    let x = kani::any();
    let y = kani::any();
    let z = kani::any();
    kani::assume(x < 11 && y < 11 && z < 11);

    assert!(FinMon::<11>::assoc(x, y, z))
}

 */


#[test]
fn kani1() {
    // assert!(FinMon::<11>::assoc(0, 4, 18446744073709551614))
    assert!(FinMon::<11>::assoc(3, 7, 31))
}

// Test generated for harness `experimental::laws::test_monoid_laws`
//
// Check for `assertion`: "assertion failed: FinMon::<11>::assoc(x, y, z)"

// #[test]
// fn kani_concrete_playback_test_monoid_laws_5117245478506141473() {
//     let concrete_vals: Vec<Vec<u8>> = vec![
//         // 3ul
//         vec![3, 0, 0, 0, 0, 0, 0, 0],
//         // 7ul
//         vec![7, 0, 0, 0, 0, 0, 0, 0],
//         // 31ul
//         vec![31, 0, 0, 0, 0, 0, 0, 0],
//     ];
//     kani::concrete_playback_run(concrete_vals, test_monoid_laws);
// }

// Test generated for harness `experimental::laws::test_monoid`
//
// Check for `assertion`: "assertion failed: FinMon::<11>::unit_left(x)"

// #[test]
// fn kani_concrete_playback_test_monoid_12772192104838113962() {
//     let concrete_vals: Vec<Vec<u8>> = vec![
//         // 18446744073709551604ul
//         vec![244, 255, 255, 255, 255, 255, 255, 255],
//     ];
//     kani::concrete_playback_run(concrete_vals, test_monoid);
// }
