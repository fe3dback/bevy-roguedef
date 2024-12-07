#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum EProfUIState {
    Nothing,
    #[default]
    Mini,
    Full,
}