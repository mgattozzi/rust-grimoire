# Traits and Generics

One of the more foundational aspects of building larger Rust programs would be
Traits and Generics. They're fairly inescapable and you utilize them even in the
smallest of programs. What the heck are they anyways? Why would you need them?
The best way I've found to describe traits is that they fall into one of two
categories:

- They represent behavior that types implement
- They are a marker for some property that the type embodies

`Read` is a trait from the first category that creates a standard way to say
give me some bytes from this type, whether it's a file or a network socket. The
types that represent the file or the socket implement how to read those bytes.
The other category is for traits like `Eq`. You don't need to implement any
methods, but it tells the compiler that this type can check for equality. This
is why most of the primitive number types implement `PartialEq` and `Eq` but
floats (`f32` and `f64`) cannot, because equality is not a property of floating
point numbers. Let's look at an example of just how prevalent traits are in
Rust. We'll read in a number from stdin and then print it out.

```rust
{{#include ../code/src/read_num.rs}}
```

Can you figure out how many traits are in use here? Take a guess!

<details>
  This program uses 4 separate traits: <code class="hljs">FromStr</code>,
  <code class="hljs">Display</code>(if you want to be pedantic you could say 5
  since <code class="hljs">Display</code> implies
  <code class="hljs">ToString</code>), <code class="hljs">BufRead</code>, and
  <code class="hljs">Error</code>.
</details>

Probably more than you expected. Traits really start to get powerful when we use
them with generics.
