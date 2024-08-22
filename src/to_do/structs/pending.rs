use super::base::Base;
use super::traits::create::Create;
use super::traits::get::Get;
use super::traits::edit::Edit;
use super::traits::delete::Delete;

#[derive(Debug)]
pub struct Pending {
    pub super_struct: Base
}

impl Pending {
    pub fn new(title: &str) -> Pending {
        Pending {super_struct: Base::new(title, "pending")}
    }
}

impl Create for Pending {}
impl Get for Pending {
    fn get_title(&self) -> &str {
        &self.super_struct.title
    }
}
impl Edit for Pending {}
impl Delete for Pending {}


#[cfg(test)]
mod pending_test {
    use crate::to_do::structs::traits::create::Create;

    use super::Pending;

    #[test]
    fn new() {
        let title = "any_title_pending";
        let actual = Pending::new(title);
        assert_eq!(title, actual.super_struct.title);
        assert_eq!("pending", actual.super_struct.status);
    }

    #[test]
    fn create() {
        let title = "any_title_pending";
        let item = Pending::new(title);
        item.create(title);
    }
}