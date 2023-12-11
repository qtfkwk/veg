# Example

```rust
// Import Veg

use veg::Veg;

// Create a custom type

struct Point {
    x: f32,
    y: f32,
}

// Implement a method that creates a Box of the custom type

impl Point {
    fn new(x: f32, y: f32) -> Box<Point> {
        Box::new(Point { x, y })
    }
}

// Implement the veg::Table::row method to define how to print the custom type

impl veg::Table for Point {
    fn row(&self) -> Vec<String> {
        // Add `$`
        [self.x, self.y].iter().map(|x| format!("${x}$")).collect()

        // Other ideas:

        // - Add 3 decimal places:
        //
        //[self.x, self.y].iter().map(|x| format!("${x:.3}$")).collect()

        // - Do something different for x and y:
        //
        // vec![
        //    format!("${:.1}$", self.x),
        //    format!("${:.4}$", self.y),
        //]

        // - Just convert to string:
        //
        //[self.x, self.y].iter().map(|x| x.to_string())).collect()

        // ...
    }
}

// Create a Veg via the table function with a header definition

let mut t = Veg::table("$x$|$y$\n---:|---:");

// Add a single point

t.push(Point::new(1.0, 1.0));

// Add a bunch of points

t.append(&mut vec![
    Point::new(2.0, 4.0),
    Point::new(3.0, 9.0),
    Point::new(4.0, 16.0),
]);

// Render as markdown

let markdown = t.markdown();

assert_eq!(
    markdown,
    "  \\
  $x$ |  $y$
-----:|-----:
  $1$ |  $1$
  $2$ |  $4$
  $3$ |  $9$
  $4$ | $16$
\\
    ",
);
```

!inc:../CHANGELOG.md

