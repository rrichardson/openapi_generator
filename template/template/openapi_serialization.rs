pub trait OpenapiSerialization {
  fn serialize(self: &Self) -> Option<String>;
}

impl OpenapiSerialization for f64 {
  fn serialize(self: &Self) -> Option<String> {
    Some(format!("{:.1}", self))
  }
}

impl OpenapiSerialization for String {
  fn serialize(self: &Self) -> Option<String> {
    Some(self.to_string())
  }
}

impl<T: OpenapiSerialization> OpenapiSerialization for Option<T> {
  fn serialize(self: &Self) -> Option<String> {
    self.as_ref().map(|n| match n.serialize() {
      Some(n) => n,
      None => "".to_string(),
    })
  }
}
