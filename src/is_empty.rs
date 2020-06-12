/// An interface for checking is value empty.
pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for str {
    fn is_empty(&self) -> bool { self.is_empty() }
}

impl IsEmpty for String {
    fn is_empty(&self) -> bool { self.is_empty() }
}

impl<T> IsEmpty for [T] {
    fn is_empty(&self) -> bool { self.is_empty() }
}

impl<T> IsEmpty for Vec<T> {
    fn is_empty(&self) -> bool { self.is_empty() }
}

impl<T> IsEmpty for &T
    where
        T: IsEmpty + ?Sized,
{
    fn is_empty(&self) -> bool { (**self).is_empty() }
}

impl<T> IsEmpty for &mut T
    where
        T: IsEmpty + ?Sized,
{
    fn is_empty(&self) -> bool { (**self).is_empty() }
}

impl<T> IsEmpty for Box<T>
    where
        T: IsEmpty + ?Sized,
{
    fn is_empty(&self) -> bool { (**self).is_empty() }
}
