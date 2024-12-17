use std::collections::HashMap;
use std::hash::Hash;


pub struct Memoizer<TInput, TOutput>
where
    TInput: Hash + Eq + Clone,
{
    memoized_results: HashMap<TInput, TOutput>,
}

impl<TInput, TOutput> Memoizer<TInput, TOutput>
where
    TInput: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Self {
            memoized_results: HashMap::new(),
        }
    }
    pub fn calculate<FnCalc>(&mut self, calc: FnCalc, input: TInput) -> &TOutput
    where
        FnCalc: Fn(&mut Self, &TInput) -> TOutput,
    {
        if !self.memoized_results.contains_key(&input) {
            let res = calc(self, &input);
            self.memoized_results.insert(input.clone(), res);
        }

        return self.memoized_results.get(&input).unwrap();
    }

    pub fn reset(&mut self) {
        self.memoized_results.clear();
    }
}
