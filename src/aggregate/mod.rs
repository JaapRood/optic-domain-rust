pub use cqrs_core::Aggregate;
use cqrs_core::AggregateEvent;

use crate::events::OpticEvent;

pub mod requests;
pub mod rfc;
pub mod shape;

// rename this to RfcState.. but then what is RfcState?
#[derive(Debug)]
pub struct OpticState<'a> {
  pub requests: &'a requests::RequestsState,
  pub rfc: &'a rfc::RfcState,
  pub shape: &'a shape::ShapeState,
}

#[derive(Default)]
pub struct OpticAggregate {
  requests: requests::RequestsAggregate,
  rfc: rfc::RfcAggregate,
  shape: shape::ShapeAggregate,
}

impl OpticAggregate {
  pub fn get_state(&self) -> OpticState {
    OpticState {
      requests: self.requests.get_state(),
      rfc: self.rfc.get_state(),
      shape: self.shape.get_state(),
    }
  }
}

impl Aggregate for OpticAggregate {
  #[inline(always)]
  fn aggregate_type() -> &'static str
  where
    Self: Sized,
  {
    "optic"
  }
}

impl AggregateEvent<OpticAggregate> for OpticEvent {
  fn apply_to(self, aggregate: &mut OpticAggregate) {
    match self {
      OpticEvent::RequestsEvent(evt) => aggregate.requests.apply(evt),
      OpticEvent::RfcEvent(evt) => aggregate.rfc.apply(evt),
      OpticEvent::ShapeEvent(evt) => aggregate.shape.apply(evt),
    }
  }
}
