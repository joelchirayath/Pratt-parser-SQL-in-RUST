# Programming Languages – project specification

## Project specification changelog

Look here for any changes to the specification.

* 11.02.2025. – Initial project specification
* 06.05.2025. – Fixes to the complex `CREATE TABLE` example, `column_name` and `table_name` were `Expression`s instead of `String`s

## About the project

The project involves implementing a `SQL` parser in Rust. It focuses on parsing `SELECT` and `CREATE` statements with `FROM`, `WHERE`, and `ORDER BY` clauses. The goal is to develop a tokenizer, a parser, a simple CLI and an error-handling mechanism, all of which are combined in order to evaluate a string from a command-line input and output an `SQL` statement.

## Project structure

* Tokenizer – an object that transforms a string input into a collection of tokens that the parser works with.
* Pratt expression parser – a part of the main parser (can be a list of functions, can be a separate object that contains a list of functions) that implements the Pratt parsing technique for parsing expressions. This parser contains logic for operator precedence.
* `SQL` parser – the main parser, that in combination with the Pratt expression parser, parses the whole `SQL` string into a database `Statement`. This parser contains logic for the structure of, for example, a `SELECT` statement. This component calls the tokenizer, as this component contains the main function that the user calls.
* Error handling – Our parser should inform the user when they type an incorrect query. This can be done with the `Result<T, E>` enum.
* CLI – a simple command line interface that loops indefinitely: ask the user for a query, parse the query, display the result (or an error).

## Project goal

Our goal with this project is to teach you Rust through parsing, and we have chosen this subset of the `SQL` language to parse because of its relative simplicity to other programming languages.

## Dos and don'ts

You need to:
* Use our starting project code. You can, however, change the file hierarchy to your liking.
* Comment (document) your code to explain what the code does and how or why you did something.
* Put your name and surname in the `Cargo.toml` file instead of `programming_languages_project_name_surname`. Also, in the `authors` list, put your academic email.
* Contact us if you think we forgot something, made a mistake, or have a question.

You don't need to:
* Semantically check the parsed expressions, meaning you don't need to check for types of columns in clauses (`SELECT name FROM users WHERE name = 5;` is completely okay) and you don't need to check for mathematical errors such as dividing by zero. This is because, in a real database, this step is done only after parsing.
* Implement floating point numbers.
* Use `git`, but we will be thankful if you do.
* Use Rust's functional paradigm but it is recommended. This topic will be covered in one of our lab work classes.

## Grading and submission

You can get up to 40 points on the project, and they are divided into five sections that follow the project structure:
* Tokenizer – implementing the tokenizer gets you up to 7 points.
* Pratt expression parser – implementing the Pratt parsing technique for expressions gets you up to 10 points (requires a functioning tokenizer).
* `SQL` parser – implementing the `SQL` parser gets you up to 13 points (requires a functioning Pratt parser).
* Error handling – implementing correct error handling gets you up to 9 points.
* CLI – implementing a CLI for the application gets you up to 1 point.

Note: Points are given based on error resistance of the components and how correct are their outputs.

Note: Do **not** develop other parsing techniques if you are developing the project ahead of lab work schedule. Pratt parsing was chosen for a reason, and you will probably lose all points in that grading section.

Note: When we say that the Pratt expression parser requires a functioning tokenizer, that doesn't mean the tokenizer needs to be 100 percent correct, just that it performs it's function to some degree that the Pratt expression parser can understand.

Note: For your points to count, you need to be able to verbally explain your code.

Note: Any form of plagiarism will be met with a 0 point project grade.

