use actix::Message;


#[derive(Message)]
#[rtype(result = "()")]
pub struct NewGame {
    pub teste: String
}