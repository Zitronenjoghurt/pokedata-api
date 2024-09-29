pub trait HasLocalizedValues {
    fn id(&self) -> i32;
    fn language_id(&self) -> i32;
    fn name(&self) -> String;
}