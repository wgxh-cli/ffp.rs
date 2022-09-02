use super::wrapper::wrap;

pub trait Category<'a, T: 'a> {
  fn value(&self) -> T;
  fn wrap(value: T) -> Self
    where Self: Sized;
}

pub trait Functor<'a, A: 'a>: Category<'a, A> + 'a {
  fn fmap<AC, B: 'a, BC, F>(mapping: F, a: AC) -> BC
    where
      AC: Category<'a, A> + 'a,
      BC: Category<'a, B> + 'a,
      F: Fn(A) -> B + 'a
  {
    wrap(mapping).map(|b| BC::wrap(b)).call(a.value())
  }
}

pub trait Monad<'a, A: 'a> {
  fn bind<AC, B, BC, F>(mapping: F, a: AC) -> BC
    where
      B: 'a,
      AC: Category<'a, A> + 'a,
      BC: Category<'a, B> + 'a,
      F: Fn(A) -> BC + 'a
  {
    wrap(mapping).call(a.value())
  }

  fn bind_curried<AC, B, BC, F>(mapping: &'a F) -> Box<dyn Fn(AC) -> BC + 'a>
    where
      B: 'a,
      AC: Category<'a, A> + 'a,
      BC: Category<'a, B> + 'a,
      F: Fn(A) -> BC + 'a
  {
    Box::new(move |a| {
      wrap(mapping).call(a.value())
    })
  }
}
