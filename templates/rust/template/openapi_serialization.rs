pub trait OpenapiSerialization {
  fn serialize(&self) -> Option<String>;
}

impl OpenapiSerialization for i32 {
  fn serialize(&self) -> Option<String> {
    Some(format!("{:?}", self))
  }
}

impl OpenapiSerialization for i64 {
  fn serialize(&self) -> Option<String> {
    Some(format!("{:?}", self))
  }
}

impl OpenapiSerialization for f32 {
  fn serialize(&self) -> Option<String> {
    Some(format!("{:?}", self))
  }
}

impl OpenapiSerialization for f64 {
  fn serialize(&self) -> Option<String> {
    Some(format!("{:?}", self))
  }
}

impl OpenapiSerialization for String {
  fn serialize(&self) -> Option<String> {
    Some(self.clone())
  }
}

impl<T: OpenapiSerialization> OpenapiSerialization for Option<T> {
  fn serialize(&self) -> Option<String> {
    self.as_ref().map(|n| match n.serialize() {
      Some(n) => n,
      None => "".to_string(),
    })
  }
}
