use std::marker::PhantomData;

pub enum Either<L, R> {
  Left(L),
  Right(R),
}

pub struct NotYetRead;
pub struct Read;
pub struct InvalidRow;
pub struct ValidRow<State>(PhantomData<State>);
pub struct CurrentRow<State>(PhantomData<State>);

pub struct Inserting;
pub struct Inserted;
pub struct InsertRow<State>(PhantomData<State>);

pub struct ReadOnly;
pub struct Updatable;
pub struct Concurrency<State>(PhantomData<State>);

pub struct Scrollable;
pub struct ForwardOnly;
pub struct Direction<State>(PhantomData<State>);

pub struct Closed;
pub struct Open<Position, Concurrency, Direction>(
  PhantomData<(Position, Concurrency, Direction)>,
);
pub struct ResultSet<State>(PhantomData<State>);

impl<S> ResultSet<S> {
  pub fn is_closed() -> bool {
    panic!()
  }
}

impl<P, C, D> ResultSet<Open<P, C, D>> {
  pub fn clear_warnings(&mut self) {
    panic!()
  }
  pub fn close(mut self) -> ResultSet<Closed> {
    panic!()
  }
}

impl<S, D> ResultSet<Open<CurrentRow<S>, Concurrency<Updatable>, D>> {
  pub fn move_to_insert_row(
    mut self,
  ) -> ResultSet<
    Open<CurrentRow<InsertRow<Inserting>>, Concurrency<Updatable>, D>,
  > {
    panic!()
  }
}

impl<S, C, D> ResultSet<Open<CurrentRow<S>, C, D>> {
  pub fn next(
    mut self,
  ) -> Either<
    ResultSet<Open<CurrentRow<ValidRow<NotYetRead>>, C, D>>,
    ResultSet<Open<CurrentRow<InvalidRow>, C, D>>,
  > {
    panic!()
  }
}
