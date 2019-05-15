<link rel="shortcut icon" type="image/x-icon" href="image/logo.ico">

<p align="center">
    <img src="image/logo_text.png" height="250">
</p>

<p align="center">
    <a href="https://travis-ci.org/JSAbrahams/mamba/branches?branch=master"><img src="https://img.shields.io/travis/JSAbrahams/mamba/master.svg?style=for-the-badge&logo=linux" alt="Travis"/></a>
    <a href="https://ci.appveyor.com/project/JSAbrahams/mamba"><img src="https://img.shields.io/appveyor/ci/JSAbrahams/mamba/master.svg?style=for-the-badge&logo=windows" alt="Appveyor"/></a>
    <a href="https://app.codacy.com/project/JSAbrahams/mamba/dashboard"><img src="https://img.shields.io/codacy/grade/74944b486d444bf2b772e7311e9ae2f4.svg?style=for-the-badge" alt="Code Quality"/></a>
    <a href="https://codecov.io/gh/JSAbrahams/mamba"><img src="https://img.shields.io/codecov/c/github/JSAbrahams/mamba.svg?style=for-the-badge" alt="Coverage"/></a>
    <br>
    <a href="https://github.com/JSAbrahams/mamba/blob/master/LICENSE"><img src="https://img.shields.io/github/license/JSAbrahams/mamba.svg?style=for-the-badge" alt="License"/></a>
    <img src="https://img.shields.io/badge/Built%20with-%E2%99%A5-red.svg?style=for-the-badge" alt="Built with Love"/>
</p>

# Mamba