| Functionality                                                                                                                                                                             | Number of points | 
|:------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|:----------------:|
| 1. **Tokenizer** - tokenization of single character tokens (`+, -, *, /, =, ...`)                                                                                                         |        1         |
| 2. **Tokenizer** - tokenization of multiple character tokens (`>=, <=, !=, ...`)                                                                                                          |        1         |
| 3. **Tokenizer** - tokenization of numbers                                                                                                                                                |        1         |
| 4. **Tokenizer** - tokenization of strings (`"String", 'String'`)                                                                                                                         |        1         |
| 5. **Tokenizer** - tokenization of keywords (`SELECT, CREATE, WHERE, ...`)                                                                                                                |        1         |
| 6. **Tokenizer** - tokenization of identifiers (`x, y, name, surname, ...`)                                                                                                               |        1         |
| 7. **Tokenizer** - implementation with an iterator                                                                                                                                        |        1         |
| 8. **Pratt parser** - correct order of operations                                                                                                                                         |        2         |
| 9. **Pratt parser** - handling parentheses                                                                                                                                                |        1         |
| 10. **Pratt parser** - parsing binary operations (`1 + 2, 3 / 4, ((17 - x) * 22 - 7) / 5, ...`)                                                                                           |        4         |
| 11. **Pratt parser** - parsing unary operations (`-5 + 6, 7 - -8`)                                                                                                                        |        3         |
| 12. **SQL parser** - parsing `SELECT` without `WHERE` and without `ORDER BY`                                                                                                              |        3         |
| 13. **SQL parser** - parsing `SELECT` with optional `WHERE` and optional `ORDER BY`                                                                                                       |        3         |
| 14. **SQL parser** - parsing `CREATE TABLE` with types only                                                                                                                               |        4         |
| 15. **SQL parser** - parsing `CREATE TABLE` with types and constraints                                                                                                                    |        3         |
| 16. **Error handling (tokenizer)** - handle errors from the tokenizer (quotes not matched, ...)                                                                                           |        1         |
| 17. **Error handling (pratt parser)** - handle errors in the pratt parser (expression ended early, invalid token, ...)                                                                    |        1         |
| 18. **Error handling (SQL parser)** - handle errors while building a `SELECT` statement (no `FROM` keyword, ...)                                                                          |        3         |
| 19. **Error handling (SQL parser)** - handle errors while building a `CREATE TABLE` statement (no length on `VARCHAR` type, expression not correct in `CHECK` constraint, ...)            |        3         |
| 20. **Error handling** - handle and propagate errors with a `Result` enumeration instead of calling `panic!()`                                                                            |        1         |
| 21. **CLI** - make a CLI that infinitely loops and prints to the user the inputted string as a parsed statement                                                                           |        1         |

### For your project to be graded, you must submit it until **15.05.2025. 23:59:59**.

### The project should be submitted on the **Moodle platform**. An assigment will be created in due time.

## Bonus points

* Classroom activity (as previously mentioned) – up to 1 point
* Writing automated unit tests for the Tokenizer, Pratt parser part, and `SQL` parser – up to 3 points
* Implementing the `FOREIGN KEY` constraint – up to 1 point
* Implementing the `SELECT * FROM table;` syntax, where `*` is treated as a wildcard operator instead of a multiplication operator – up to 2 points

Note: bonus points are capped at 5 points maximum.

## Materials

Note: "Pratt parsing" is also called "Top-Down operator precedence parsing"

Note: We have given multiple sites for the Pratt parsing explanation, as different explanations can better suit different students

