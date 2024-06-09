use std::marker::PhantomData;

pub struct RepositoryImpl<T> {
    _marker: PhantomData<T>,
}
