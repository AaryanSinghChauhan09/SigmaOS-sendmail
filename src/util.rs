//! sendmail model — mail transfer agent.

#[derive(Debug, Clone)]
pub struct Mail {
    pub id: String,
    pub from: String,
    pub to: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub mails: Vec<Mail>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { mails: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn send(&mut self, from: &str, to: Vec<String>) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.mails.push(Mail { id: id.clone(), from: from.into(), to, status: "queued".into() });
        id
    }

    pub fn queue(&self) -> &[Mail] { &self.mails }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn send_queue() {
        let mut s = Model::new();
        let id = s.send("alice@x.com", vec!["bob@x.com".into()]);
        assert_eq!(s.queue()[0].id, id);
        assert_eq!(s.queue()[0].status, "queued");
    }
}
