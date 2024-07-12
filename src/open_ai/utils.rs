use serde::Serializer;

use super::model::Model;

pub fn is_none_or_empty<T: AsRef<[U]>, U>(opt: &Option<T>) -> bool {
    opt.as_ref().map_or(true, |v| v.as_ref().is_empty())
}

pub fn serialize_model<S>(model: &Model, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match model {
        Model::Custom { name, .. } => serializer.serialize_str(name),
        _ => serializer.serialize_str(model.id()),
    }
}
