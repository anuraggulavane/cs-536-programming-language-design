pub mod gradelib;
//pub mod asgn2_mini;
pub mod asgn2;
use asgn2::*;
use asgn2::Expr as ex;
use asgn2::Defn as df;
//use asgn2_mini::*;

fn test_numeral(score: f64, input: String, out: Option<i32>) -> f64 {
  match (asgn2::parser::numeral(&input), out) {
    (Ok(asgn2::Expr::Numeral(f1)), Some(f2)) =>
      if f1 == f2.into()  {
        return score
      } else {
        println!("bad");
        return 0.0
      },
    (Err(_), None) => return score,
    _ => {println!("Expression test case"); return 0.0},
//    (Err(_), Some(_f)) => {println!("Expression test case "); return 0.0}
  }
}

fn test_id(score: f64, input: String, out: Option<String>) -> f64 {
  match (asgn2::parser::id(&input), out) {
    (Ok(s1), Some(s2)) =>
      if s1 == s2  {
        return score
      } else {
        println!("Expression test case {} expected id {} but got {}", input,s2,s1);
        return 0.0
      },
    (Err(_), None) => return score,
    (Ok(f), None) => {println!("Expression test case {} expected not to parse but got {}", input, f); return 0.0},
    (Err(_), Some(f)) => {println!("Expression test case {} expected {} but did not parse", input, f); return 0.0}
  }
}
fn test_expr(score: f64, input: String, out: Option<asgn2::Expr>) -> f64 {
  match (asgn2::parser::expr(&input), out) {
    (Ok(e1), Some(e2)) =>
      if asgn2::expr_eq(e1.clone(),e2.clone()) {
        return score
      } else {
        println!("Expression test case {} expected expr {} but got {}", input,expr_to_string(e2),expr_to_string(e1));
        return 0.0
      },
    (Err(_), None) => return score,
    (Ok(f), None) => {println!("Expression test case {} expected not to parse but got {}", input, expr_to_string(f)); return 0.0},
    (Err(_), Some(f)) => {println!("Expression test case {} expected {} but did not parse", input, expr_to_string(f)); return 0.0}
  }
}
fn test_defn(score: f64, input: String, out: Option<asgn2::Defn>) -> f64 {
  match (asgn2::parser::defn(&input), out) {
    (Ok(d1), Some(d2)) =>
      if asgn2::defn_eq(d1.clone(),d2.clone()) {
        return score
      } else {
        println!("Definition test case {} expected defn {} but got {}", input,defn_to_string(d2),defn_to_string(d1));
        return 0.0
      },
    (Err(_), None) => return score,
    (Ok(d), None) => {println!("Definition test case {} expected not to parse but got {}", input, defn_to_string(d)); return 0.0},
    (Err(_), Some(d)) => {println!("Definition test case {} expected {} but did not parse", input, defn_to_string(d)); 
    return 0.0}
  }
}

