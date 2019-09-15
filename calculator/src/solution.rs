use main::Operation::{self, *};
use main::CalcError::{self, *};

pub fn press_operator(&mut self, operation: &Operation) -> Result<i64, CalcError> {
    if let Some(x) = self.stack.pop() {
        if let Some(y) = self.stack.pop() {
            let result = match operation {
                Add => y + x,
                Mul => y * x,
                Sub => y - x,
                Div => {
                    if x != 0 {
                        y / x
                    } else {
                        // Restore the state
                        self.stack.push(y);
                        self.stack.push(x);
                        return Err(DivideByZero)
                    }
                },
            };
            self.stack.push(result);
            Ok(result)
        } else {
            // If second arg isn't there, restore first arg
            self.stack.push(x);
            Err(StackTooSmall)
        }
    } else {
        Err(StackTooSmall)
    }
}