[Rust book](https://doc.rust-lang.org/book/)

[Pratt parsing technique on Stackexchange](https://langdev.stackexchange.com/questions/3254/what-exactly-is-pratt-parsing-used-for-and-how-does-it-work)

[A blog post on Pratt parsers](https://eli.thegreenplace.net/2010/01/02/top-down-operator-precedence-parsing) (the implementation is given in Python 2, which is not ideal. Look at the `pratt_parser_simplified_python.py` file which is the same implementation, but in Python 3)

[Another blog post on Pratt parsers](https://dev.to/jrop/pratt-parsing)

[Operator precedence](https://www.bouraspage.com/repository/algorithmic-thinking/what-is-the-order-of-precedence-of-arithmetic-comparison-and-logical-operators)

[Writing automated unit tests in Rust](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) (for bonus points)

[`FOREIGN KEY` constraint usages in `SQL`](https://www.w3schools.com/sql/sql_foreignkey.asp) (for bonus points; implement the SQL Server syntax)

## Project start

To download the project, you can `git clone` it, or click the green code button, and download the code as zip. Viewing this specification is recommended through GitHub, as this markdown file is correctly rendered here. Alternatively, open the `Project specification.html` in a browser to view this file rendered locally.

The project you will be building will already contain some entities related to your parser's output, to make life easier for you (and for us). By giving you these entities as a starting point, we will ensure project uniformity across every student.
* `Statement` – final parser return type
* `Expression` – tree-like structured enum that encapsulates parsed expressions
* `TableColumn` – a structure containing all information about a table column (used in `CREATE TABLE` statements)
* `DBType` – an enum that contains different types of data that our database supports
* `Constraint` – an enum that contains different types of constraints that our database supports
* `BinaryOperator` – operators that have two operands
* `UnaryOperator` – operators that have one operand
* `Token` – tokens that your tokenizer can produce
* `Keyword` – `SQL` reserved keywords

Since this project isn't structured like the other projects you have already worked on, we are giving you a general guide on development steps. You don't have to follow these steps, but they are here to give you a sense of direction.

Firstly, transforming a string directly into an SQL statement is hard, because you need to make your parser work with characters one by one. This is where the tokenizer comes into play, and you should develop the tokenizer before anything else. When you have a tokenizer, your basic entity aren't characters anymore, but tokens, and in the parser, there is no worry if a token is typed correctly, or if the token should be interpreted as a keyword, or a number, ...

Secondly, when you make your tokenizer work, you should begin on implementing the Pratt parsing technique. The Pratt parsing technique is essentially a couple of functions that call each other to create an `Expression`. Test it in an isolated environment, where you can feed it arbitrary tokens that you know can be parsed into a valid `Expression`.

Thirdly, the SQL parser, which utilizes the tokenizer and the Pratt parsing technique, is where the SQL logic handled. The SQL parser calls the tokenizer to enable working with tokens. Here, when the user calls the `build_statement` function, you firstly check if the first token in the sequence is `SELECT` or `CREATE`. In the case of `CREATE`, you must check if the second token in the sequence is `TABLE`. If the tokens aren't in the correct order, return an error to the user. When you know that, for example, the first token is `SELECT`, you know that the next batch of tokens are going to be related to the selection expressions. Since selection expressions are separated by a comma, check for commas after each selection expression. When you know there aren't any more selection expressions, the `FROM` keyword must be next. Return an error if there is no `FROM` keyword, or if there is no string after the `FROM` keyword (this string is the table name). Now, you know that the `WHERE` keyword may come, if it's there, parse the expression after it, if it's not there, `ORDER`, `BY` may come, parse the expression after it. This logic is the same in the `CREATE TABLE` statement, but for different keywords and different requirements. When you have all the parts of either statement, create an instance of `Statement` and return it to the user.

When you finish with these three parts, implement a simple CLI and thoroughly test the code.

Error handling can be done on the way, but you can make it more robust once you know you have a working application. 

---
### Entities related to the `Statement` enum (the first big concept)

```rust
#[derive(Debug, PartialEq)]  
pub enum Statement {  
    Select {  
        columns: Vec<Expression>,  
        from: String,  
        r#where: Option<Expression>,  
        orderby: Vec<Expression>,  
    },  
    CreateTable {  
        table_name: String,  
        column_list: Vec<TableColumn>,  
    }  
}
```
The main entity of the whole parser. `Statement` is implemented as an enumeration because adding functionality is as easy as adding an enumeration constant and implementing functionality for that enumeration constant (implementation in the database command interpreter, which is not a part of this project). Parsing any correct `SELECT` or `CREATE`  (or `UPDATE`, `INSERT INTO`, ... hypothetically) statement should be turned into an instance of this enumeration. Ultimately, your main parser function (something like `build_statement(query: &str) -> Statement`) should return this enumeration.

The `SELECT` statement has four components:
1. `columns` – A vector of columns (selection expressions) from the selected table that the database should return.
2. `from` – A simple string, containing a table that is being queried (we aren't doing joins because they complicate stuff too much for this project).
3. `where` – A single expression that is the actual filter for the database query. It is wrapped in an `Option` because not every `SELECT` query contains a filter. The actual name is `r#where` because in Rust, `where` is a reserved keyword, and the prefix `r#` means: interpret this token as a raw string, do not check for keyword matches.
4. `orderby` – A vector of expressions that define how the data should be ordered. A vector is needed because the data can be ordered by the first column, and then all data that has the same first column can be ordered by the second column, ... Also, the data can be ordered not simply by columns, but by complex expressions as well.

The `CREATE TABLE` statement has two components:
1. `table_name` – A simple string, the name of the table.
2. `column_list` – A vector of table column types, where each table column contains the definition of one column.

---
```rust
#[derive(Debug, PartialEq)]  
pub struct TableColumn {  
    pub column_name: String,  
    pub column_type: DBType,  
    pub constraints: Vec<Constraint>,  
}
```
A structure containing a definition for one column, when creating a table.
1. `column_name` – A simple string, representing a name.
2. `column_type` – The type of the column. Types are defined in the `DBType` enum.
3.  `constraints` – A vector of constraints on the column. Types of constraints are defined in the `Constraint` enum.

---

```rust
#[derive(Debug, PartialEq)]  
pub enum DBType {  
    Int,  
    Varchar(usize),  
    Bool,  
}
```
A column in the database can be any of these types. `Int` and `Bool` types have no additional info, while the `Varchar(n)` type has an additional argument – the length of the string. Adding a type, such as `DECIMAL(n, m)` is boiled down to adding tokens for that type, parsing that type and adding it to this enum.

---

```rust
#[derive(Debug, PartialEq)]  
pub enum Constraint {  
    NotNull,  
    PrimaryKey,  
    Check(Expression)  
}
```
A column can be limited to a domain of values, which is defined by constraints on that column. `PrimaryKey` and `NotNull` constraints have no additional info, while the `Check` constraints has an additional argument – the expression which every table row must satisfy.

---
### Entities related to the `Expression` enum (the second big concept)

```rust
#[derive(Debug, PartialEq)]  
pub enum Expression {  
    BinaryOperation {  
        left_operand: Box<Expression>,  
        operator: BinaryOperator,  
        right_operand: Box<Expression>,  
    },  
    UnaryOperation {  
        operand: Box<Expression>,  
        operator: UnaryOperator,  
    },  
    Number(u64),  
    Bool(bool),  
    Identifier(String),  
    String(String),  
}
```
The main entity of the expression parser. The Expression enum is structured like this, where an expression can contain another expression. This naturally allows us to represent complex expressions as trees. `Box<T>` smart pointers are used on unary and binary types of expressions because the compiler needs to know the size of the enum at compile time which is impossible when an enum contains itself (infinite size).

An expression can be:
* complex - a number of other expressions (tree-like structure, unary and binary operations)
* a single number
* a single identifier (like a variable 'x')
* a single string (when doing parsing of WHERE statements that do operations with strings, strings must be in matching quotes – either `""` or `''`)
* a boolean (only true or false)

---
```rust
#[derive(Debug, PartialEq)]  
pub enum BinaryOperator {  
    Plus, Minus, Multiply, Divide,  
    GreaterThan, GreaterThanOrEqual,  
    LessThan, LessThanOrEqual,  
    Equal, NotEqual, And, Or,  
}
```
```rust
#[derive(Debug, PartialEq)]  
pub enum UnaryOperator {  
    Not, Plus, Minus,  
    Asc, Desc,  
}
```
Binary and unary operators are defined as enums, where each enumeration constant represents one operator. Binary and unary operators are defined separately because a `-` (minus), for example can be in a binary operation: `5 - 4`, as well as in a unary operation: `-2`. `Asc` and `Desc` are `ORDER BY` operators that have the lowest operator precedence in any expression. While both unary and binary operators may be the exact same as tokens that represent them (read below), it is important to make a distinction between them, as they are used in different contexts.

---
### Entities related to the `Tokenizer` struct (the third big concept)

While the `Tokenizer` struct isn't provided as starting material, tokens that the `Tokenizer` struct uses, are. Your `Tokenizer` should produce these tokens, based on the input string. Your `Parser` should use them to create `Expression`s and `Statement`s.

```rust
#[derive(PartialEq, Clone, Debug)]  
pub(crate) enum Token {  
    Keyword(Keyword),  
    Identifier(String),  
    String(String),  
    Number(u64),  
    Invalid(char),  
    RightParentheses, LeftParentheses,  
    GreaterThan, GreaterThanOrEqual,  
    LessThan, LessThanOrEqual,  
    Equal, NotEqual, Multiply,  
    Divide, Minus, Plus,  
    Comma, Semicolon, Eof,  
}
```
Adding, for example, the exponent operator, requires adding the `Caret` (`^`) enumeration constant here.

---

```rust
#[derive(PartialEq, Clone, Debug)]  
pub(crate) enum Keyword {  
    Select, Create, Table, Where,  
    Order, By, Asc, Desc, From,  
    And, Or, Not, True, False,  
    Primary, Key, Check, Int,  
    Bool, Varchar, Null  
}
```
Keywords in the `SQL` language. Adding, for example, the `FOREIGN KEY` constraint, requires adding the `FOREIGN` keyword here. If you structure your tokenizer well enough, that is all that is needed for adding a new keyword.

## Examples

All given examples are valid Rust code and can be used for testing purposes. Our advice is to directly compare your parser's result to the given example code. The `PartialEq` trait is derived for every enum and struct in the given files so comparing should be straightforward with `==`.

### Tokenized input strings (result of the tokenizer's `tokenize_string` function)

---

```sql
SELECT name, surname FROM users;
```
is a string that, when tokenized into a vector of tokens, looks like this:
```rust
vec![  
    Token::Keyword(Keyword::Select),  
    Token::Identifier("name".to_string()),  
    Token::Comma,  
    Token::Identifier("surname".to_string()),  
    Token::Keyword(Keyword::From),  
    Token::Identifier("users".to_string()),  
    Token::Semicolon,
    Token::Eof  
]
```
---
```sql
SELECT one, two FROM users WHERE one>1 AND two<1;
```
is a string that, when tokenized into a vector of tokens, looks like this:
```rust
vec![  
    Token::Keyword(Keyword::Select),  
    Token::Identifier("one".to_string()),  
    Token::Comma,  
    Token::Identifier("two".to_string()),  
    Token::Keyword(Keyword::From),  
    Token::Identifier("users".to_string()),  
    Token::Keyword(Keyword::Where),  
    Token::Identifier("one".to_string()),  
    Token::GreaterThan,  
    Token::Number(1),  
    Token::Keyword(Keyword::And),  
    Token::Identifier("two".to_string()),  
    Token::LessThan,  
    Token::Number(1),  
    Token::Semicolon,
    Token::Eof  
]
```
---
```sql
SELECT "string" FROM users;
```
is a string that, when tokenized into a vector of tokens, looks like this:
```rust
vec![  
    Token::Keyword(Keyword::Select),  
    Token::String("string".to_string()),  
    Token::Keyword(Keyword::From),  
    Token::Identifier("users".to_string()),  
    Token::Semicolon,
    Token::Eof  
]
```
---
```sql
'aa123'
```
is a string that, when tokenized into a vector of tokens, looks like this:
```rust
vec![  
    Token::String(String::from("aa123")),  
    Token::Eof  
]
```
---
```
'no matching quotes" "no ending quotes
```
are strings, that, the tokenizer should throw an error to the user when it encounters them.

### Parsed expressions (result of the parser's `parse_expression` function)

---
```
(13 + 7) - 4
``` 
is an expression that contains two expressions:
1. `(13 + 7)` which is
```rust
Expression::BinaryOperation {
    left_operand: Box::new(Expression::Number(13)),
    operator: BinaryOperator::Plus,
    right_operand: Box::new(Expression::Number(7))
}
```
2. `4` which is
```rust
Expression::Number(4)  
```  
Therefore, the whole expression after parsing should look like this:
```rust
Expression::BinaryOperation {
    left_operand: Expression::BinaryOperation {
        left_operand: Box::new(Expression::Number(13)),
        operator: BinaryOperator::Plus,
        right_operand: Box::new(Expression::Number(7))
    },
    operator: BinaryOperator::Minus,
    right_operand: Box::new(Expression::Number(4))
}
```  
---  
```
(5 - x) < (4 + y) OR name = "Donna"
``` 
is an expression that contains five (three small and two combining) expressions:
1. `(5 - x)` which is
```rust
Expression::BinaryOperation {
    left_operand: Box::new(Expression::Number(5)),
    operator: BinaryOperator::Minus,
    right_operand: Box::new(Expression::Identifier("x".to_string())),
}
```  
2. `(4 - y)` which is
```rust
Expression::BinaryOperation {
    left_operand: Box::new(Expression::Number(4)),
    operator: BinaryOperator::Plus,
    right_operand: Box::new(Expression::Identifier("y".to_string()))
}
```  
3. `name = "Donna"` which is
```rust
Expression::BinaryOperation {
    left_operand: Box::new(Expression::Identifier("name".to_string())),
    operator: BinaryOperator::Equal,
    right_operand: Box::new(Expression::String("Donna".to_string()))
}
```  
Therefore, the whole expression after parsing should look like this:
```rust
Expression::BinaryOperation {
    left_operand: Box::new(Expression::BinaryOperation {
        left_operand: Box::new(Expression::BinaryOperation {
            left_operand: Box::new(Expression::Number(5)),
            operator: BinaryOperator::Minus,
            right_operand: Box::new(Expression::Identifier("x".to_string()))
        }),
        operator: BinaryOperator::LessThan,
        right_operand: Box::new(Expression::BinaryOperation {
            left_operand: Box::new(Expression::Number(4)),
            operator: BinaryOperator::Plus,
            right_operand: Box::new(Expression::Identifier("y".to_string()))
        })
    }),
    operator: BinaryOperator::Or,
    right_operand: Box::new(Expression::BinaryOperation {
        left_operand: Box::new(Expression::Identifier("name".to_string())),
        operator: BinaryOperator::Equal,
        right_operand: Box::new(Expression::String("Donna".to_string()))
    })
}
```  
---
```
NOT some_boolean = TRUE
``` 
should look like this:
```rust
Expression::BinaryOperation {
    left_operand: Box::new(Expression::UnaryOperation {
        left_operand: Box::new(Expression::Identifier("some_boolean".to_string())),
        operator: UnaryOperator::Not
    }),
    operator: BinaryOperator::Equal,
    right_operand: Box::new(Expression::Bool(true))
} 
```
---
```
5 * 3 - 4 + c / (13 -)
```
is a string, that, the parser should throw an error to the user when it encounters it.


### Completely parsed `SELECT` and `CREATE TABLE` statements (result of the parser's `build_statement` function)

---
```sql
SELECT name, surname FROM users;
``` 
is a `SELECT` statement that,  when parsed, looks like this:
```rust
Statement::Select {  
	columns: vec![
		Expression::Identifier("name".to_string()), 
		Expression:Identifier("surname".to_string()) 
	], 
	from: "users".to_string(), 
	r#where: None, 
	orderby: vec![]
}  
```  
 ---
```sql
SELECT age * 5, 'this is a string' FROM users;
``` 
is a `SELECT` statement that,  when parsed, looks like this:
```rust
Statement::Select {
    columns: vec![
        Expression::BinaryOperation {
            left_operand: Box::new(Expression::Identifier("age".to_string())),
            operator: BinaryOperator::Multiply,
            right_operand: Box::new(Expression::Number(5)),
        },
        Expression::String("this is a string".to_string()),
    ],
    from: "users".to_string(),
    r#where: None,
    orderby: vec![]
} 
```  
 ---
```sql
SELECT name, surname FROM users WHERE name = \"Voldemort\" AND surname = 'Riddle';
``` 
is a  `SELECT` statement that, when parsed, looks like this:
```rust
Statement::Select {
    columns: [
        Expression::Identifier("name".to_string()),
        Expression::Identifier("surname".to_string()),
    ],
    from: "users".to_string(),
    r#where: Some(
        Expression::BinaryOperation {
            left_operand: Box::new(Expression::BinaryOperation {
                left_operand: Box::new(Expression::Identifier("name".to_string())),
                operand: BinaryOperator::Equals,
                right_operand: Box::new(Expression::String("Voldemort".to_string())),
            }),
            operand: BinaryOperator::And,
            right_operand: Box::new(Expression::BinaryOperation {
                left_operand: Box::new(Expression::Identifier("surname".to_string())),
                operand: BinaryOperator::Equals,
                right_operand: Box::new(Expression::String("Riddle".to_string())),
            }),
        },
    ),
    orderby: vec![]
}
```  
 ---
```sql
SELECT id, salary FROM users ORDER BY salary - 2 * 10 ASC, id DESC;
```
is a  `SELECT` statement that, when parsed, looks like this:
```rust
Statement::Select {
    columns: vec![
        Expression::Identifier("id".to_string()),
        Expression::Identifier("salary".to_string()),
    ],
    from: "users".to_string(),
    r#where: None,
    orderby: vec![
        Expression::UnaryOperation {
            operand: Box::new(Expression::BinaryOperation {
                left_operand: Box::new(Expression::Identifier("salary".to_string())),
                operator: BinaryOperator::Minus,
                right_operand: Box::new(Expression::BinaryOperation {
                    left_operand: Box::new(Expression::Number(2)),
                    operator: BinaryOperator::Multiply,
                    right_operand: Box::new(Expression::Number(10)),
                }),
            }),
            operator: UnaryOperator::Asc,
        },
        Expression::UnaryOperation {
            operand: Box::new(Expression::Identifier("id".to_string())),
            operator: UnaryOperator::Desc,
        },
    ],
}
```  
 ---
```sql
SELECT id FROM registered_users WHERE password_encryption = TRUE ORDER BY id DESC;
``` 
is a  `SELECT` statement that, when parsed, looks like this:
```rust
Statement::Select {
    columns: vec![
        Expression::Identifier("id".to_string())
    ],
    from: "registered_users".to_string(),
    r#where: Some(
        Expression::BinaryOperation {
            left_operand: Box::new(Expression::Identifier("password_encryption".to_string())),
            operator: BinaryOperator::Equals,
            right_operand: Box::new(Expression::Bool(true))
        }
    ),
    orderby: vec![
        Expression::UnaryOperation {
            operand: Box::new(Expression::Identifier("id".to_string())),
            operator: UnaryOperator::Desc
        }
    ]
}
```  
---
```sql
CREATE TABLE simple_table(
	int_col INT, 
	string_col VARCHAR(255),
	bool_col BOOL
);
``` 
is a  `CREATE TABLE` statement that, when parsed, looks like this:
```rust
Statement::CreateTable {
    table_name: "simple_table".to_string(),
    column_list: vec![
        TableColumn {
            column_name: "int_col".to_string(),
            column_type: DBType::Int,
            constraints: vec![],
        },
        TableColumn {
            column_name: "string_col".to_string(),
            column_type: DBType::Varchar(255),
            constraints: vec![],
        },
        TableColumn {
            column_name: "bool_col".to_string(),
            column_type: DBType::Bool,
            constraints: vec![],
        },
    ]
}
```  
---
```sql
CREATE TABLE complex_table(
	id INT PRIMARY KEY, 
	email VARCHAR(255) NOT NULL,  
	is_junior BOOL, 
	age INT CHECK(age >= 18) CHECK(age <= 65)
);
``` 
is a  `CREATE TABLE` statement that, when parsed, looks like this:
```rust
Statement::CreateTable {
    table_name: "complex_table".to_string(),
    column_list: vec![
        TableColumn {
            column_name: "id".to_string(),
            column_type: DBType::Int,
            constraints: vec![
                Constraint::PrimaryKey,
            ],
        },
        TableColumn {
            column_name: "email".to_string(),
            column_type: DBType::Varchar(255),
            constraints: vec![
                Constraint::NotNull,
            ],
        },
        TableColumn {
            column_name: "is_junior".to_string(),
            column_type: DBType::Bool,
            constraints: vec![],
        },
        TableColumn {
            column_name: "age".to_string(),
            column_type: DBType::Int,
            constraints: vec![
                Constraint::Check(Expression::BinaryOperation {
                    left_operand: Box::new(Expression::Identifier("age".to_string())),
                    operator: BinaryOperator::GreaterThanOrEqual,
                    right_operand: Box::new(Expression::Number(18)),
                }),
                Constraint::Check(Expression::BinaryOperation {
                    left_operand: Box::new(Expression::Identifier("age".to_string())),
                    operator: BinaryOperator::LessThanOrEqual,
                    right_operand: Box::new(Expression::Number(65)),
                }),
            ],
        },
    ],
}
```
---
```sql
SELECT salary WHERE salary > 1000;
```
is a string, that, the parser should throw an error to the user when it encounters it (no `FROM` clause).

---
```sql
CREATE TABLE work_hours(num_hours INT)
```
is a string, that, the parser should throw an error to the user when it encounters it (no semicolon at the end).

### Call stack example

The call stack overview of the program is given in the project folder.

### Working CLI example

The video example of the program is given in the project folder. One difference is that the `from` field in the `Statement` enum is an `Expression`. This does not make a difference to you, you can put it to be a string without any problems. `{...}` can be completely ignored, as it is a quirk of my terminal.
