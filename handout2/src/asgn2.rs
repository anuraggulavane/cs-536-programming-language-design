use peg::*;
/* Implement a PEG parser for the following context-free grammar.
 * 
 * Implement each terminal symbol according to its English specification
 * Terminal Symbols:
 * An id starts with any letter, optionally followed by any combination
 * of letters, numbers and underscores.
 * 
 * A numeral is an optional minus sign, then a required integer part, 
 * then an optional period and fractional part.
 * An integer part optionally starts with a minus sign, which is then
 * followed by either a single 0 or a sequence of digits that starts with a nonzero digit.
 * A fractional part is a nonempty sequence of digits.
 * 
 * Note that the function syntax differs between function definitions
 * and function calls. In a definition, a list of variable names is given,
 * in a call, a list of expressions is given. The variable list is called
 * a ParamList and the expression list is called an ArgList.
 * 
 * Variable Symbols:
 * Atom <- numeral |  id "(" ArgList ")" | id | "(" Expr ")"
 * Op2 <- Atom * Op2 | Atom
 * Op1 <- Op2 + Op1 | Op2 - Op1 | Op2
 * Expr <- "let" Defn "in" Expr | Op1
 * Defn <- "var" id "=" Expr | "function" id "(" ParamList ")" "{" Expr "}"
 *
 * NonEmptyArgList <- Expr, NonEmptyArgList | Expr
 * ArgList <-  NonEmptyArgList | <empty string>
 *
 * NonEmptyParamList <- id, NonEmptyParamList | id
 * ParamList <-  NonEmptyParamList | <empty string>
 */

 /* BEGIN STARTER CODE */
 /* Your code will use the types Expr and Defn. 
    Do read these definitions to understand them. */
#[derive(Clone)]
pub enum Expr {
    Id(String),
    Numeral(f64),
    Times(Box<Expr>,Box<Expr>),
    Plus(Box<Expr>,Box<Expr>),
    Minus(Box<Expr>,Box<Expr>),
    Let(Box<Defn>,Box<Expr>),
    FunCall(String, Vec<Expr>),
} 
#[derive(Clone)]
pub enum Defn {
    VarDefn(String, Box<Expr>),
    FunDefn(String, Vec<String>, Box<Expr>),
}

/* The following three functions can be used in debugging and testing code.
   You do not need them to implement the homework itself. */
pub fn expr_eq(e1: Expr, e2: Expr) -> bool {
  match (e1,e2) {
    (Expr::Id(s1),Expr::Id(s2)) => s1 == s2,
    (Expr::Numeral(n1), Expr::Numeral(n2)) => n1 == n2,
    (Expr::Times(l1,r1),Expr::Times(l2,r2)) => expr_eq(*l1,*l2) && expr_eq(*r1,*r2),
    (Expr::Plus(l1,r1),Expr::Plus(l2,r2)) => expr_eq(*l1,*l2) && expr_eq(*r1,*r2),
    (Expr::Minus(l1,r1),Expr::Minus(l2,r2)) => expr_eq(*l1,*l2) && expr_eq(*r1,*r2),
    (Expr::Let(d1,e1),Expr::Let(d2,e2)) => defn_eq(*d1,*d2) && expr_eq(*e1,*e2),
    (Expr::FunCall(f1, args1),Expr::FunCall(f2,args2)) =>  {
    for (x,y) in args1.iter().zip(args2.iter()) {
      if !expr_eq(x.clone(),y.clone()) {
        return false;
      }
    }

    return f1 == f2},
    _ =>false,
  }
}

pub fn defn_eq(d1: Defn, d2: Defn) -> bool {
  match (d1,d2) {
    (Defn::FunDefn(f1, args1, body1), Defn::FunDefn(f2,args2,body2)) => 
    f1 == f2 && args1 == args2 && expr_eq(*body1,*body2),
    (Defn::VarDefn(x1, body1), Defn::VarDefn(x2, body2)) => x1 == x2 && expr_eq(*body1,*body2),
    _ => false,
  }
}

pub fn expr_to_string(e: Expr) -> String {
 match e {
    Expr::Id(s) => s,
    Expr::Numeral(f) => f.to_string(),
    Expr::Times(l,r) =>format!("{}*{}", expr_to_string(*l), expr_to_string(*r)),
    Expr::Plus(l,r) =>format!("{}+{}", expr_to_string(*l), expr_to_string(*r)),
    Expr::Minus(l,r) =>format!("{}-{}", expr_to_string(*l), expr_to_string(*r)),
    Expr::Let(d,e)=>format!("let {} in {}", defn_to_string(*d), expr_to_string(*e)),
    Expr::FunCall(f,args)=>{
      let mut arg_str = "".to_string();
      for s in args {
        arg_str = format!("{},{}",arg_str, expr_to_string(s))
      }
    format!("{}({})", f,arg_str)
    }  }
}

