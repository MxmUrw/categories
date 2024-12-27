
const fn comp<'a>(f:  &'a dyn Fn(u8) -> u8, g: &'a dyn Fn(u8) -> u8) -> impl Fn(u8) -> u8 + use<'a>
{
    |a| g(f(a))
}

pub fn myfun(v: u8) -> u8
{
    const x: &dyn Fn(u8) -> u8 = &comp(&|a| a + 1, &|b| b * 2);
    x(v)
}



