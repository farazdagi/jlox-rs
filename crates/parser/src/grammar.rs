use core::fmt;

/// Expression types.
#[derive(Debug)]
pub enum Expr<'a> {
    Binary(Binary<'a>),
    Grouping(Grouping<'a>),
    Literal(Literal<'a>),
    Unary(Unary<'a>),
}

#[derive(Debug)]
pub struct Binary<'a> {
    pub left: Box<Expr<'a>>,
    pub operator: BinaryOperator,
    pub right: Box<Expr<'a>>,
}

#[derive(Debug)]
pub struct Grouping<'a> {
    pub expression: Box<Expr<'a>>,
}

#[derive(Debug)]
pub enum Literal<'a> {
    Number(f64),
    String(&'a str),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub struct Unary<'a> {
    pub operator: UnaryOperator,
    pub right: Box<Expr<'a>>,
}

#[derive(Debug)]
pub enum BinaryOperator {
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Self::EqualEqual => "==",
            Self::BangEqual => "!=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Star => "*",
            Self::Slash => "/",
        };
        write!(f, "{}", out)
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    Minus,
    Bang,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Self::Minus => "-",
            Self::Bang => "!",
        };
        write!(f, "{}", out)
    }
}

/// Implementors of this trait can visit a data structure `U` and return a
/// result of type `V`.
pub trait Visitor<U, V> {
    fn visit(&mut self, data: &U) -> V;
}

/// Expression visitor must be able to visit all expression types.
pub trait ExprVisitor<'a, V>
where
    Self: Sized
        + Visitor<Binary<'a>, V>
        + Visitor<Grouping<'a>, V>
        + Visitor<Literal<'a>, V>
        + Visitor<Unary<'a>, V>,
{
    fn visit_expr(&mut self, data: &Expr<'a>) -> V {
        match data {
            Expr::Binary(binary) => binary.accept(self),
            Expr::Grouping(grouping) => grouping.accept(self),
            Expr::Literal(literal) => literal.accept(self),
            Expr::Unary(unary) => unary.accept(self),
        }
    }
}

impl<'a, T, V> Visitor<Expr<'a>, V> for T
where
    T: ExprVisitor<'a, V>,
{
    fn visit(&mut self, data: &Expr<'a>) -> V {
        self.visit_expr(data)
    }
}

/// Defines a data structure that can be visited by a visitor.
pub trait Visitable<T>
where
    Self: Sized,
{
    fn accept(&self, visitor: &mut dyn Visitor<Self, T>) -> T {
        visitor.visit(self)
    }
}

impl<T> Visitable<T> for Expr<'_> {}
impl<T> Visitable<T> for Binary<'_> {}
impl<T> Visitable<T> for Grouping<'_> {}
impl<T> Visitable<T> for Literal<'_> {}
impl<T> Visitable<T> for Unary<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    // Sample visitor implementation that prints the expression.
    struct AstPrinter;

    // Implementing this marker trait enforces implementation of the Visitor for
    // other Visitable types.
    impl ExprVisitor<'_, String> for AstPrinter {}

    impl Visitor<Binary<'_>, String> for AstPrinter {
        fn visit(&mut self, data: &Binary<'_>) -> String {
            format!(
                "({} {} {})",
                data.operator,
                data.left.accept(self),
                data.right.accept(self)
            )
        }
    }

    impl Visitor<Grouping<'_>, String> for AstPrinter {
        fn visit(&mut self, data: &Grouping<'_>) -> String {
            format!("(group {})", data.expression.accept(self))
        }
    }

    impl Visitor<Literal<'_>, String> for AstPrinter {
        fn visit(&mut self, data: &Literal<'_>) -> String {
            match data {
                Literal::Number(n) => n.to_string(),
                Literal::String(s) => s.to_string(),
                Literal::Boolean(b) => b.to_string(),
                Literal::Nil => "nil".to_string(),
            }
        }
    }

    impl Visitor<Unary<'_>, String> for AstPrinter {
        fn visit(&mut self, data: &Unary<'_>) -> String {
            format!("({} {})", data.operator, data.right.accept(self))
        }
    }

    #[test]
    fn test_printer() {
        let expr = Expr::Binary(Binary {
            left: Box::new(Expr::Unary(Unary {
                operator: UnaryOperator::Minus,
                right: Box::new(Expr::Literal(Literal::Number(123.0))),
            })),
            operator: BinaryOperator::Star,
            right: Box::new(Expr::Grouping(Grouping {
                expression: Box::new(Expr::Literal(Literal::Number(45.67))),
            })),
        });
        let mut printer = AstPrinter;
        let result = expr.accept(&mut printer);
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}
