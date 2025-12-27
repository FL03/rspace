use rspace_derive::Spatial;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Spatial)]
pub struct A<T>(T);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Spatial)]
pub struct B<T> {
    pub value: T,
}

#[test]
fn test_derive_tuple_spatial() {
    let mut a: A<usize> = A::new(1).map(|x| x + 100);
    assert_eq!(a.get_mut(), &mut 101);
    a.set(<usize>::MAX);
    assert_eq!(a.get(), &<usize>::MAX);
}

#[test]
fn test_derive_struct_spatial() {
    let mut b: B<isize> = B::new(1).map(|x| x + 100);
    assert_eq!(b.get_mut(), &mut 101);
    b.set(<isize>::MAX);
    assert_eq!(b.get(), &<isize>::MAX);
}
