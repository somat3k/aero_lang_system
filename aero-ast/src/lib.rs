use serde::{Deserialize, Serialize};

/// Represents a complete AERO program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub items: Vec<Item>,
}

/// Top-level item in an AERO program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    World(WorldDecl),
    Use(UseDecl),
}

/// Function declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub effects: Vec<String>,
    pub body: Block,
    pub attributes: Vec<Attribute>,
}

/// Function parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub ty: Type,
}

/// Struct declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
}

/// Struct field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub ty: Type,
}

/// Enum declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<Variant>,
}

/// Enum variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    pub name: String,
    pub fields: Option<Vec<Type>>,
}

/// World type declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldDecl {
    pub name: String,
    pub adapter: String,
    pub fields: Vec<Field>,
}

/// Use declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseDecl {
    pub path: Vec<String>,
}

/// Type representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Named(String),
    Generic(String, Vec<Type>),
    World(Box<Type>),
    Result(Box<Type>, Box<Type>),
    Option(Box<Type>),
    Unit,
}

/// Code block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Statement>,
}

/// Statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Know(KnowStmt),
    Emit(EmitStmt),
    Expr(Expression),
    Return(Option<Expression>),
}

/// Know statement (knowledge assertion)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowStmt {
    pub name: String,
    pub value: Expression,
}

/// Emit statement (effect emission)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmitStmt {
    pub effect: String,
    pub args: Vec<Expression>,
}

/// Expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Call(Box<Expression>, Vec<Expression>),
    Binary(Box<Expression>, BinaryOp, Box<Expression>),
    Unary(UnaryOp, Box<Expression>),
    Match(Box<Expression>, Vec<MatchArm>),
}

/// Literal value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Unit,
}

/// Binary operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

/// Unary operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOp {
    Negate,
    Not,
}

/// Match arm in a match expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Expression,
}

/// Pattern for pattern matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Wildcard,
    Literal(Literal),
    Identifier(String),
    Variant(String, Vec<Pattern>),
}

/// Attribute (e.g., #[test])
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub args: Vec<String>,
}

/// Find all test functions in the AST
pub fn find_test_functions(program: &Program) -> Vec<&Function> {
    program
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Function(f) => Some(f),
            _ => None,
        })
        .filter(|f| {
            f.attributes
                .iter()
                .any(|attr| attr.name == "test")
        })
        .collect()
}

/// Format the AST back to AERO source code
pub fn format(program: &Program) -> String {
    let mut output = String::new();

    for item in &program.items {
        match item {
            Item::Function(f) => {
                // Attributes
                for attr in &f.attributes {
                    output.push_str(&format!("#[{}]\n", attr.name));
                }

                // Function signature
                output.push_str("fn ");
                output.push_str(&f.name);
                output.push('(');

                for (i, param) in f.parameters.iter().enumerate() {
                    if i > 0 {
                        output.push_str(", ");
                    }
                    output.push_str(&param.name);
                    output.push_str(": ");
                    output.push_str(&format_type(&param.ty));
                }

                output.push(')');

                if let Some(ref ret_ty) = f.return_type {
                    output.push_str(" -> ");
                    output.push_str(&format_type(ret_ty));
                }

                if !f.effects.is_empty() {
                    output.push_str(" ! [");
                    for (i, eff) in f.effects.iter().enumerate() {
                        if i > 0 {
                            output.push_str(", ");
                        }
                        output.push_str(eff);
                    }
                    output.push(']');
                }

                output.push_str(" {\n");
                output.push_str(&format_block(&f.body, 1));
                output.push_str("}\n\n");
            }
            Item::Struct(s) => {
                output.push_str("struct ");
                output.push_str(&s.name);
                output.push_str(" {\n");
                for field in &s.fields {
                    output.push_str("    ");
                    output.push_str(&field.name);
                    output.push_str(": ");
                    output.push_str(&format_type(&field.ty));
                    output.push_str(",\n");
                }
                output.push_str("}\n\n");
            }
            _ => {} // Other items
        }
    }

    output
}

fn format_type(ty: &Type) -> String {
    match ty {
        Type::Named(name) => name.clone(),
        Type::Generic(name, args) => {
            let args_str = args
                .iter()
                .map(format_type)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}<{}>", name, args_str)
        }
        Type::World(inner) => format!("world<{}>", format_type(inner)),
        Type::Result(ok, err) => format!("Result<{}, {}>", format_type(ok), format_type(err)),
        Type::Option(inner) => format!("Option<{}>", format_type(inner)),
        Type::Unit => "()".to_string(),
    }
}

fn format_block(block: &Block, indent: usize) -> String {
    let mut output = String::new();
    let indent_str = "    ".repeat(indent);

    for stmt in &block.statements {
        output.push_str(&indent_str);
        match stmt {
            Statement::Know(k) => {
                output.push_str(&format!("know {} = {};\n", k.name, format_expr(&k.value)));
            }
            Statement::Emit(e) => {
                output.push_str(&format!("emit {}(", e.effect));
                for (i, arg) in e.args.iter().enumerate() {
                    if i > 0 {
                        output.push_str(", ");
                    }
                    output.push_str(&format_expr(arg));
                }
                output.push_str(");\n");
            }
            Statement::Expr(expr) => {
                output.push_str(&format!("{};\n", format_expr(expr)));
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    output.push_str(&format!("return {};\n", format_expr(e)));
                } else {
                    output.push_str("return;\n");
                }
            }
        }
    }

    output
}

fn format_expr(expr: &Expression) -> String {
    match expr {
        Expression::Literal(lit) => match lit {
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Integer(i) => i.to_string(),
            Literal::Float(f) => f.to_string(),
            Literal::Boolean(b) => b.to_string(),
            Literal::Unit => "()".to_string(),
        },
        Expression::Identifier(id) => id.clone(),
        Expression::Call(func, args) => {
            let args_str = args
                .iter()
                .map(format_expr)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({})", format_expr(func), args_str)
        }
        Expression::Binary(left, op, right) => {
            let op_str = match op {
                BinaryOp::Add => "+",
                BinaryOp::Subtract => "-",
                BinaryOp::Multiply => "*",
                BinaryOp::Divide => "/",
                BinaryOp::Equal => "==",
                BinaryOp::NotEqual => "!=",
                BinaryOp::Less => "<",
                BinaryOp::Greater => ">",
                BinaryOp::LessEqual => "<=",
                BinaryOp::GreaterEqual => ">=",
            };
            format!("{} {} {}", format_expr(left), op_str, format_expr(right))
        }
        Expression::Unary(op, expr) => {
            let op_str = match op {
                UnaryOp::Negate => "-",
                UnaryOp::Not => "!",
            };
            format!("{}{}", op_str, format_expr(expr))
        }
        Expression::Match(_, _) => {
            // Simplified for now
            "match { ... }".to_string()
        }
    }
}
