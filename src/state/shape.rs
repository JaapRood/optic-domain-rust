use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct ShapeState {
  shapes: HashMap<ShapeId, ShapeEntity>,
  fields: HashMap<FieldId, FieldEntity>,
}

pub type ShapeId = String;
pub type FieldId = String;
type ShapeParameterId = String;

#[derive(Debug)]
struct ShapeValue {
  is_user_defined: bool,
  base_shape_id: ShapeId,
  parameters: ShapeParametersDescriptor,
  field_ordering: Vec<FieldId>,
  name: String,
}

#[derive(Debug)]
struct ShapeEntity {
  shape_id: ShapeId,
  descriptor: ShapeValue,
  is_removed: bool,
}

#[derive(Debug)]
struct FieldEntity {
  field_id: FieldId,
  descriptor: FieldValue,
  is_removed: bool,
}

#[derive(Debug)]
struct FieldValue {
  shape_id: ShapeId,
  shape_descriptor: FieldShapeDescriptor,
  name: String,
}

#[derive(Debug, Deserialize)]
pub enum ShapeParametersDescriptor {
  NoParameterList,
  StaticParameterList(StaticShapeParametersDescriptor),
  DynamicParameterList(DynamicShapeParametersDescriptor),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticShapeParametersDescriptor {
  shape_parameter_ids: Vec<ShapeParameterId>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicShapeParametersDescriptor {
  shape_parameter_ids: Vec<ShapeParameterId>,
}

#[derive(Debug, Deserialize)]
pub enum FieldShapeDescriptor {
  FieldShapeFromShape(FieldShapeFromShape),
  FieldShapeFromParameter(FieldShapeFromParameter),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldShapeFromShape {
  field_id: FieldId,
  shape_id: ShapeId,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldShapeFromParameter {
  field_id: FieldId,
  shape_parameter_id: ShapeParameterId,
}

impl ShapeState {
  pub fn with_shape(
    &mut self,
    shape_id: ShapeId,
    assigned_shape_id: ShapeId,
    parameters: ShapeParametersDescriptor,
    name: String,
  ) {
    self.shapes.insert(
      shape_id.clone(),
      ShapeEntity {
        shape_id: shape_id.clone(),
        descriptor: ShapeValue {
          is_user_defined: true,
          base_shape_id: assigned_shape_id,
          parameters,
          name,
          field_ordering: vec![],
        },
        is_removed: false,
      },
    );
  }

  pub fn with_field(
    &mut self,
    field_id: FieldId,
    shape_id: ShapeId,
    name: String,
    shape_descriptor: FieldShapeDescriptor,
  ) {
    let shape = self
      .shapes
      .get_mut(&shape_id)
      .expect("shape must exist to add field for it");

    shape.with_appended_field_id(field_id.clone());
    self.fields.insert(
      field_id.clone(),
      FieldEntity {
        field_id: field_id.clone(),
        descriptor: FieldValue {
          shape_id,
          shape_descriptor,
          name,
        },
        is_removed: false,
      },
    );
  }
}

impl ShapeEntity {
  pub fn with_appended_field_id(&mut self, field_id: FieldId) {
    self.descriptor.field_ordering.push(field_id);
  }
}
