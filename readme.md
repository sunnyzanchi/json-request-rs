Nothing particularly interesting. I'm just experimenting with Rust. I wanted to do something that:

- I might actually need to do (call an API, do some sort of filtering/transform on the data)
- I thought I _could_ actually do

All this does right now is make a GET call to the [JSON PlaceHolder](https://jsonplaceholder.typicode.com/) `/posts` API, parses the JSON as a strongly typed struct and gets the unique user ids.
It was fun to write in a language that the compiler would be upset if I did literally anything wrong.
And I did a lot wrong :sweat_smile:
Also, coming from JavaScript, it was fun to write a language that has build-time checking at all :sparkles:

I think a lot of my initial frustration/confusion came from trying to understand how traits work and how futures work.
It took me forever to figure out what the hell `Future<Item = (), Error = ()>` meant.
_(Hint: They're [Associated Types](https://doc.rust-lang.org/book/2018-edition/ch19-03-advanced-traits.html))_
Once I figured that out, I understood how to properly annotate the `stringify` function so I could extract it from the closure, and why the final `map` and `map_err` in the chain are necessary.

# Running

```
cargo run

// [
//   1,
//   2,
//   3,
//   4,
//   5,
//   6,
//   7,
//   8,
//   9,
//   10
// ]
```
