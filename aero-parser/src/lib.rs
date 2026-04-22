use aero_ast::*;
use aero_lexer::Token;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),
    #[error("Expected {expected}, found {found:?}")]
    Expected {
        expected: String,
        found: Token,
    },
    #[error("Unexpected end of file")]
    UnexpectedEof,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let token = self.peek().clone();
        self.current += 1;
        token
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let token = self.advance();
        if std::mem::discriminant(&token) == std::mem::discriminant(&expected) {
            Ok(())
        } else {
            Err(ParseError::Expected {
                expected: format!("{:?}", expected),
                found: token,
            })
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut items = Vec::new();

        while !matches!(self.peek(), Token::Eof) {
            items.push(self.parse_item()?);
        }

        Ok(Program { items })
    }

    fn parse_item(&mut self) -> Result<Item, ParseError> {
        match self.peek() {
            Token::Fn | Token::Hash => Ok(Item::Function(self.parse_function()?)),
            Token::Struct => Ok(Item::Struct(self.parse_struct()?)),
            Token::Enum => Ok(Item::Enum(self.parse_enum()?)),
            Token::World => Ok(Item::World(self.parse_world()?)),
            Token::Use => Ok(Item::Use(self.parse_use()?)),
            token => Err(ParseError::UnexpectedToken(token.clone())),
        }
    }

    fn parse_attributes(&mut self) -> Result<Vec<Attribute>, ParseError> {
        let mut attributes = Vec::new();

        while matches!(self.peek(), Token::Hash) {
            self.advance(); // consume #
            self.expect(Token::LBracket)?;

            if let Token::Identifier(name) = self.advance() {
                attributes.push(Attribute {
                    name,
                    args: Vec::new(),
                });
            } else {
                return Err(ParseError::Expected {
                    expected: "attribute name".to_string(),
                    found: self.peek().clone(),
                });
            }

            self.expect(Token::RBracket)?;
        }

        Ok(attributes)
    }

    fn parse_function(&mut self) -> Result<Function, ParseError> {
        let attributes = self.parse_attributes()?;

        self.expect(Token::Fn)?;

        let name = if let Token::Identifier(n) = self.advance() {
            n
        } else {
            return Err(ParseError::Expected {
                expected: "function name".to_string(),
                found: self.peek().clone(),
            });
        };

        self.expect(Token::LParen)?;

        let mut parameters = Vec::new();
        while !matches!(self.peek(), Token::RParen) {
            if !parameters.is_empty() {
                self.expect(Token::Comma)?;
            }

            let param_name = if let Token::Identifier(n) = self.advance() {
                n
            } else {
                return Err(ParseError::Expected {
                    expected: "parameter name".to_string(),
                    found: self.peek().clone(),
                });
            };

            self.expect(Token::Colon)?;
            let param_type = self.parse_type()?;

            parameters.push(Parameter {
                name: param_name,
                ty: param_type,
            });
        }

        self.expect(Token::RParen)?;

        // Parse return type
        let return_type = if matches!(self.peek(), Token::Arrow) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        // Parse effects
        let effects = if matches!(self.peek(), Token::Bang) {
            self.advance();
            self.expect(Token::LBracket)?;

            let mut effs = Vec::new();
            while !matches!(self.peek(), Token::RBracket) {
                if !effs.is_empty() {
                    self.expect(Token::Comma)?;
                }

                if let Token::Identifier(eff) = self.advance() {
                    effs.push(eff);
                } else {
                    return Err(ParseError::Expected {
                        expected: "effect name".to_string(),
                        found: self.peek().clone(),
                    });
                }
            }

            self.expect(Token::RBracket)?;
            effs
        } else {
            Vec::new()
        };

        let body = self.parse_block()?;

        Ok(Function {
            name,
            parameters,
            return_type,
            effects,
            body,
            attributes,
        })
    }

    fn parse_struct(&mut self) -> Result<Struct, ParseError> {
        self.expect(Token::Struct)?;

        let name = if let Token::Identifier(n) = self.advance() {
            n
        } else {
            return Err(ParseError::Expected {
                expected: "struct name".to_string(),
                found: self.peek().clone(),
            });
        };

        self.expect(Token::LBrace)?;

        let mut fields = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            let field_name = if let Token::Identifier(n) = self.advance() {
                n
            } else {
                return Err(ParseError::Expected {
                    expected: "field name".to_string(),
                    found: self.peek().clone(),
                });
            };

            self.expect(Token::Colon)?;
            let field_type = self.parse_type()?;

            fields.push(Field {
                name: field_name,
                ty: field_type,
            });

            self.expect(Token::Comma)?;
        }

        self.expect(Token::RBrace)?;

        Ok(Struct { name, fields })
    }

    fn parse_enum(&mut self) -> Result<Enum, ParseError> {
        self.expect(Token::Enum)?;

        let name = if let Token::Identifier(n) = self.advance() {
            n
        } else {
            return Err(ParseError::Expected {
                expected: "enum name".to_string(),
                found: self.peek().clone(),
            });
        };

        self.expect(Token::LBrace)?;

        let mut variants = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            if !variants.is_empty() {
                self.expect(Token::Comma)?;
            }

            let variant_name = if let Token::Identifier(n) = self.advance() {
                n
            } else {
                return Err(ParseError::Expected {
                    expected: "variant name".to_string(),
                    found: self.peek().clone(),
                });
            };

            variants.push(Variant {
                name: variant_name,
                fields: None,
            });
        }

        self.expect(Token::RBrace)?;

        Ok(Enum { name, variants })
    }

    fn parse_world(&mut self) -> Result<WorldDecl, ParseError> {
        self.expect(Token::World)?;

        let name = if let Token::Identifier(n) = self.advance() {
            n
        } else {
            return Err(ParseError::Expected {
                expected: "world name".to_string(),
                found: self.peek().clone(),
            });
        };

        // Simplified: just parse as struct-like for now
        self.expect(Token::LBrace)?;

        let fields = Vec::new();

        self.expect(Token::RBrace)?;

        Ok(WorldDecl {
            name,
            adapter: "default".to_string(),
            fields,
        })
    }

    fn parse_use(&mut self) -> Result<UseDecl, ParseError> {
        self.expect(Token::Use)?;

        let mut path = Vec::new();
        loop {
            if let Token::Identifier(name) = self.advance() {
                path.push(name);
            } else {
                return Err(ParseError::Expected {
                    expected: "module name".to_string(),
                    found: self.peek().clone(),
                });
            }

            if !matches!(self.peek(), Token::DoubleColon) {
                break;
            }
            self.advance();
        }

        self.expect(Token::Semicolon)?;

        Ok(UseDecl { path })
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        match self.advance() {
            Token::Identifier(name) => {
                if matches!(self.peek(), Token::LAngle) {
                    self.advance();
                    let mut args = Vec::new();

                    loop {
                        args.push(self.parse_type()?);
                        if !matches!(self.peek(), Token::Comma) {
                            break;
                        }
                        self.advance();
                    }

                    self.expect(Token::RAngle)?;
                    Ok(Type::Generic(name, args))
                } else {
                    Ok(Type::Named(name))
                }
            }
            Token::World => {
                self.expect(Token::LAngle)?;
                let inner = self.parse_type()?;
                self.expect(Token::RAngle)?;
                Ok(Type::World(Box::new(inner)))
            }
            Token::LParen => {
                self.expect(Token::RParen)?;
                Ok(Type::Unit)
            }
            token => Err(ParseError::Expected {
                expected: "type".to_string(),
                found: token,
            }),
        }
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        self.expect(Token::LBrace)?;

        let mut statements = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            statements.push(self.parse_statement()?);
        }

        self.expect(Token::RBrace)?;

        Ok(Block { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.peek() {
            Token::Know => {
                self.advance();
                let name = if let Token::Identifier(n) = self.advance() {
                    n
                } else {
                    return Err(ParseError::Expected {
                        expected: "variable name".to_string(),
                        found: self.peek().clone(),
                    });
                };

                self.expect(Token::Assign)?;
                let value = self.parse_expression()?;
                self.expect(Token::Semicolon)?;

                Ok(Statement::Know(KnowStmt { name, value }))
            }
            Token::Emit => {
                self.advance();

                let effect = if let Token::Identifier(n) = self.advance() {
                    n
                } else {
                    return Err(ParseError::Expected {
                        expected: "effect name".to_string(),
                        found: self.peek().clone(),
                    });
                };

                // Handle :: for namespaced effects like log::info
                let effect = if matches!(self.peek(), Token::DoubleColon) {
                    self.advance();
                    if let Token::Identifier(method) = self.advance() {
                        format!("{}::{}", effect, method)
                    } else {
                        effect
                    }
                } else {
                    effect
                };

                self.expect(Token::LParen)?;

                let mut args = Vec::new();
                while !matches!(self.peek(), Token::RParen) {
                    if !args.is_empty() {
                        self.expect(Token::Comma)?;
                    }
                    args.push(self.parse_expression()?);
                }

                self.expect(Token::RParen)?;
                self.expect(Token::Semicolon)?;

                Ok(Statement::Emit(EmitStmt { effect, args }))
            }
            Token::Return => {
                self.advance();
                let expr = if matches!(self.peek(), Token::Semicolon) {
                    None
                } else {
                    Some(self.parse_expression()?)
                };
                self.expect(Token::Semicolon)?;
                Ok(Statement::Return(expr))
            }
            _ => {
                let expr = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                Ok(Statement::Expr(expr))
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self, min_precedence: u8) -> Result<Expression, ParseError> {
        let mut left = self.parse_primary_expression()?;

        loop {
            let (op, precedence) = match self.peek() {
                Token::Plus => (BinaryOp::Add, 1),
                Token::Minus => (BinaryOp::Subtract, 1),
                Token::Star => (BinaryOp::Multiply, 2),
                Token::Slash => (BinaryOp::Divide, 2),
                Token::EqualEqual => (BinaryOp::Equal, 0),
                Token::NotEqual => (BinaryOp::NotEqual, 0),
                Token::LAngle => (BinaryOp::Less, 0),
                Token::RAngle => (BinaryOp::Greater, 0),
                Token::LessEqual => (BinaryOp::LessEqual, 0),
                Token::GreaterEqual => (BinaryOp::GreaterEqual, 0),
                _ => break,
            };

            if precedence < min_precedence {
                break;
            }

            self.advance();
            let right = self.parse_binary_expression(precedence + 1)?;
            left = Expression::Binary(Box::new(left), op, Box::new(right));
        }

        Ok(left)
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, ParseError> {
        match self.advance() {
            Token::String(s) => Ok(Expression::Literal(Literal::String(s))),
            Token::Integer(i) => Ok(Expression::Literal(Literal::Integer(i))),
            Token::Float(f) => Ok(Expression::Literal(Literal::Float(f))),
            Token::True => Ok(Expression::Literal(Literal::Boolean(true))),
            Token::False => Ok(Expression::Literal(Literal::Boolean(false))),
            Token::Identifier(id) => {
                if matches!(self.peek(), Token::LParen) {
                    // Function call
                    self.advance();
                    let mut args = Vec::new();

                    while !matches!(self.peek(), Token::RParen) {
                        if !args.is_empty() {
                            self.expect(Token::Comma)?;
                        }
                        args.push(self.parse_expression()?);
                    }

                    self.expect(Token::RParen)?;
                    Ok(Expression::Call(
                        Box::new(Expression::Identifier(id)),
                        args,
                    ))
                } else {
                    Ok(Expression::Identifier(id))
                }
            }
            Token::LParen => {
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            Token::Minus => {
                let expr = self.parse_primary_expression()?;
                Ok(Expression::Unary(UnaryOp::Negate, Box::new(expr)))
            }
            Token::Bang => {
                let expr = self.parse_primary_expression()?;
                Ok(Expression::Unary(UnaryOp::Not, Box::new(expr)))
            }
            token => Err(ParseError::UnexpectedToken(token)),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, ParseError> {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aero_lexer::lex;

    #[test]
    fn test_parse_simple_function() {
        let source = r#"
            fn main() {
                know x = 42;
            }
        "#;

        let tokens = lex(source).unwrap();
        let program = parse(tokens).unwrap();

        assert_eq!(program.items.len(), 1);
        assert!(matches!(program.items[0], Item::Function(_)));
    }

    #[test]
    fn test_parse_function_with_effects() {
        let source = r#"
            fn test() ! [log] {
                emit log::info("test");
            }
        "#;

        let tokens = lex(source).unwrap();
        let program = parse(tokens).unwrap();

        if let Item::Function(func) = &program.items[0] {
            assert_eq!(func.effects.len(), 1);
            assert_eq!(func.effects[0], "log");
        } else {
            panic!("Expected function");
        }
    }
}
