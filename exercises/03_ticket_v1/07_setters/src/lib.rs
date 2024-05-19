// TODO: Add &mut-setters to the `Ticket` struct for each of its fields.
//   Make sure to enforce the same validation rules you have in `Ticket::new`!
//   Even better, extract that logic into private methods and reuse it in both places.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket { 
        Ticket::validate_emptyness("Title", &title); 
        Ticket::validate_emptyness("Description", &description);
        Ticket::validate_length("Title", &title);
        Ticket::validate_length("Description", &description);
        Ticket::validate_status(&status);
        Ticket {
            title,
            description,
            status,
        }
    }
    pub fn validate_emptyness(field:&str, value:&String) -> bool {
        if value.is_empty() {
            panic!("{} cannot be empty", field);
        }
        true
    }
    pub fn validate_length(field:&str, value:&String) -> bool {
        if field == "Title" {
            if value.len() > 50 {
               panic!("Title cannot be longer than 50 characters"); 
            }
        } else if field == "Description" {
            if value.len() > 500 {
                panic!("Description cannot be longer than 500 characters"); 
            }
        } else {
            panic!("Invalid field!!");
        }
        true
    }
    pub fn validate_status(value:&String) -> bool {
        if value != "To-Do" && value != "In Progress" && value != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }
        true
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
    pub fn set_title(&mut self, new_title:String) {
        if Ticket::validate_emptyness("Title", &new_title) && Ticket::validate_length("Title", &new_title) {
            self.title = new_title;
        }  
    }
    pub fn set_description(&mut self, new_description:String) {
        if Ticket::validate_emptyness("Description", &new_description) && Ticket::validate_length("Description", &new_description) {
            self.description = new_description;
        }
    }
    pub fn set_status(&mut self, new_status:String) {
        if Ticket::validate_status(&new_status) {
            self.status = new_status;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Ticket;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn works() {
        let mut ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        ticket.set_title("A new title".into());
        ticket.set_description("A new description".into());
        ticket.set_status("Done".into());

        assert_eq!(ticket.title(), "A new title");
        assert_eq!(ticket.description(), "A new description");
        assert_eq!(ticket.status(), "Done");
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title("".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description("".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 characters")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_title(overly_long_title())
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 characters")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_description(overly_long_description())
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_status("Funny".into());
    }
}
