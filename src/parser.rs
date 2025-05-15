use crate::tokenizer::{Token, Keyword};
use crate::ast::{Statement, Expression, ColumnDef, DataType};
use crate::pratt::PrattParser;
use crate::ParseError;

pub struct SQLParser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> SQLParser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, position: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position);
        self.position += 1;
        token
    }

    fn expect_keyword(&mut self, keyword: Keyword) -> Result<(), ParseError> {
        match self.advance() {
            Some(Token::Keyword(k)) if *k == keyword => Ok(()),
            Some(_) => Err(ParseError::ExpectedKeyword(format!("{:?}", keyword))),
            None => Err(ParseError::UnexpectedEnd),
        }
    }

    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        match self.advance() {
            Some(Token::Identifier(name)) => Ok(name.clone()),
            Some(_) => Err(ParseError::ExpectedIdentifier),
            None => Err(ParseError::UnexpectedEnd),
        }
    }

    /// Parses a single top-level SQL statement by dispatching to the appropriate handler
    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        // Peek at the current token to decide which kind of statement we're dealing with
        match self.peek() {
            Some(Token::Keyword(Keyword::Select)) => self.parse_select(),         // Handle SELECT
            Some(Token::Keyword(Keyword::Create)) => self.parse_create_table(),   // Handle CREATE TABLE
            Some(Token::Keyword(Keyword::Insert)) => self.parse_insert(),         // Handle INSERT INTO
            Some(tok) => Err(ParseError::UnknownStartOfStatement(format!("{:?}", tok))), // Unknown keyword
            None => Err(ParseError::General("Empty input".to_string())),         // No tokens to parse
        }
    }

    fn parse_select(&mut self) -> Result<Statement, ParseError> {
        self.expect_keyword(Keyword::Select)?;

        let mut columns = Vec::new();

        loop {
            match self.advance() {
                Some(Token::Identifier(name)) => columns.push(name.clone()),
                Some(Token::Comma) => continue,
                Some(Token::Keyword(Keyword::From)) => break,
                Some(tok) => {
                    return Err(ParseError::General(format!("Unexpected token in column list: {:?}", tok)))
                }
                None => {
                    return Err(ParseError::General("Unexpected end of input while reading columns.".to_string()))
                }
            }
        }

        let table = self.expect_identifier()?;

        let mut selection = None;
        if let Some(Token::Keyword(Keyword::Where)) = self.peek() {
            self.advance();
            let remaining_tokens = &self.tokens[self.position..];
            let mut expr_parser = PrattParser::new(remaining_tokens);
            let expr = expr_parser
                .parse_expression(1)
                .map_err(ParseError::InvalidExpression)?;
            selection = Some(expr);
        }

        let mut order_by = None;
        if let Some(Token::Keyword(Keyword::Order)) = self.peek() {
            self.advance();
            self.expect_keyword(Keyword::By)?;

            let mut columns = Vec::new();
            loop {
                match self.advance() {
                    Some(Token::Identifier(name)) => columns.push(name.clone()),
                    Some(Token::Comma) => continue,
                    Some(Token::Semicolon) | Some(Token::Eof) => break,
                    Some(tok) => {
                        return Err(ParseError::General(format!("Unexpected token in ORDER BY: {:?}", tok)))
                    }
                    None => {
                        return Err(ParseError::UnexpectedEnd);
                    }
                }
            }
            order_by = Some(columns);
        }

        Ok(Statement::Select {
            columns,
            table,
            selection,
            order_by,
        })
    }

    fn parse_create_table(&mut self) -> Result<Statement, ParseError> {
        self.expect_keyword(Keyword::Create)?;
        self.expect_keyword(Keyword::Table)?;

        let table_name = self.expect_identifier()?;

        self.expect_keyword(Keyword::LeftParen)?;

        let mut columns = Vec::new();
        loop {
            match self.advance() {
                Some(Token::Identifier(col_name)) => {
                    let name = col_name.clone(); // clone to end borrow
                    let col_type = self.parse_column_type()?;
                        columns.push(ColumnDef {
                        name,
                        data_type: col_type
                    });
                }
                Some(Token::Comma) => continue,
                Some(Token::RightParen) => break,
                Some(tok) => {
                    return Err(ParseError::General(format!("Unexpected token: {:?}", tok)))
                }
                None => {
                    return Err(ParseError::UnexpectedEnd);
                }
            }
        }

        Ok(Statement::CreateTable {
            table_name,
            columns,
        })
    }

    fn parse_column_type(&mut self) -> Result<DataType, ParseError> {
        match self.advance() {
            Some(Token::Keyword(Keyword::Int)) => Ok(DataType::Int),
            Some(Token::Keyword(Keyword::Varchar)) => {
                if let Some(Token::LeftParen) = self.peek() {
                    self.advance();
                    if let Some(Token::Number(n)) = self.advance() {
                        let size = *n;
                        if let Some(Token::RightParen) = self.advance() {
                            return Ok(DataType::Varchar(size as usize));
                        }
                    }
                }
                Err(ParseError::General("Expected size for Varchar".to_string()))
            }
            Some(Token::Keyword(Keyword::Boolean)) => Ok(DataType::Boolean),
            Some(tok) => Err(ParseError::General(format!("Unexpected column type: {:?}", tok))),
            None => Err(ParseError::UnexpectedEnd),
        }
    }

    fn parse_insert(&mut self) -> Result<Statement, ParseError> {
        self.expect_keyword(Keyword::Insert)?;
        self.expect_keyword(Keyword::Into)?;

        let table_name = self.expect_identifier()?;

        self.expect_keyword(Keyword::LeftParen)?;

        let mut columns = Vec::new();
        loop {
            match self.advance() {
                Some(Token::Identifier(col_name)) => columns.push(col_name.to_string()),
                Some(Token::Comma) => continue,
                Some(Token::RightParen) => break,
                Some(tok) => {
                    return Err(ParseError::General(format!("Unexpected token in column list: {:?}", tok)))
                }
                None => {
                    return Err(ParseError::UnexpectedEnd);
                }
            }
        }

        self.expect_keyword(Keyword::Values)?;

        self.expect_keyword(Keyword::LeftParen)?;

        let mut values = Vec::new();
        loop {
            match self.advance() {
                Some(Token::Number(n)) => values.push(Expression::Number(*n)),
                Some(Token::String(s)) => values.push(Expression::String(s.to_string())),
                Some(Token::Boolean(b)) => values.push(Expression::Boolean(*b)),
                Some(Token::Null) => values.push(Expression::Null),
                Some(Token::Identifier(s)) => values.push(Expression::Identifier(s.to_string())),
                Some(Token::Comma) => continue,
                Some(Token::RightParen) => break,
                Some(tok) => {
                    return Err(ParseError::General(format!("Unexpected token in VALUES: {:?}", tok)))
                }
                None => {
                    return Err(ParseError::UnexpectedEnd);
                }
            }
        }

        Ok(Statement::Insert {
            table_name,
            columns,
            values,
        })
    }
}
