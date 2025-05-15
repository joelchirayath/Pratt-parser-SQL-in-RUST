#[derive(Debug, PartialEq, Clone)]
pub enum Keyword 
{
    Select,
    From,
    Where,
    Create,
    Table,
    Order,
    By,
    And,
    Or,
    Not,
    True,
    False,
    Int,
    Bool,
    Varchar,
    Primary,
    Key,
    Check,
    Null,
    Into,
    Insert,
    Values,
    Boolean,
    LeftParen,
    RightParen
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token 
{
    Keyword(Keyword),
    Identifier(String),
    String(String),
    Number(u64),
    Invalid(char),
    Boolean(bool),
    LeftParentheses,
    LeftParen,
    RightParentheses,
    RightParen,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
    Multiply,
    Divide,
    Minus,
    Plus,
    Comma,
    Semicolon,
    Eof,
    Null
}

pub struct Tokenizer 
{
    input: Vec<char>,
    position: usize,
}

impl Tokenizer 
{
    pub fn new(input: &str) -> Self 
    {
        Self 
        {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> 
    {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<char> 
    {
        let ch = self.peek();
        self.position += 1;
        ch
    }

    fn skip_whitespace(&mut self) 
    {
        while let Some(ch) = self.peek() 
        {
            if ch.is_whitespace() 
            {
                self.advance();
            } else 
            {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String 
    {
        let mut result = String::new();
        while let Some(ch) = self.peek() 
        {
            if ch.is_alphanumeric() || ch == '_' 
            {
                result.push(ch);
                self.advance();
            } 
            else 
            {
                break;
            }
        }
        result
    }

    fn lookup_keyword(word: &str) -> Option<Keyword> 
    {
        match word.to_uppercase().as_str() 
        {
            "SELECT" => Some(Keyword::Select),
            "FROM" => Some(Keyword::From),
            "WHERE" => Some(Keyword::Where),
            "CREATE" => Some(Keyword::Create),
            "TABLE" => Some(Keyword::Table),
            "ORDER" => Some(Keyword::Order),
            "BY" => Some(Keyword::By),
            "AND" => Some(Keyword::And),
            "OR" => Some(Keyword::Or),
            "NOT" => Some(Keyword::Not),
            "TRUE" => Some(Keyword::True),
            "FALSE" => Some(Keyword::False),
            "INT" => Some(Keyword::Int),
            "BOOL" => Some(Keyword::Bool),
            "VARCHAR" => Some(Keyword::Varchar),
            "PRIMARY" => Some(Keyword::Primary),
            "KEY" => Some(Keyword::Key),
            "CHECK" => Some(Keyword::Check),
            "NULL" => Some(Keyword::Null),
            _ => None,
        }
    }

    pub fn next_token(&mut self) -> Token 
    {
        self.skip_whitespace();
        match self.advance() 
        {
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some('(') => Token::LeftParentheses,
            Some(')') => Token::RightParentheses,
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Multiply,
            Some('/') => Token::Divide,
            Some('=') => Token::Equal,
            Some('>') => 
            {
                if self.peek() == Some('=') 
                {
                    self.advance();
                    Token::GreaterThanOrEqual
                } 
                else 
                {
                    Token::GreaterThan
                }
            }
            Some('<') => 
            {
                if self.peek() == Some('=') 
                {
                    self.advance();
                    Token::LessThanOrEqual
                } 
                else 
                {
                    Token::LessThan
                }
            }
            Some('!') => 
            {
                if self.peek() == Some('=') 
                {
                    self.advance();
                    Token::NotEqual
                } 
                else 
                {
                    Token::Invalid('!')
                }
            }
            Some(ch) if ch.is_alphabetic() => 
            {
                let mut ident = String::new();
                ident.push(ch);
                ident.push_str(&self.read_identifier());
                if let Some(keyword) = Self::lookup_keyword(&ident) 
                {
                    Token::Keyword(keyword)
                } else 
                {
                    Token::Identifier(ident)
                }
            }
            Some(ch) if ch.is_digit(10) => 
            {
                let mut num_str = String::new();
                num_str.push(ch);
                while let Some(next) = self.peek() 
                {
                    if next.is_digit(10) 
                    {
                        num_str.push(next);
                        self.advance();
                    } 
                    else 
                    {
                        break;
                    }
                }
                Token::Number(num_str.parse::<u64>().unwrap())
            }
            Some('"') | Some('\'') => 
            {
                let quote = self.input[self.position - 1];
                let mut result = String::new();
                while let Some(ch) = self.peek() 
                {
                    self.advance();
                    if ch == quote 
                    {
                        return Token::String(result);
                    } 
                    else 
                    {
                        result.push(ch);
                    }
                }
                Token::Invalid(quote)
            }
            None => Token::Eof,
            Some(ch) => Token::Invalid(ch),
        }
    }
}
