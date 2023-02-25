use std::{collections::HashMap, hash::Hash};

/// A general purpose structure for memoization.
/// Useful when you will be memoization multiple data points.
pub struct Memory<T, TIn, TOut> {
    remember: T,
    values: HashMap<TIn, TOut>,
}

impl<T, TIn, TOut> Memory<T, TIn, TOut>
where
    T: FnMut(&TIn) -> TOut,
{
    /// Create a new Memory, providing a function for handling a cache miss
    ///
    /// The provided function can return a different type than it accepts.
    ///
    /// # Examples
    /// ```
    /// use souvenir::Memory;
    /// // Basic memory
    /// let mut doubler = Memory::new(|x| x * 2);
    ///
    /// // TIn vs. TOut
    /// let mut lookup = Memory::new(|x: &u32| {
    ///     format!("{}", x)
    /// });
    ///
    /// // Can also accept arguments via closure
    /// let square = 4;
    /// let mut key_resolver = Memory::new(|x| x * square);
    /// ```
    pub fn new(remember: T) -> Self {
        Self {
            remember,
            values: HashMap::new(),
        }
    }

    /// Search the memory for the previous result of this input
    ///
    /// The provided input will be passed as the input for the cache-miss function.
    ///
    /// The result of the provided function will be stored
    /// in the cache with the input value as the key.
    ///
    /// # Examples
    /// ```
    /// use souvenir::Memory;
    /// let mut doubler = Memory::new(|x| x * 2);
    /// assert_eq!(doubler.resolve(&2), 4);
    ///
    /// let mut doubler = Memory::new(|key: &String| key.to_owned() + "test");
    /// let test_input = String::from("test");
    /// assert_eq!(doubler.resolve(&test_input), "testtest");
    /// ```
    pub fn resolve(&mut self, input: &TIn) -> TOut
    where
        TIn: Clone + Eq + Hash,
        TOut: Clone,
    {
        if let Some(value) = self.values.get(input) {
            value.clone()
        } else {
            let result = (self.remember)(input);
            self.values.insert(input.clone(), result.clone());
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_primatives() {
        let mut doubler = Memory::new(|key: &u32| key * 2);
        assert_eq!(doubler.resolve(&2), 4);
    }

    #[test]
    fn it_works_with_strings() {
        let mut doubler = Memory::new(|key: &String| key.to_string() + "test");
        let test_input = String::from("test");
        assert_eq!(doubler.resolve(&test_input), "testtest");
    }

    #[test]
    fn it_only_runs_once() {
        let mut counter = 0;
        let tool = |key: &u32| {
            counter += 1;
            key * 2
        };

        let mut doubler = Memory::new(tool);
        assert_eq!(doubler.resolve(&2), 4); // counter == 1
        assert_eq!(doubler.resolve(&2), 4); // counter == 1
        assert_eq!(doubler.resolve(&3), 6); // counter == 2
        assert_eq!(counter, 2);
    }

    #[test]
    fn it_works_with_structs() {
        #[derive(Hash, PartialEq, Eq, Clone)]
        struct User {
            scores: Vec<u32>,
        }
        impl User {
            fn total_score(&self) -> u32 {
                self.scores.iter().sum()
            }
        }

        let mut trigger = 0;
        let tool = |user: &User| -> u32 {
            trigger += 1;
            user.total_score()
        };
        let test_user = User {
            scores: Vec::from([1, 2, 3]),
        };

        let mut memory = Memory::new(tool);
        let user1_score = memory.resolve(&test_user);
        let user2_score = memory.resolve(&test_user);
        assert_eq!(user1_score, 6);
        assert_eq!(user1_score, user2_score);
        assert_eq!(trigger, 1);
    }
}
