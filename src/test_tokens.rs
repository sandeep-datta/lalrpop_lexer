#[derive(Debug, Clone)]
pub enum TestToken<'input> {
    Integer(&'input str),
}
