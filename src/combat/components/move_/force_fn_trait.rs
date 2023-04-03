use crate::combat::{ForceFnInput, ForceFnOutput};
use std::fmt::Debug;

impl Debug for dyn ForceFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TranslationFn").finish()
    }
}

pub trait ForceFn: Send + Sync {
    fn call(&self, input: ForceFnInput) -> ForceFnOutput;
    fn clone_box<'a>(&self) -> Box<dyn ForceFn + 'a>
    where
        Self: 'a;
}
impl<F> ForceFn for F
where
    F: Fn(ForceFnInput) -> ForceFnOutput + Send + Sync + Clone,
{
    fn call(&self, input: ForceFnInput) -> ForceFnOutput {
        self(input)
    }

    fn clone_box<'a>(&self) -> Box<dyn ForceFn + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn ForceFn + 'a> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}