This is the Mamba programming language. 
The Documentation can be found [here](https://joelabrahams.nl/mamba_doc).
This documentation outlines the different language features, and also contains a formal specification of the language.

In short, Mamba is like Python, but with a few key features:
-   Strict typing rules, but with type inference so it doesn't get in the way too much, and type refinement features.
-   Null safety features.
-   More explicit error handling.
-   Clear distinction between mutability and immutability.
-   Pure, or injective, functions.

This is a transpiler, written in [Rust](https://www.rust-lang.org/), which converts Mamba source code to Python source files.
Mamba code should therefore, in theory, be interoperable with Python code.
Functions written in Python can be called in Mamba and vice versa.

## ⌨️ Code Examples

Below are some code examples to showcase the features of Mamba.
We highlight how functions work, how de define classes, how types and type refinement features are applied, how Mamba can be used to ensure pureness, and how error handling works.
For more extensive examples and explanations check out the [documentation](https://joelabrahams.nl/mamba_doc).

### ➕ Functions

We can write a simple script that computes the factorial of a value given by the user.
```mamba
def factorial(x: Int) => match x with
    0 => 1
    n => n * factorial (n - 1)

def num <- input "Compute factorial: "
if num.is_digit then
    def result <- factorial int num
    print "Factorial [num] is: [result]."
else
    print "Input was not an integer."
```

Notice how here we specify the type of argument `x`, in this case an `Int`, by writing `x: Int`.
This means that the compiler will check for us that factorial is only used with integers as argument.

### 📋 Classes and mutability

Classes are similar to classes in Python, though we can for each function state whether we can write to `self` or not by stating whether it is mutable or not.
We showcase this using a simple dummy `Server` object.
```mamba
import ipaddress

class MyServer(def ip_address: ipaddress.ip_address)
    def mut is_connected: Bool           <- false
    def mut private last_message: String <- undefined

    def last_sent(self) => if self.last_message /= undefined 
        then message
        else Err("No last message!")

    def connect(mut self) => self.is_connected <- true

    def send(mut self, message: String) => if self.is_connected 
        then self.last_message <- message
        else Err("Not connected!")

    def disconnect(mut self) => self.is_connected <- true
```

Notice how:
-   `self` is not mutable in `last_sent`, meaning we can only read variables, whereas in connect `self` is mutable, so we can change properties of `self`.
-   `last_message` is private, denoted by the `private` keyword.
    This means that we cannot access is directly, meaning we cannot for instance do `server.last_message <- "Mischief"`.
    Instead, we call `server.last_sent`.

Which we can then use as follows in our script:
```mamba
import ipaddress
from server import MyServer

def some_ip   <- ipaddress.ip_address "151.101.193.140"
def my_server <- MyServer(some_ip)

http_server connect
if my_server.connected then http_server send "Hello World!"

print "last message sent before disconnect: \"[my_server.last_sent]\"."
my_server.disconnect
```

### 🗃 Types and type refinement

As shown above Mamba has a type system.
Mamba however also has type refinement features to assign additional properties to types.

Lets expand our server example from above, and rewrite it slightly:
```mamba
import ipaddress

type Server
    def ip_address: ipaddres.ip_address

    def connect:    () -> ()       throws [ServerErr]
    def send:       (String) -> () throws [ServerErr]
    def disconnect: () -> ()

type ServerErr(msg: String) isa Err(msg)

class MyServer(mut self: DisconnectedMyServer, def ip_address: ipaddress.ip_address) isa Server
    def mut is_connected: Bool           <- false
    def mut private last_message: String <- undefined

    def last_sent(self): String => self.last_message

    def connect(mut self: DisconnectedMyServer) => self.is_connected <- true

    def send(mut self: ConnectedMyServer, message: String) => self.last_message <- message

    def disconnect(mut self: ConnectedMyServer) => self.is_connected <- false

type ConnectedMyServer isa MyServer when
    self is_connected else ServerErr("Not connected.")

type DisconnectedMyServer isa MyServer when
    not self is_connected else ServerErr("Already connected.")
```

Notice how above, we define the type of `self`.

Each type effectively denotes another state that `self` can be in.
For each type, we use `when` to show that it is a type refinement, which certain conditions.

```mamba
import ipaddress
from server import MyServer

def some_ip   <- ipaddress.ip_address "151.101.193.140"
def my_server <- MyServer(some_ip)

# The default state of http_server is DisconnectedHTTPServer, so we don't need to check that here
http_server.connect

# We check the state
if my_server isa ConnectedMyServer then
    # http_server is a Connected Server if the above is true
    my_server.send "Hello World!"

print "last message sent before disconnect: \"[my_server.last_sent]\"."
if my_server isa ConnectedMyServer then my_server.disconnect
```

Type refinement also allows us to specify the domain and co-domain of a function, say, one that only takes and returns positive integers:
```mamba
type PositiveInt isa Int where self >= 0 else Err("Expected positive Int but was [self].")

def factorial (x: PositiveInt) => match x with
    0 => 1
    n => n * factorial (n - 1)
```

In short, types allow us to specify the domain and co-domain of functions with regards to the type of input, say, `Int` or `String`.

Type refinement allows us to to some additional things:
-   It allows us to further specify the domain or co-domain of a function
-   It allows us to explicitly name the possible states of an object.
    This means that we don't constantly have to check that certain conditions hold.
    We can simply ask whether a given object is a certain state by checking whether it is a certain type.
    
### 🔒 Pure functions

Mamba has features to ensure that functions are injective, meaning that if `x = y`, for any injective `f`, `f(x) = f(y)`.
Such injective functions are also `pure` functions.
By default, functions are not pure, and can read any variable they want, such as in Python.
When we make a function `puer`, it cannot:
-   Read mutable variables not passed as an argument (with one exception).
-   Read mutable properties of `self`.
    Mainly since `self` is never given as an argument, so a function output only depends on its explicit arguments.
-   Call non-pure functions.

With the above properties, we can ensure that a function is pure, or injective.
`pure` is similar to `mut`.
When a function is `pure`, we ensure that its output is always the same for a given input.
Mutability, denoted with `mut`, decides whether we can or can't change a value.
So, `pure` is a property of functions, and `mut` is a property of variables.

```mamba
def taylor <- 7

# the sinus function is injective, its output depends solely on the input
def pure sin(x: Int) =>
    def mut ans <- x
    for i in 1 ..= taylor step 2 do
        ans <- (x ^ (i + 2)) / (factorial (i + 2))
    ans
```

We can add `pure` to the top of a file, which ensures all functions in said file are pure.
This is useful when we want to write multiple pure functions.

```mamba
pure

def taylor <- 7

def sin(x: Int): Real =>
    def mut ans <- x
    for i in 1 ..= taylor step 2 do ans <- (x ^ (i + 2)) / (factorial (i + 2))
    ans
    
def cos(x: Int): Real =>
    def mut ans <- x
    for i in 0 .. taylor step 2 do ans <- (x ^ (i + 2)) / (factorial (i + 2))
    ans
```

### ⚠ Error handling

Unlike Python, Mamba does not have `try` `except` and `finally` (or `try` `catch` as it is sometimes known).
Instead, we aim to directly handle errors on-site so the origin of errors is more tracable.
The following is only a brief example.
Error handling can at times becomes quite verbose, so we do recommend checking out the [docs](https://joelabrahams.nl/mamba_doc/features/safety/error_handling.html) on error handling to get a better feel for error handling.

We can modify the above script such that we don't check whether the server is connected or not.
In that case, we must handle the case where `http_server` throws a `ServerErr`:
```mamba
import ipaddress
from server import MyServer

def some_ip   <- ipaddress.ip_address "151.101.193.140"
def my_server <- MyServer(some_ip)

def message <- "Hello World!"
my_server.send message handle
    err: ServerErr => 
        print "Error while sending message: \"[message]\": [err]"
        # We must now return to halt execution
        return

if my_server isa ConnectedMyServer then my_server.disconnect
```

In the above script, we will always print the error since we forgot to actually connect to the server.
Here we showcase how we try to handle errors on-site instead of in a (large) `try` block.
This means that we don't need a `finally` block: We aim to deal with the error where it happens and then continue executing the remaining code.
This also prevents us from wrapping large code blocks in a `try`, where it might not be clear what statement or expression might throw what error.

`handle` can also be combined with an assign.
In that case, we must either always return (halting execution or exiting the function), or evaluate to a value.
This is shown below:
```mamba
def a <- function_may_throw_err() handle
    err : MyErr => 
        print "We have a problem: [err.message]."
        # we return, halting execution
        return
    err : MyOtherErr => 
        print "We have another problem: [err.message]."
        # or if we don't return, return a default value
        0
        
print ""
```

## 👥 Contributing

Before submitting your first issue or pull request, please take the time to read both our [contribution guidelines](CONTRIBUTING.md) and our [code of conduct](CODE_OF_CONDUCT.md).

## 🔨 Tooling

Several tools are used to help maintain the quality of the codebase.
These tools are used by the continuous integration tools to statically check submitted code.
Therefore, to save time, it is a good idea to install these tools locally and run them before pushing your changes.

### Rustfmt

[Rustfmt](https://github.com/rust-lang/rustfmt) formats Rust code and ensures the formatting is consistent across the codebase.

-   **To install** `rustup component add rustfmt --toolchain nightly`
-   **To run** `cargo +nightly fmt`

The configuration of `Rustfmt` can be found in `.rustfmt.toml`.

*Note* The nightly build of `cargo` must be used (`rustup install nightly`).

### Clippy

[Clippy](https://github.com/rust-lang/rust-clippy) catches common mistakes made in Rust.

-   **To install** `rustup component add clippy`
-   **To run** `cargo clippy`

The configuration of `Clippy` can be found in `.clippy.toml`.

*Note* The stable build of `cargo` must be used (`rustup install stable`).
