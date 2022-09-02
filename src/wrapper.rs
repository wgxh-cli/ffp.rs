pub struct Wrapper<'a, I: 'a, O: 'a> {
  wrapped_fn: Box<dyn Fn(I) -> O + 'a>,
}

impl<'a, I, O> Wrapper<'a, I, O> {
  pub fn new(fun: impl Fn(I) -> O + 'a) -> Self {
    Wrapper {
      wrapped_fn: Box::new(fun),
    }
  }

  pub fn map<NO>(self, map_fn: impl Fn(O) -> NO + 'a) -> Wrapper<'a, I, NO> {
    Wrapper::new(move |x| {
      map_fn((self.wrapped_fn)(x))
    })
  }

  pub fn call(&self, input: I) -> O {
    (self.wrapped_fn)(input)
  }

  pub fn unwrap(self) -> impl Fn(I) -> O + 'a {
    self.wrapped_fn
  }
}

pub fn wrap<'a, I, O>(fun: impl Fn(I) -> O + 'a) -> Wrapper<'a, I, O> {
  Wrapper::new(fun)
}

pub type ReaderFn<'a, T: 'a + Clone> = impl Fn() -> T;
pub fn reader<'a, T: 'a + Clone>(x: T) -> ReaderFn<'a, T> {
  move || { x.clone() }
}
