

pub fn mytest(mut a: Vec<u8>, mut b: Vec<u8>) -> Vec<u8>
{
    a.append(&mut b);
    a
}


#[cfg(kani)]
#[kani::proof]
#[kani::unwind(10)]
fn test_monoid_laws() {

    // const xl: usize = kani::any();
    let mut xs : Vec<u8> = kani::vec::any_vec::<_, 2>();
    // let y : Vec<u8> = kani::vec::any_vec::<_, 2>();
    // kani::assume(x < 11 && y < 11 && z < 11);

    xs.sort();

    assert!(xs[0] <= xs[1]);
}

