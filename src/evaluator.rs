use lexer::Token;
use parser::SExpr;
use env::EnvRef;
use env::EnvRefT;

pub fn eval(sexpr: &SExpr, env: &EnvRef) -> SExpr {
    match sexpr {
        SExpr::Atom(Token::Symbol(ref x)) => {
            let result = env.get(x)
                .expect(&format!("Unbound variable: {}", x));

            if result.is_lazy() {
                result.eval(env)
            } else {
                result
            }
        },
        SExpr::Atom(x) => {
            SExpr::Atom(x.clone())
        },
        SExpr::Procedure(x) => {
            SExpr::Procedure(x.clone())
        },
        SExpr::Unspecified => {
            SExpr::Unspecified
        },
        SExpr::Lazy(expr) => {
            expr.eval(&env)
        },
        SExpr::Pair(ref pair) => {
            // FIXME: not a correct implementation
            let head = pair.0.clone();
            let tail = pair.1.clone();

            println!("head: {} || tail {}", head, tail);

            // If the tail is also a list, then flatten the pair and eval it
            if let SExpr::List(mut xs) = tail {
                let flatten = {
                    xs.insert(0, head);
                    SExpr::List(xs)
                };
                flatten.eval(&env)
            } else {
                panic!("Can not evaluate.");
            }
        },
        SExpr::List(xs) => {
            let op = xs.get(0)
                .expect("Expected an operator, found nothing.");

            match op {
                SExpr::Atom(Token::Symbol(symbol)) => {
                    // Skip the op name
                    let args = xs[1..].to_args(&env);
                    call_procedure(symbol, args)
                },
                x => {
                    // Trying to use something other than a symbol as procedure
                    // Evaluate and see if it's a procedure.
                    let evaled = eval(x, env);
                    if let SExpr::Procedure(x) = evaled {
                        let args = xs[1..].to_args(&env);
                        x.apply(args)
                    } else {
                        panic!("Wrong type to apply: {:#?}", x)
                    }
                }
            }
        }
    }
}

pub fn call_procedure(op: &str, args: Args) -> SExpr {
    let procedure = args.env
        .get(op)
        .expect(&format!("Unbound variable: {}", op));

    fn call(proc_expr: SExpr, args: Args) -> SExpr {
        match proc_expr {
            SExpr::Procedure(proc) => proc.apply(args),
            SExpr::Lazy(p) => call(p.eval(&args.env), args),
            _ => panic!("Not a type to apply: {:#?}", proc_expr)
        }
    }

    call(procedure, args)
}

#[derive(Debug)]
pub enum Extra {
    QQLevel(usize),
    Nothing
}

#[derive(Debug)]
pub struct Args {
    pub env: EnvRef,
    pub extra: Extra,
    vec: Vec<SExpr>
}

impl Args {
    pub fn new_with_extra(vec: Vec<SExpr>, extra: Extra, env: &EnvRef) -> Args {
        Args {
            env: env.clone(),
            extra: extra,
            vec: vec
        }
    }

    pub fn new(vec: Vec<SExpr>, env: &EnvRef) -> Args {
        Args {
            env: env.clone(),
            extra: Extra::Nothing,
            vec: vec
        }
    }

    pub fn env(&self) -> EnvRef {
        self.env.clone()
    }

    pub fn into_all(self) -> Vec<SExpr> {
        self.vec
    }

    pub fn into_split(self) -> Option<(SExpr, Vec<SExpr>)> {
        let mut iter = self.vec.into_iter();
        let head = iter.next();
        let tail = iter.collect();

        if head.is_some() {
            Some((head.unwrap(), tail))
        } else {
            None
        }
    }

    pub fn get(&self, i: usize) -> Option<&SExpr> {
        self.vec.get(i)
    }

    pub fn all(&self) -> &Vec<SExpr> {
        &self.vec
    }

    // FIXME: iter -> into_iter?
    pub fn eval(&self) -> Vec<SExpr> {
        self.vec.iter()
            .map(|x| eval(&x, &self.env))
            .collect::<Vec<SExpr>>()
    }

    pub fn map<F>(mut self, mut f: F) -> Args
    where F: FnMut(SExpr) -> SExpr {
        self.vec = self.vec.into_iter()
            .map(|x| f(x))
            .collect::<Vec<SExpr>>();

        self
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}


pub trait ToArgs {
    fn to_args(&self, env: &EnvRef) -> Args;
}


impl ToArgs for [SExpr] {
    fn to_args(&self, env: &EnvRef) -> Args {
        Args::new(self.to_vec(), &env)
    }
}
