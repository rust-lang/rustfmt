# Testing rustfmts support for formatting code blocks in markdown files

The goal here is to format rust code blocks in markdown files.\
Formatting will _mostly_ impact unformatted code blocks, however there will be minor changes to markdown constructs outside of code blocks. For example, aligning GitHub Flavored Markdown (GFM) tables.


```rust
fn main()   {
                println!(            "Hello world!"
                )
}
```

Here's an indented code block that won't be formatted

    fn main()   {
                    println!(            "Hello world!"
                    )
    }

> > Here's a code block in a blockquote
> > ``` rust
> > fn main()   {
> >                   println!(            "Hello world!"
> >                   )
> > }
> > ```

* Here's a code block in a list!
  ```rust
  fn main()   {
                  println!(            "Hello world!"
                  )
  }
  ```

>> * Here's a code block in a deeply nested markdown context
>> * ```rust
>>   fn main()   {
>>                 println!(            "Hello world!"
>>                  )
>>   }
>>   ```

<p>Markdown also support inline HTML</p>

Oh and we've got some cool support for aligning tables:

| column 1 | column 2 | column 3|
| :---: | :--- | ---: |
| values for column 1 | values for column 2 | values for column 3 |
| 😁😁 | 🎉🎉🎉 | 😁 :^) :^)|

Check out the [commonmark spec]!

[commonmark spec]: https://spec.commonmark.org/0.30/

Look we can also link to rust traits or types like [`Debug`] and [`Vec`].
Some additional text with [brackets]. what if I manually \[esacpe the bracket\]? looks like they stay escaped!


[a dead link]: https://this/link/isnt/used

[`Debug`]: core::fmt::Debug
