use cqrs_core::{Aggregate, AggregateEvent, Event};

use crate::events::rfc::RfcEvent;

#[derive(Default, Debug)]
pub struct RfcState {}

#[derive(Default)]
pub struct RfcAggregate {
  pub state: RfcState,
}

impl RfcAggregate {
  pub fn get_state(&self) -> &RfcState {
    &self.state
  }
}

impl Aggregate for RfcAggregate {
  #[inline(always)]
  fn aggregate_type() -> &'static str
  where
    Self: Sized,
  {
    "rfc"
  }
}

impl AggregateEvent<RfcAggregate> for RfcEvent {
  fn apply_to(self, aggregate: &mut RfcAggregate) {
    // let state = &mut aggregate.state;

    match self {
      _ => println!(
        "Missing application logic of '{}' event for '{}' aggregate",
        self.event_type(),
        RfcAggregate::aggregate_type()
      ),
    }
  }
}
