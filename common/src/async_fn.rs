use std::future::Future;

/**
source: <https://stackoverflow.com/questions/70503578/lifetime-bound-in-async-function-which-is-also-an-argument>
 */
pub trait AsyncFn<T>: FnOnce(T) -> <Self as AsyncFn<T>>::Fut {
    type Fut: Future<Output = <Self as AsyncFn<T>>::Output>;
    type Output;
}

impl<T, F, Fut> AsyncFn<T> for F
where
    F: FnOnce(T) -> Fut,
    Fut: Future,
{
    type Fut = Fut;
    type Output = Fut::Output;
}
