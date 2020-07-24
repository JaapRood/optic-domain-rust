use super::shape::ShapeId;
use std::collections::hash_map::HashMap;
// use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct RequestsState {
  path_components: HashMap<PathComponentId, PathComponent>,
  parent_path: HashMap<PathComponentId, PathComponentId>,
  request_parameters: HashMap<RequestParameterId, HttpRequestParameter>,
  requests: HashMap<RequestId, HttpRequest>,
  responses: HashMap<ResponseId, HttpResponse>,
}

pub type PathComponentId = String;
pub type RequestId = String;
pub type RequestParameterId = String;
pub type ResponseId = String;

#[derive(Debug)]
struct PathComponent {
  path_id: PathComponentId,
  descriptor: PathComponentDescriptor,
  is_removed: bool,
}

#[derive(Debug)]
struct PathComponentDescriptor {
  parent_path_id: PathComponentId,
  name: String,
}

#[derive(Debug)]
pub struct HttpRequest {
  pub request_id: RequestId,
  pub request_descriptor: RequestDescriptor,
  pub is_removed: bool,
}

#[derive(Debug)]
struct HttpRequestParameter {
  parameter_id: RequestParameterId,
  request_parameter_descriptor: RequestParameterDescriptor,
  is_removed: bool,
}

#[derive(Debug)]
pub struct HttpResponse {
  pub response_id: ResponseId,
  pub response_descriptor: ResponseDescriptor,
  pub is_removed: bool,
}

#[derive(Debug)]
pub struct RequestDescriptor {
  pub path_component_id: PathComponentId,
  pub http_method: String,
  pub body_descriptor: BodyDescriptor,
}

#[derive(Debug)]
pub struct ResponseDescriptor {
  pub path_id: PathComponentId,
  pub http_method: String,
  pub http_status_code: u16,
  pub body_descriptor: BodyDescriptor,
}

#[derive(Debug)]
pub enum BodyDescriptor {
  Unset,
  Shaped(ShapedBodyDescriptor),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapedBodyDescriptor {
  http_content_type: String,
  shape_id: ShapeId,
  is_removed: bool,
}

#[derive(Debug)]
pub struct RequestParameterDescriptor {
  path_id: RequestId,
  http_method: String,
  location: String,
  name: String,
  shape_descriptor: RequestParameterShapeDescriptor, // bodyDescriptor: BodyDescriptor
}

#[derive(Debug)]
enum RequestParameterShapeDescriptor {
  Unset,
  Shaped(ShapedRequestParameterShapeDescriptor),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapedRequestParameterShapeDescriptor {
  shape_id: ShapeId,
  is_removed: bool,
}

impl RequestsState {
  pub fn all_requests(&self) -> impl Iterator<Item = &HttpRequest> {
    self.requests.values()
  }

  pub fn all_responses(&self) -> impl Iterator<Item = &HttpResponse> {
    self.responses.values()
  }

  // Path components
  // ---------------
  pub fn with_path_component(
    &mut self,
    path_id: PathComponentId,
    parent_path_id: PathComponentId,
    name: String,
  ) {
    self.path_components.insert(
      path_id.clone(),
      PathComponent {
        path_id: path_id.clone(),
        descriptor: PathComponentDescriptor {
          parent_path_id,
          name,
        },
        is_removed: false,
      },
    );
  }

  // Requests
  // --------
  pub fn with_request(
    &mut self,
    request_id: RequestId,
    path_id: PathComponentId,
    http_method: String,
  ) {
    self.requests.insert(
      request_id.clone(),
      HttpRequest {
        request_id: request_id.clone(),
        request_descriptor: RequestDescriptor {
          path_component_id: path_id,
          http_method,
          body_descriptor: BodyDescriptor::Unset,
        },
        is_removed: false,
      },
    );
  }

  // Request parameters
  // ------------------

  pub fn with_request_parameter_by_path_and_method(
    &mut self,
    parameter_id: RequestParameterId,
    path_id: PathComponentId,
    http_method: String,
    parameter_location: String,
    name: String,
  ) {
    self.request_parameters.insert(
      parameter_id.clone(),
      HttpRequestParameter {
        parameter_id: parameter_id.clone(),
        request_parameter_descriptor: RequestParameterDescriptor {
          path_id,
          http_method,
          location: parameter_location,
          name,
          shape_descriptor: RequestParameterShapeDescriptor::Unset {},
        },
        is_removed: false,
      },
    );
  }

  pub fn with_request_parameter_shape(
    &mut self,
    parameter_id: RequestParameterId,
    parameter_shape_descriptor: ShapedRequestParameterShapeDescriptor,
  ) {
    let parameter = self
      .request_parameters
      .get_mut(&parameter_id)
      .expect("request parameter must exist to setup parameter descriptor");
    let existing_descriptor = &mut parameter.request_parameter_descriptor;
    existing_descriptor.shape_descriptor =
      RequestParameterShapeDescriptor::Shaped(parameter_shape_descriptor);
  }

  // Responses
  // ---------

  pub fn with_response_by_path_and_method(
    &mut self,
    response_id: ResponseId,
    path_id: PathComponentId,
    http_method: String,
    http_status_code: u16,
  ) {
    self.responses.insert(
      response_id.clone(),
      HttpResponse {
        response_id: response_id.clone(),
        response_descriptor: ResponseDescriptor {
          path_id,
          http_method,
          http_status_code,
          body_descriptor: BodyDescriptor::Unset,
        },
        is_removed: false,
      },
    );
  }

  pub fn with_response_body(&mut self, response_id: ResponseId, body_descriptor: BodyDescriptor) {
    let response = self
      .responses
      .get_mut(&response_id)
      .expect("response must exist to set body for it");
    let response_descriptor = &mut response.response_descriptor;
    response_descriptor.body_descriptor = body_descriptor;
  }
}
