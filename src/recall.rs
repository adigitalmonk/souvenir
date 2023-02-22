/// A general purpose structure for a one-time lazy evaluation.
/// Useful when you want to declare the logic now,
/// but execute it later (and only once!)
pub struct Recall<T, TOut> {
    evaluator: T,
    value: Option<TOut>,
}

impl<T, TOut> Recall<T, TOut>
where
    T: FnMut() -> TOut,
    TOut: Clone,
{
    /// Create a new Recall, providing the lazy-evaluated function
    /// that will resolve the expected value.
    ///
    /// # Examples
    /// ```
    /// use souvenir::Recall;
    /// let key = 2;
    /// let mut doubled_once = Recall::new(|| key * 2);
    /// assert_eq!(doubled_once.value(), 4);
    /// ```
    pub fn new(evaluator: T) -> Self {
        Self {
            evaluator,
            value: None,
        }
    }

    /// Resolve the lazy-evaluated function and return the value.
    ///
    /// If the function has already resolved, the value will return immediately.
    ///
    /// # Examples
    /// ```
    /// use souvenir::Recall;
    /// let mut some_expensive_calc = Recall::new(|| {
    ///   // Some slow calculation
    ///   42
    /// });
    ///
    /// assert_eq!(some_expensive_calc.value(), 42); // Took a long time!
    /// assert_eq!(some_expensive_calc.value(), 42); // Was instant!
    /// ```
    pub fn value(&mut self) -> TOut {
        match &self.value {
            Some(value) => (*value).clone(),
            None => {
                let value = (self.evaluator)();
                self.value = Some(value.clone());
                value
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_primatives() {
        let key = 2;
        let mut doubler = Recall::new(|| key * 2);
        assert_eq!(doubler.value(), 4);
    }

    #[test]
    fn it_works_with_strings() {
        let key = "unit";
        let mut doubler = Recall::new(|| key.to_owned() + "test");
        assert_eq!(doubler.value(), "unittest");
    }

    #[test]
    fn it_only_runs_once() {
        let mut counter = 0;
        let key = 2;
        let tool = || {
            counter += 1;
            key * 2
        };

        let mut doubler = Recall::new(tool);
        assert_eq!(doubler.value(), 4); // counter == 1
        assert_eq!(doubler.value(), 4);
        assert_eq!(counter, 1);
    }

    #[test]
    fn it_works_with_structs() {
        #[derive(Hash, PartialEq, Clone, Copy)]
        struct User {
            id: u32,
        }

        let mut trigger = 0;
        let tool = || -> User {
            trigger += 1;
            User { id: 1 }
        };

        let mut one_user = Recall::new(tool);
        let user1 = one_user.value();
        let user2 = one_user.value();
        let user3 = one_user.value();
        assert_eq!(user1.id, user2.id);
        assert_eq!(user1.id, user3.id);
        assert_eq!(trigger, 1);
    }
}
