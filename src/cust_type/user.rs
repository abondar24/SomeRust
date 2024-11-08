#[derive(Debug)]
pub struct User {
    pub(super) name: String,
    pub(super) pass: String,
}

impl User {
    pub fn pass_enc(&self) -> String {
        let mut enc = String::from("enc");
        enc.push_str(&*self.pass);
        enc
    }

    pub fn new_user() -> Self {
        Self {
            name: String::from("new"),
            pass: String::from("pass"),
        }
    }
}