use crate::scanner::token::Token;

pub struct ScopeTracker {
    locals: Vec<Local>,
    local_count: usize,
    scope_depth: i64,
}

impl ScopeTracker {
    pub fn new() -> Self {
        ScopeTracker {
            locals: Vec::new(),
            local_count: 0,
            scope_depth: 0,
        }
    }

    pub fn begin(&mut self) {
        self.scope_depth += 1;
    }

    pub fn end(&mut self) -> i32 {
        self.scope_depth -= 1;
        let mut pop_count = 0;

        while self.local_count > 0 && self.locals.last().as_ref().unwrap().depth > self.scope_depth {
            pop_count += 1;
            self.local_count -= 1;
        }

        pop_count
    }

    pub fn is_global(&self) -> bool {
        self.scope_depth == 0
    }

    pub fn add_local(&mut self, name: Token) {
        self.local_count += 1;
        self.locals.push(Local {
            name,
            depth: -1
        });
    }

    pub fn define_last(&mut self) {
        self.locals.last_mut().unwrap().depth = self.scope_depth;
    }
}

impl ScopeTracker
    {
    pub fn locate_local<F>(&self, f: F) -> i64 where F: Fn(&str) -> bool {
        for local in self.locals.iter().rev() {
            if local.depth != -1 && local.depth < self.scope_depth {
                return -1;
            }

            if f(&local.name.lexem) {
                return local.depth;
            }
        }

        -1
    }
}

pub struct Local {
    name: Token,
    depth: i64,
}
