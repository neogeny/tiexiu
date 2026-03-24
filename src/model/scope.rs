use super::ctx::Ctx;
// In state_scope.rs

pub struct StateScope<'a, 'b> {
    // `'a` is the lifetime of the input string inside Ctx
    // `'b` is the lifetime of this specific borrow of Ctx
    pub ctx: &'b mut Ctx<'a>,
}

impl<'a, 'b> StateScope<'a, 'b> {
    pub fn new(ctx: &'b mut Ctx<'a>) -> Self {
        ctx.state_push();
        Self { ctx }
    }

    pub fn merge(self) {
        self.ctx.state_merge();
        std::mem::forget(self);
    }

    pub fn pop(self) {
        self.ctx.state_pop();
        std::mem::forget(self);
    }
}

impl<'a, 'b> Drop for StateScope<'a, 'b> {
    fn drop(&mut self) {
        self.ctx.state_undo();
    }
}
