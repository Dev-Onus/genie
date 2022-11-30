# rust cheat sheet for javascript(nodejs) developers

- basics of rust packages(crates)

```rust
// comparing nodejs package architecture, we can relate concepts in rust to following in nodejs
// to create an app(package) we need to use "npm init" and in rust we need to use "cargo new APP_NAME"
// additionally if you can specify the project type as "library" or "binary"(exe) by specifying "--lib" or "--bin" respectively. this kind of seggregation is not there in nodejs
// "cargo" is the "npm" for the rust.
// "cargo.toml" is the "package.json" for rust.
// "cargo.lock" is the "package-lock.json" for rust
// "cargo run" is the "npm run" in rust for running the scripts.
//  there is no support for custom scripts
// "cargo add" is npm install "npm i" for rust.
//  some of the cargo commands: "cargo run", "cargo add", "cargo new"
```

- basics of rust file structure

```javascript
import React from "react"; // default external import
import App from "./app"; // default relative import
import { createRoot } from "react-dom"; //named import
import "./app.scss"; // style import supported by bundlers like webpack
export default const a = 5;
```

->

```rust
// for using external crates
extern crate quote; //external package need to be used in the code.
extern crate swc_ecma_ast;
// for using modules from external crates
use quote::Quote;
use swc_ecma_ast::{Program,ModuleDecl};//for importing multiple objects use "{}"

//to declare a module of a crate, you need to use "mod" keyword.
// physically  a module can be created in 2 ways:
//1.create rs file with module name(ex:migrate.rs)
//2. if i need a folder to organize the module inside a folder then i need to create a folder with a module name with "mod.rs" file
// below code will look either for migrate.rs or migrate/mod.rs
// if crate is of type library then "lib.rs" is the base file rust compiler(rustc) will look for and mod declaration should start from there otherwise if it is a binary type then "main.rs" will be looked for.
mod migrate;

// to export a file from a module use "pub" keyword to declare it as public. If you want a struct to be public only to the current crate alone then use "pub(crate)"
pub(crate) struct ScssWriter {
  pub program:Program,
};
```

- basics of functions in rust

```rust
// to create a function use a keyword "fn"
// followed by single space and then the function name in the snake case
// followed by functional brackets "(", ")" and parameters for the function between them seperated by comma ",".
// each parameter will have its name followed by colon ":" and then its type.
// if there is a return value use the arrow keyword "->" with appropriate space before and after
// and the return type.
// if there is no return simply ignore the arrow keyword above.
// if we need a value to be returned at the end of the function do not give semicolon ";" at the end of the line
// if there is no return give semicolon without fail like we do for all the other lines for the function
// all main.rs file should have a main function which rust will call for the initial execution
fn execute_code_no_return(){
  print!("Hello, ");
}

fn execute_code_return_string(name:String) -> String {
  format!("{}", name)
}

fn main(){
  execute_code_no_return();
  print!("{}",execute_code_return_string("Akash"));
}

```

- print a new line in terminal like Hello World

```javascript
console.log("Hello World");
```

->

```rust
println!("Hello World");
```

- print a new line in terminal like Hello World along with a value

```javascript
const a = 5;
console.log("Hello World", a);
console.log(`Hello World ${a}`);
```

->

```rust
let a = 5;
println!("Hello World {:?}", a); or dbg!("Hello World", a);
println!("Hello World {}", a);
```

- print a new line in terminal like Hello World along with a string interpolation/template literal

```javascript
const a = 5;
console.log(`Hello World ${a}`);
```

->

```rust
let a = 5;
let str = format!("Hello Yogesh {}", a);
println!("Hello World {}", str);
```

- create a function and return string from it

```javascript
const myFunc = (a) => {
  return `Hello ${a}`;
};
console.log(myFunc("Meyy"));
```

->

```rust
let a = 5;
let str = format!("Hello Yogesh {}", a);
println!("Hello World {}", str);
```
