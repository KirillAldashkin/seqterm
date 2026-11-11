use termseq::{NullTerm, Seq, SeqMut};

#[test]
fn create() {
    let buf1 = [4, 3, 2, 1];

    let buf2 = [4, 3, 2, 1, 0];
    let buf2 = unsafe { Seq::<i32, NullTerm>::from_ptr(buf2.as_ptr()) };

    assert!(buf1.iter().eq(buf2));
}

#[test]
fn square() {
    let mut buf = [3, 2, 1, 0];
    for e in unsafe { SeqMut::<i32, NullTerm>::from_ptr(buf.as_mut_ptr()) } {
        *e *= *e;
    }
    assert_eq!(buf, [9, 4, 1, 0]);
}
