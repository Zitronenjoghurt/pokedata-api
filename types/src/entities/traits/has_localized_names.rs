pub trait HasLocalizedNames {
    fn id(&self) -> u32;
    fn language_id(&self) -> u32;
    fn name(&self) -> String;
}