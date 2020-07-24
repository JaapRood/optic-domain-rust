use cqrs_core::{Aggregate, AggregateEvent, Event};

use crate::events::requests::RequestsEvent;
pub use crate::state::requests::{BodyDescriptor, RequestsState};

#[derive(Default)]
pub struct RequestsAggregate {
  pub state: RequestsState,
}

impl RequestsAggregate {
  pub fn get_state(&self) -> &RequestsState {
    &self.state
  }
}

impl Aggregate for RequestsAggregate {
  #[inline(always)]
  fn aggregate_type() -> &'static str
  where
    Self: Sized,
  {
    "requests"
  }
}

impl AggregateEvent<RequestsAggregate> for RequestsEvent {
  fn apply_to(self, aggregate: &mut RequestsAggregate) {
    let state = &mut aggregate.state;

    match self {
      // Path components
      // ---------------
      RequestsEvent::PathComponentAdded(e) => {
        state.with_path_component(e.path_id, e.parent_path_id, e.name)
      }
      // Requests
      // --------
      RequestsEvent::RequestAdded(e) => state.with_request(e.request_id, e.path_id, e.http_method),

      // RequestParameters
      // -----------------
      RequestsEvent::RequestParameterAddedByPathAndMethod(e) => state
        .with_request_parameter_by_path_and_method(
          e.parameter_id,
          e.path_id,
          e.http_method,
          e.parameter_location,
          e.name,
        ),
      RequestsEvent::RequestParameterShapeSet(e) => {
        state.with_request_parameter_shape(e.parameter_id, e.parameter_descriptor)
      }
      // Responses
      // ---------
      RequestsEvent::ResponseAddedByPathAndMethod(e) => state.with_response_by_path_and_method(
        e.response_id,
        e.path_id,
        e.http_method,
        e.http_status_code,
      ),
      RequestsEvent::ResponseBodySet(e) => {
        state.with_response_body(e.response_id, BodyDescriptor::Shaped(e.body_descriptor))
      }
      _ => println!(
        "Missing application logic of '{}' event for '{}' aggregate",
        self.event_type(),
        RequestsAggregate::aggregate_type()
      ),
    }
  }
}
