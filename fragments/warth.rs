// A sketch of a "Grow" loop inside a Rule model
let mut current_ctx = ctx;
let mut current_res = seed;

while let Ok((next_ctx, next_res)) = self.body.parse(current_ctx) {
    if next_ctx.offset() <= current_ctx.offset() { break; } // Stopped growing
    current_ctx = next_ctx;
    current_res = next_res;
}
