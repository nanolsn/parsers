pub trait Concat<L, R>
    where
        Self: Sized,
{
    fn empty() -> Self;

    fn concat(l: L, r: R) -> Self;
}

impl<L, R> Concat<L, R> for String
    where
        L: Into<String>,
        R: AsRef<str>,
{
    fn empty() -> Self { String::new() }

    fn concat(l: L, r: R) -> Self {
        let mut res = l.into();
        res.push_str(r.as_ref());
        res
    }
}

impl<'l, 'r: 'l> Concat<&'l str, &'r str> for &'l str {
    fn empty() -> Self { "" }

    fn concat(l: &'l str, r: &'r str) -> Self {
        if l.len() == 0 { return r }

        if unsafe { l.as_ptr().offset(l.len() as isize) } != r.as_ptr() {
            panic!("The trying to concat not adjacent string slices!")
        }

        let len = l.len() + r.len();

        unsafe {
            let slice: &[u8] = std::slice::from_raw_parts(l.as_ptr(), len);
            std::str::from_utf8_unchecked(slice)
        }
    }
}

impl<T> Concat<Vec<T>, T> for Vec<T> {
    fn empty() -> Self { Vec::new() }

    fn concat(mut l: Vec<T>, r: T) -> Self {
        l.push(r);
        l
    }
}
