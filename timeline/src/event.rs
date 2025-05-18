use time::Date;

pub struct Event {
    information: EventInformation
}

pub struct EventInformation { 
    when: Date,
    title: String,
    description: String
}