use tranche::BasedTranche;

fn do_test<T>(slice: &impl AsRef<[T]>) {
    let mut tranche = BasedTranche::new(slice);
    assert_eq!(tranche.offset(), 0);

    tranche.take_first().unwrap();
    assert_eq!(tranche.offset(), 1);

    tranche.take_front(2).unwrap();
    assert_eq!(tranche.offset(), 3);
}

#[test]
fn test_bytes() {
    do_test(&[1u8, 2, 3, 4, 5, 6]);
}

#[test]
fn test_words() {
    do_test(&[1usize, 2, 3, 4, 5, 6]);
}

#[test]
fn test_units() {
    do_test(&[(), (), (), (), (), ()]);
}
