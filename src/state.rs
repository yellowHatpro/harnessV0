#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub status: TaskStatus,
    pub messages: Vec<Message>,
}

#[derive(Debug)]
pub struct Message {
    pub role: String,
    pub content: String
}

#[derive(Debug)]
pub enum TaskStatus {
    Running,
    Completed,
    Failed,
}

