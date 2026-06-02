pub trait Convertable<T> {
    fn to(self) -> T;
}

pub trait ConvertableRef<T> {
    fn to(&self) -> T;
}

pub trait ConvertableMut<T> {
    fn to(&mut self) -> T;
}
