


pub(crate) fn post_test() {
    let mut post = Post::new();
    post.add_text("Сегодня на обед я ел салат");
    assert_eq!("", post.content());
    
    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Сегодня на обед я ел салат", post.content())

}

struct Post{
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post{
        Post{
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str){
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str{
        ""
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take(){
            self.state = Some(s.request_review())
        }
    }
}

trait State{
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft{}

impl State for Draft{
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }
}

struct PendingReview {}
impl State for PendingReview{
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}