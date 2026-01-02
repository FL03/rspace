/*
    Appellation: ops <module>
    Created At: 2026.01.01:21:49:55
    Contrib: @FL03
*/
use rspace_traits::Apply;

fn add<A, B, C>(a: A, b: B) -> C
where
    A: core::ops::Add<B, Output = C>,
{
    core::ops::Add::add(a, b)
}

#[test]
fn test_apply() {
    let something = Some(19u8);
    let result = something.apply(|&x| {
        let new_x = x as f64;
        add(new_x, 3.0f64)
    });
    assert_eq! {result, Some(22.0f64)};
}
