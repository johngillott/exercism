bobahop
Mentor
posted 5 days ago

Congratulations on passing all the tests!

    I like the use of a constant.

    I like that PartialEq was derived instead of implementing it manually.

    I like the use of {:02}:{:02} instead of calculating the leading zero manually.

    I like that Clock only stores minutes.

Instead of % or /, we can consider using the integer method rem_euclid which returns non-negative results, or div_euclid for Euclidean division. These were made stable in Rust 1.38.

Another approach to consider would be to use chrono::NaiveTime with chrono::Duration . The chrono crate was introduced in the Gigasecond exercise.

Another thing to consider is how a ::new function could be bypassed by initializing from a struct literal, something like so...

let my_Clock = Clock {hours: -24, minutes:-1440};

There is an article on struct literals vs constructors that explains how to prevent initialization by a struct literal. The struct literal concerns are not specifically related to this exercise (because the tests won't allow the struct literals) but are to be considered when using structs in your own projects. As the author writes: "To me, understanding things like this is when I really start to feel like I’m getting to know a language: putting three or four disparate concepts together to achieve some goal. It’s when a language stops being a bunch of disjoint parts and starts becoming a cohesive whole."
