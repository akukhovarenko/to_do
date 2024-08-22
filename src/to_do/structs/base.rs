#[derive(Debug)]
pub struct Base {
    pub title: String,
    pub status: String,
}

impl Base {
    pub fn new(title: &str, status: &str) -> Base {
        Base {title: title.to_string(), status: status.to_string()}
    }
    pub fn print(&self) -> () {
        println!("{} -> {}", self.title, self.status)
    }
}


#[cfg(test)]
mod base_test {
    use super::Base;
    #[test]
    fn new(){
        let title = String::from("any_title");
        let status = String::from("any_status"); 
        let actual = Base::new("any_title", "any_status");
        assert_eq!(title, actual.title);
        assert_eq!(status, actual.status);
    }
}