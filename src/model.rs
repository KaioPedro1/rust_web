use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;


pub struct NewRegistration {
    pub name: UserName,
    pub id: Uuid
}
pub struct UserName(String);
impl UserName {
    pub fn parse(s:String)->Result<UserName, String>{
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 50;
        let forbidden_characters=['/','(',')','"','<','>','\\','{','}'];
        let contains_forbidden_characters=s.chars().any(|g|forbidden_characters.contains(&g));

        if is_empty_or_whitespace||is_too_long||contains_forbidden_characters{
            Err(format!("{} is not a valid subscriber name.",s))
        }else{
            Ok(Self(s))
        }
    }
}
impl AsRef<str> for UserName{
    fn as_ref(&self)->&str{
        &self.0
    }
}


