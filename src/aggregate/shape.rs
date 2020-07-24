use cqrs_core::{Aggregate, AggregateEvent, Event};

use crate::events::shape::ShapeEvent;
pub use crate::state::shape::ShapeState;

#[derive(Default)]
pub struct ShapeAggregate {
  pub state: ShapeState,
}

impl ShapeAggregate {
  pub fn get_state(&self) -> &ShapeState {
    &self.state
  }
}

impl Aggregate for ShapeAggregate {
  #[inline(always)]
  fn aggregate_type() -> &'static str
  where
    Self: Sized,
  {
    "shape"
  }
}

impl AggregateEvent<ShapeAggregate> for ShapeEvent {
  fn apply_to(self, aggregate: &mut ShapeAggregate) {
    let state = &mut aggregate.state;

    match self {
      ShapeEvent::ShapeAdded(e) => {
        state.with_shape(e.shape_id, e.base_shape_id, e.parameters, e.name)
      }
      ShapeEvent::FieldAdded(e) => {
        state.with_field(e.field_id, e.shape_id, e.name, e.shape_descriptor)
      }
      _ => println!(
        "Missing application of '{}' event for '{}' aggregate",
        self.event_type(),
        ShapeAggregate::aggregate_type()
      ),
    }
  }
}
