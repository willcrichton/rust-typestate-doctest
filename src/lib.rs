use std::marker::PhantomData;

pub enum Either<L, R> {
  Left(L),
  Right(R),
}

mod markers {
  use super::*;

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
}

pub struct ResultSet<State>(PhantomData<State>);

impl<S> ResultSet<S> {
  pub fn is_closed() -> bool {
    panic!()
  }
}

pub mod typestates {
  use super::{ResultSet, markers::*};
  pub type ClosedResultSet = ResultSet<Closed>;
  pub type OpenResultSet<P, C, D> = ResultSet<Open<P, C, D>>;
  pub type CurrentRowResultSet<S, C, D> = OpenResultSet<CurrentRow<S>, C, D>;
  pub type ValidRowResultSet<S, C, D> = CurrentRowResultSet<ValidRow<S>, C, D>;
  pub type NotYetReadResultSet<C, D> = CurrentRowResultSet<ValidRow<NotYetRead>, C, D>;
  pub type ReadResultSet<C, D> = CurrentRowResultSet<ValidRow<Read>, C, D>;
  pub type InsertRowResultSet<S, C, D> = OpenResultSet<InsertRow<S>, C, D>;
  pub type InsertingRowResultSet<C, D> = InsertRowResultSet<Inserting, C, D>;
  pub type InsertedRowResultSet<C, D> = InsertRowResultSet<Inserted, C, D>;
}

use markers::*;
use typestates::*;

impl<P, C, D> OpenResultSet<P, C, D> {
  pub fn clear_warnings(&mut self) {
    panic!()
  }
  pub fn close(mut self) -> ClosedResultSet {
    panic!()
  }
}

impl<S, D> CurrentRowResultSet<S, Concurrency<Updatable>, D> {
  pub fn move_to_insert_row(
    mut self,
  ) -> InsertingRowResultSet<Concurrency<Updatable>, D>
  {
    panic!()
  }
}

impl<S, C, D> CurrentRowResultSet<S, C, D> {
  pub fn next(
    mut self,
  ) -> Either<
    NotYetReadResultSet<C, D>,
    ReadResultSet<C, D>,
  > {
    panic!()
  }
}
