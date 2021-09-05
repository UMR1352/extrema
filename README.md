Utility for finding the minimum and maximum elements of an iterable collection in a single pass without copy.
Similar to C++'s [minmax_element](https://en.cppreference.com/w/cpp/algorithm/minmax_element), 
Julia's [extrema](http://www.jlhub.com/julia/manual/en/function/extrema) and Ruby's 
[minmax](https://apidock.com/ruby/v2_5_5/Enumerable/minmax).

## Usage
Just call it on a collection!
```
let xs = vec![0, 1, 2, 3];
let minmax = xs.extrema();

assert_eq!(minmax, Some((0, 3)));
```