pub fn main() {
  //id 10
  let r1 = test_id(1.0, "a".to_string(), Some("a".to_string()));
  let r2 = test_id(1.0, "A".to_string(), Some("A".to_string()));
  let r3 = test_id(1.0, "aB".to_string(), Some("aB".to_string()));
  let r4 = test_id(1.0, "Ab".to_string(), Some("Ab".to_string()));
  let r5 = test_id(1.0, "a_2b".to_string(), Some("a_2b".to_string()));
  let r6 = test_id(1.0, "c3".to_string(), Some("c3".to_string()));
  let r7 = test_id(1.0, "d7_".to_string(), Some("d7_".to_string()));
  let r8 = test_id(1.0, "0ab".to_string(), None);
  let r9 = test_id(1.0, "_8e".to_string(), None);
  let r10 = test_id(1.0, "-29".to_string(), None);
  let r_id = r1+r2+r3+r4+r5+r6+r7+r8+r9+r10;
  //numeral 5
  //let r11 = test_numeral(1.0, "3".to_string(), Some(3));
  //let r12 = test_numeral(1.0, "-3".to_string(), Some(-3));
  //let r13 = test_numeral(1.0, "7.0".to_string(), Some(7));
  let r14 = test_numeral(1.0, "-1.000".to_string(), Some(-1));
  let r15 = test_numeral(1.0, "738".to_string(), Some(738));
  let r16 = test_numeral(1.0, "-0".to_string(), Some(0));
  //let r17 = test_numeral(1.0, "00.7".to_string(), None);
  let r18 = test_numeral(1.0, "00".to_string(), None);
  let r19 = test_numeral(1.0, "-9.".to_string(), None);
  //let r20 = test_numeral(1.0, "9.".to_string(), None);
  //let r_num = r11+r12+r13+r14+r15+r16+r17+r18+r19+r20;
  let r_num = r14+r15+r16+r18+r19;
  //expr without defn 21
  let r21 = test_expr(1.0, "xyzzy".to_string(), Some(ex::Id("xyzzy".to_string())));
  let r22 = test_expr(1.0, "234".to_string(), Some(ex::Numeral(234.0)));
  let r23 = test_expr(1.0, "x*y".to_string(), Some(ex::Times(Box::new(ex::Id("x".to_string())), Box::new(ex::Id("y".to_string())))));
  let r24 = test_expr(2.0, "1.2+z_3".to_string(), Some(ex::Plus(Box::new(ex::Numeral(1.2)),Box::new(ex::Id("z_3".to_string())))));
  let r25 = test_expr(2.0, "3-1".to_string(), Some(ex::Minus(Box::new(ex::Numeral(3.0)),Box::new(ex::Numeral(1.0)))));
  let r26 = test_expr(2.0, "(2-(y))".to_string(), Some(ex::Minus(Box::new(ex::Numeral(2.0)),Box::new(ex::Id("y".to_string())))));
  let r27 = test_expr(2.0, "((2-(y))".to_string(), None);
  let r28 = test_expr(2.0, "+32".to_string(), None);
  let r29 = test_expr(2.0, "1*2+3*4".to_string(), 
  Some(ex::Plus(Box::new(ex::Times(Box::new(ex::Numeral(1.0)),Box::new(ex::Numeral(2.0)))),
                    Box::new(ex::Times(Box::new(ex::Numeral(3.0)),Box::new(ex::Numeral(4.0)))))));
  let r30 = test_expr(2.0, "1*2-3*4".to_string(), 
  Some(ex::Minus(Box::new(ex::Times(Box::new(ex::Numeral(1.0)),Box::new(ex::Numeral(2.0)))),
                    Box::new(ex::Times(Box::new(ex::Numeral(3.0)),Box::new(ex::Numeral(4.0)))))));
  let r31 = test_expr(2.0, "-1*-2--3*-4".to_string(), 
  Some(ex::Minus(Box::new(ex::Times(Box::new(ex::Numeral(-1.0)),Box::new(ex::Numeral(-2.0)))),
                    Box::new(ex::Times(Box::new(ex::Numeral(-3.0)),Box::new(ex::Numeral(-4.0)))))));
  let r32 = test_expr(2.0, "--1".to_string(), None);
  let r_expr = r21+r22+r23+r24+r25+r26+r27+r28+r29+r30+r31+r32;


  // defns - basic
  let r33 = test_expr(3.0, "let var x = y in x".to_string(),
   Some(ex::Let(Box::new(df::VarDefn("x".to_string(), Box::new(ex::Id("y".to_string())))),Box::new(ex::Id("x".to_string())))));
  let r34a = test_expr(3.0, "let function f(x){y*x} in z".to_string(),
   Some(ex::Let(Box::new(df::FunDefn("f".to_string(), vec!["x".to_string()], 
    Box::new(ex::Times(Box::new(ex::Id("y".to_string())), Box::new(ex::Id("x".to_string())))))),
    Box::new(ex::Id("z".to_string())))));
  let _r34b = test_expr(3.0, "let function f(x){y*x} in f(2)".to_string(),
    Some(ex::Let(Box::new(df::FunDefn("f".to_string(), vec!["x".to_string()], 
     Box::new(ex::Times(Box::new(ex::Id("y".to_string())), Box::new(ex::Id("x".to_string())))))),
     Box::new(ex::FunCall("f".to_string(), vec![ex::Numeral(2.0)])))));
   let r35 = test_expr(3.0, "let var x = let var y = 1 in y in let var y = x in x".to_string(),
   Some(ex::Let(Box::new(
    df::VarDefn("x".to_string(),  
      Box::new(ex::Let(Box::new(df::VarDefn("y".to_string(),Box::new(ex::Numeral(1.0)))),
               Box::new(ex::Id("y".to_string())))))),
   Box::new(ex::Let(Box::new(df::VarDefn("y".to_string(),Box::new(ex::Id("x".to_string())))),Box::new(ex::Id("x".to_string())))))));
  let r36 = test_expr(3.0, "(let var x = y in x)+(let var x = y in x)".to_string(),
   Some(ex::Plus(Box::new(ex::Let(Box::new(df::VarDefn("x".to_string(),Box::new(ex::Id("y".to_string())))),Box::new(ex::Id("x".to_string())))),
   Box::new(ex::Let(Box::new(df::VarDefn("x".to_string(), Box::new(ex::Id("y".to_string())))),Box::new(ex::Id("x".to_string())))))));
  let r37 = test_defn(3.0, "var x = y".to_string(),
   Some(df::VarDefn("x".to_string(), Box::new(ex::Id("y".to_string())))));
  let r38 = test_defn(3.0, "function f(x){y*x}".to_string(),
   Some(df::FunDefn("f".to_string(), vec!["x".to_string()], 
    Box::new(ex::Times(Box::new(ex::Id("y".to_string())), Box::new(ex::Id("x".to_string())))))));
  let r39 = test_defn(3.0, "var x = let var y = 1 in y".to_string(),
   Some(df::VarDefn("x".to_string(),  
      Box::new(ex::Let(Box::new(df::VarDefn("y".to_string(),Box::new(ex::Numeral(1.0)))),
               Box::new(ex::Id("y".to_string())))))));

  let r_defn = r33+r34a+r35+r36+r37+r38+r39;

  // defns - advanced
  let r40 = test_defn(3.0, "function f(x+2){y*x}".to_string(),
    None);

  let r41 = test_expr(3.0, "(let var x = y in x)+(let var x = y in x)".to_string(),
    Some(ex::Plus(Box::new(ex::Let(Box::new(df::VarDefn("x".to_string(),Box::new(ex::Id("y".to_string())))),Box::new(ex::Id("x".to_string())))),
    Box::new(ex::Let(Box::new(df::VarDefn("x".to_string(), Box::new(ex::Id("y".to_string())))),Box::new(ex::Id("x".to_string())))))));
  let r42 = test_expr(3.0, "let function f(x){x*x} in let function g(y){f(y)} in g(x)".to_string(),
  Some(
    ex::Let(Box::new(
     df::FunDefn("f".to_string(), vec!["x".to_string()], 
      Box::new(ex::Times(Box::new(ex::Id("x".to_string())), Box::new(ex::Id("x".to_string()))))),
    ),
    Box::new(ex::Let(Box::new(
      df::FunDefn("g".to_string(), vec!["y".to_string()],
       Box::new(ex::FunCall("f".to_string(), vec![ex::Id("y".to_string())])))),
       Box::new(ex::FunCall("g".to_string(), vec![ex::Id("x".to_string())])))
     ))));

  let r43 = test_defn(3.0, "function f(x,y){x+y}".to_string(),
  Some(df::FunDefn("f".to_string(), vec!["x".to_string(),"y".to_string()], 
    Box::new(ex::Plus(Box::new(ex::Id("x".to_string())), Box::new(ex::Id("y".to_string())))))));
  let r44 = test_expr(3.0, "f(x,y)".to_string(),
    Some(ex::FunCall("f".to_string(), vec![ex::Id("x".to_string()),ex::Id("y".to_string())])));
  let r45 = test_expr(3.0, "f(g(),h(x+y))".to_string(),
    Some(ex::FunCall("f".to_string(), vec![
      ex::FunCall("g".to_string(),vec![]),
      ex::FunCall("h".to_string(),vec![ex::Plus(Box::new(ex::Id("x".to_string())),Box::new(ex::Id("y".to_string())))]),
      ])));
   
  // Added in 2024 to fix a bug found in the 2023 test suite,  did not 
  // adequately test param lists, especially expressions in args and  multiple args and
  let r_bugfix = r40 + r41 + r42 + r43 + r44 + r45;
  
  let r = r_id + r_num + r_expr + r_defn + r_bugfix;
  gradelib::gradelib::record_grade(r);
}