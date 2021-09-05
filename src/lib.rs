//! Utility for finding the minimum and maximum values of a collection.
//! Similar to C++'s [minmax_element](https://en.cppreference.com/w/cpp/algorithm/minmax_element),
//! Julia's [extrema](http://www.jlhub.com/julia/manual/en/function/extrema) and Ruby's [minmax](https://apidock.com/ruby/v2_5_5/Enumerable/minmax).
//!
//! # Quick Start:
//! Just call it on any iterable collection!
//! ```
//! let my_set: HashSet<i32> = (1..300).collect();
//! let (min, max) = my_set.iter().extrema();
//!
//! assert_eq!((min, max), (&1, &300));
//! ```

/// By implementing `Extrema` on a collection you will be able to
/// find its minimum and maximum elements.
pub trait Extrema {
    /// The type of the collection's element
    type Item: Ord;
    /// Finds, if they exists, the minimum and maximum elements
    /// in a single pass.
    /// # Examples:
    ///
    /// Basic usage:
    /// ```
    /// let xs = vec![0, 1, 2, 3];
    /// let minmax = xs.extrema();
    ///
    /// assert_eq!(minmax, Some((0, 3)));
    /// ```
    /// `extrema` consumes the collection; to avoid this call
    /// it on a borrowed value like so:
    /// ```
    /// let xs = vec![0, 1, 2, 3];
    /// let minmax = (&xs).extrema(); // or xs[..].extrema(), xs.iter().extrema()  
    ///
    /// assert_eq!(minmax, Some((&0, &3)));
    /// ```
    fn extrema(self) -> Option<(Self::Item, Self::Item)>;
}

impl<C, T> Extrema for C
where
    C: IntoIterator<Item = T>,
    T: Ord,
{
    type Item = T;
    fn extrema(self) -> Option<(Self::Item, Self::Item)> {
        let mut iter = self.into_iter();
        let first = iter.next();
        let second = iter.next();

        // Edge case: collections with less than 2 items have no extremes
        if first.is_none() || second.is_none() {
            return None;
        }

        let mut min = first.unwrap();
        let mut max = second.unwrap();

        if min > max {
            std::mem::swap(&mut min, &mut max);
        }

        for x in iter {
            if x < min {
                min = x;
            } else if x > max {
                max = x;
            }
        }

        Some((min, max))
    }
}
