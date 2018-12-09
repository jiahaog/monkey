#[derive(PartialEq, Debug)]
pub enum Object {
    Null,
    Boolean(bool),
    Integer(usize),
}
