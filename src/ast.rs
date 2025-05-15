// SQL AST components for Rust SQL parser
// Author: Joel Chirayath

//This defines all the possible kinds of values or conditions in SQL expressions like in WHERE, ORDER BY, or math formulas.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    Number(u64),
    String(String),
    UnaryOperation {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    BinaryOperation {
        left_operand: Box<Expression>,
        operator: BinaryOperator,
        right_operand: Box<Expression>,
    },
    Boolean(bool),
    Null,
    Grouped(Box<Expression>),
}

//this defines all the two-input operators used in SQL expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    And,
    Or,
    Add,
    Subtract,
    Multiply,
    Divide,
}

//These are single-input operators.
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Not,
    Negate,
}

//This represents top-level SQL statements. Right now, only support SELECT.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Select {
        columns: Vec<String>,
        table: String,
        selection: Option<Expression>,
        order_by: Option<Vec<String>>,
    },
    CreateTable {
        table_name: String,
        columns: Vec<ColumnDef>,
    },
    Insert {
        table_name: String,
        columns: Vec<String>,
        values: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Int,
    Varchar(usize),
    Boolean,
}