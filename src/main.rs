//use rusty_lib;
fn main() {
    println!("Hello,2025 rustyapp vscode git submodule test3,git IDEA 19.04 Travis world!");
    greet();
    let _x = std::f64::consts::PI;
    //rust by example
    //https://doc.rust-lang.org/stable/rust-by-example/hello/print.html
    comments();
    formatprint();
    do_struct();
    do_traits();
    do_derive();
    //rusty_lib
}

fn greet() {
    let greetings = [
        "Hello",
        "Hola",
        "Bonjour",
        "Ciao",
        "こんにちは",
        "안녕하세요",
        "Cześć",
        "Olá",
        "Здравствуйте",
        "Chào bạn",
        "您好",
        "Hallo",
        "Hej",
        "Ahoj",
        "سلام",
    ];

    for (num, greeting) in greetings.iter().enumerate() {
        print!("{} : ", greeting);
        match num {
            0 => println!("This code is editable and runnable!"),
            1 => println!("¡Este código es editable y ejecutable!"),
            2 => println!("Ce code est modifiable et exécutable !"),
            3 => println!("Questo codice è modificabile ed eseguibile!"),
            4 => println!("このコードは編集して実行出来ます！"),
            5 => println!("여기에서 코드를 수정하고 실행할 수 있습니다!"),
            6 => println!("Ten kod można edytować oraz uruchomić!"),
            7 => println!("Este código é editável e executável!"),
            8 => println!("Этот код можно отредактировать и запустить!"),
            9 => println!("Bạn có thể edit và run code trực tiếp!"),
            10 => println!("这段代码是可以编辑并且能够运行的！"),
            11 => println!("Dieser Code kann bearbeitet und ausgeführt werden!"),
            12 => println!("Den här koden kan redigeras och köras!"),
            13 => println!("Tento kód můžete upravit a spustit"),
            14 => println!("این کد قابلیت ویرایش و اجرا دارد!"),
            _ => {}
        }
    }
}

// tutorial on
fn comments() {
    // This is an example of a line comment
    // There are two slashes at the beginning of the line
    // And nothing written inside these will be read by the compiler

    // println!("Hello, world!");

    // Run it. See? Now try deleting the two slashes, and run it again.

    /*
     * This is another type of comment, a block comment. In general,
     * line comments are the recommended comment style. But
     * block comments are extremely useful for temporarily disabling
     * chunks of code. /* Block comments can be /* nested, */ */
     * so it takes only a few keystrokes to comment out everything
     * in this main() function. /*/*/* Try it yourself! */*/*/
     */

    /*
    Note: The previous column of `*` was entirely for style. There's
    no actual need for it.
    */

    // You can manipulate expressions more easily with block comments
    // than with line comments. Try deleting the comment delimiters
    // to change the result:
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}

// tutorial on positional params and arguments and print format macro
fn formatprint() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Special formatting can be specified after a `:`.
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number = 1, width = 6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "JAMES!!");
    // FIXDONE ^ Add the missing argument: "James"

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    //println!("This struct `{}` won't print...", Structure(3));
    // FIXDONE ^ Comment out this line.
    let my_pi = 3.1;
    println!("Pi is roughly {0:.3}", my_pi); //https://doc.rust-lang.org/std/fmt/
}

fn do_struct() {
    #[derive(Debug)]
    struct Person<'a> {
        // The 'a defines a lifetime
        name: &'a str,
        age: u8,
    }

    // A unit struct
    struct Nil;

    // A tuple struct
    struct Pair(i32, f32);

    // A struct with two fields
    struct Point {
        x: f32,
        y: f32,
    }

    // Structs can be reused as fields of another struct
    #[allow(dead_code)]
    struct Rectangle {
        // A rectangle can be specified by where the top left and bottom right
        // corners are in space.
        top_left: Point,
        bottom_right: Point,
    }
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    /*
        // Destructure the point using a `let` binding
        let Point {
            x: top_edge,
            y: left_edge,
        } = point;

        let _rectangle = Rectangle {
            // struct instantiation is an expression too
            top_left: Point {
                x: left_edge,
                y: top_edge,
            },
            bottom_right: bottom_right,
        };
    */
    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

fn do_traits() {
    //@see https://doc.rust-lang.org/std/boxed/struct.Box.html
    //@Pimpl Pointer implement pattern in C++ , private members in impl struct
    // Pimpl For
    // Compile-Time Encapsulation
    // Cheshire Cat or Compiler Firewall idiom.
    /*  @see https://disney.fandom.com/wiki/Cheshire_Cat

    “Most everyone's mad here. Aha...
     HAHAHAHAHA!! You may have noticed that I'm not all there myself.”
    ―Cheshire Cat
     */

    struct Sheep {
        naked: bool,
        name: &'static str,
    }

    trait Animal {
        // Static method signature; `Self` refers to the implementor type.
        fn new(name: &'static str) -> Self;

        // Instance method signatures; these will return a string.
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        // Traits can provide default method definitions.
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            if self.is_naked() {
                // Implementor methods can use the implementor's trait methods.
                println!("{} is already naked...", self.name());
            } else {
                println!("{} gets a haircut!", self.name);

                self.naked = true;
            }
        }
    }

    // Implement the `Animal` trait for `Sheep`.
    impl Animal for Sheep {
        // `Self` is the implementor type: `Sheep`.
        fn new(name: &'static str) -> Sheep {
            Sheep { name, naked: false }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "baaaaah?"
            } else {
                "baaaaah!"
            }
        }

        // Default trait methods can be overridden.
        fn talk(&self) {
            // For example, we can add some quiet contemplation.
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }
    // Type annotation is necessary in this case.
    //let mut dolly: Sheep = Animal::new("Dolly");
    let mut dolly = Box::new(Sheep {
        name: "Dolly",
        naked: false,
    });
    // TODONE ^ Try removing the type annotations.

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

fn do_derive() {
    // `Centimeters`, a tuple struct that can be compared
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    // `Inches`, a tuple struct that can be printed
    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;

            Centimeters(inches as f64 * 2.54)
        }
    }

    // `Seconds`, a tuple struct with no additional attributes
    #[derive(Debug, PartialEq)]
    struct Seconds(i32);

    let _one_second = Seconds(1);

    // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
    println!("One second looks like: {:?}", _one_second);
    // TODONE ^ Try uncommenting this line

    // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
    let _this_is_true = _one_second == _one_second;
    // TODONE ^ Try uncommenting this line

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("One foot is {} than one meter.", cmp);
}