pub fn defn_to_string(d: Defn) -> String {
 match d {
  Defn::FunDefn(f, al, b) =>{
    let mut arg_str = "".to_string();
    for s in al {
      arg_str = format!("{}{}",arg_str, s)
    }
    format!("function {}({}){{{}}}", f,arg_str,expr_to_string(*b))
  }
  ,
  Defn::VarDefn(x,e)=> format!("var {} = {}",x,expr_to_string(*e)),
 }
}

pub fn e_res_to_str(r: Result<Expr,peg::error::ParseError<peg::str::LineCol>>) -> String {
  match r {
    Ok(s) => expr_to_string(s),
    Err(_s) => "err".to_string(),
  }
}
/* END STARTER CODE */

/* BEGIN ASSIGNMENT: */

/* We list how long the staff solutions are in order to help you
   find out if you are overcomplicating a problem. Yours do not
   need to be the same length as ours, nor use the same number of helpers. */
peg::parser!{
  pub grammar parser() for str {
  /* Provided helper functions to make the starter code type-check */
  pub rule unimplemented_string() -> String = empty:$("") {? Ok(empty.to_string())}
  pub rule unimplemented_expr() -> Expr = empty:$("") {? Ok(Expr::Numeral(383962395862.0)) }
  pub rule unimplemented_defn() -> Defn = empty:$("") {? Ok(Defn::VarDefn("x".to_string(), Box::new(Expr::Numeral(893.923))))}
  
 /* Parse a single variable. The cleanest solution uses id() as a helper.
    var() behaves just like id(), except with a different return type.
    Staff solution length: 1 lines */ 
  pub rule id() -> String = s:$(['a'..='z'|'A'..='Z'] (['a'..='z'|'A'..='Z'|'0'..='9'|'_'])*) { s.to_string()}
  pub rule var() -> Expr = x:id(){ Expr::Id (x)}
  /* Parse a single literal number. 
    Staff solution length: 6 lines */
    pub rule numeral() -> Expr = 
    s:$("-"? ("0" / ['1'..='9']['0'..='9']*) ("." ['0'..='9']+)?) {
      Expr::Numeral(s.parse::<f64>().unwrap())
    }
  
  /* Implement a parser for (all the) expressions. You should define
     and call your own helpers. See the precedence-climbing approach
     from the book and lecture to help decide on your helpers.
     Both expr() and defn() will call each other.
     Staff solution length: 10 lines, including 7 helpers */
     pub rule fun_call() -> Expr = f:id() _ "(" _ args:arg_list() _ ")" {
      Expr::FunCall(f, args)
  }

  pub rule atom() -> Expr = numeral() / fun_call() / var() / "(" _ expr:expr() _ ")" {
    expr
  }

  pub rule non_empty_arg_list() -> Vec<Expr> = 
  e:expr() "," _ rest:non_empty_arg_list() { 
    let mut v = vec![e]; 
    v.extend(rest); v } / e:expr() { vec![e] }

  pub rule arg_list() -> Vec<Expr> = 
  args:non_empty_arg_list() { args } / 
  "" { Vec::new() }

  pub rule op2() -> Expr = l:atom() _ "*" _ r:op2() { 
    Expr::Times(Box::new(l), Box::new(r)) 
    } / atom()

  pub rule op1() -> Expr = 
    l:op2() _ "+" _ r:op1() { Expr::Plus(Box::new(l), Box::new(r)) } / 
    l:op2() _ "-" _ r:op1() { Expr::Minus(Box::new(l), Box::new(r)) } / 
    op2()

  pub rule expr() -> Expr = 
  "let" _ d:defn() _ "in" _ e:expr() { Expr::Let(Box::new(d), Box::new(e)) } / 
  op1()
  
  /* Implement a parser for (all the) definitions.
     You are allowed to define and call your own helpers if you prefer.
     Both expr() and defn() will call each other.
     Staff solution length: 3 lines, no new helpers.
     Depending on your approach, your "expr" could be shorter and your "defn"
     could be longer, with different numbers of helpers for each.
     You are not graded on length, the goal is to communicate typical complexity
     ) */
     pub rule defn() -> Defn = 
     "var" _ v:id() _ "=" _ e:expr() { Defn::VarDefn(v, Box::new(e)) } / 
     "function" _ f:id() _ "(" _ params:param_list() _ ")" _ "{" _ body:expr() _ "}" { 
       Defn::FunDefn(f, params, Box::new(body)) 
     }
   
     pub rule non_empty_param_list() -> Vec<String> = 
     p:id() "," _ rest:non_empty_param_list() { 
       let mut v = vec![p]; 
       v.extend(rest); 
       v 
     } / 
     p:id() { vec![p] }
   
     pub rule param_list() -> Vec<String> = 
     params:non_empty_param_list() { params } / 
     "" { Vec::new() }
   
     rule _() = [' ' | '\t' | '\n']*
   
     }
   }


