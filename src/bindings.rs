use std::collections::hash_map::{HashMap, Entry};
use syntax::ast::{Ident, Expr, Pat, Stmt, Item};
use syntax::ptr::P;
use syntax::symbol::Symbol;

use ast_equiv::AstEquiv;


#[derive(Clone, Debug)]
pub struct Bindings {
    map: HashMap<Symbol, Value>,
}

impl Bindings {
    pub fn new() -> Bindings {
        Bindings {
            map: HashMap::new(),
        }
    }

    /// Try to add a binding, mapping `sym` to `val`.  Fails if `sym` is already bound to a value
    /// and that value is not equal to `val`.
    fn try_add<S: IntoSymbol>(&mut self, sym: S, val: Value) -> bool {
        match self.map.entry(sym.into_symbol()) {
            Entry::Vacant(e) => {
                e.insert(val);
                true
            },
            Entry::Occupied(e) => {
                val.ast_equiv(e.get())
            },
        }
    }
}

macro_rules! define_binding_values {
    ($( $Thing:ident($Repr:ty),
            $add_thing:ident, $try_add_thing:ident,
            $thing:ident, $get_thing:ident; )*) => {
        #[derive(Clone, PartialEq, Eq, Debug)]
        enum Value {
            $( $Thing($Repr), )*
        }

        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Type {
            $( $Thing, )*
        }

        impl Bindings {
            $(
                pub fn $add_thing<S: IntoSymbol>(&mut self, name: S, val: $Repr) {
                    let name = name.into_symbol();
                    let ok = self.$try_add_thing(name, val);
                    assert!(ok, "cannot alter existing binding {:?}", name);
                }

                pub fn $try_add_thing<S: IntoSymbol>(&mut self, name: S, val: $Repr) -> bool {
                    self.try_add(name, Value::$Thing(val))
                }

                pub fn $thing<S: IntoSymbol>(&self, name: S) -> &$Repr {
                    let name = name.into_symbol();
                    match self.map.get(&name) {
                        Some(&Value::$Thing(ref val)) => val,
                        Some(_) => panic!(
                            concat!("found binding for {}, but its type is not ",
                                    stringify!($Thing)),
                            name),
                        None => panic!("found no binding for {}", name),
                    }
                }

                pub fn $get_thing<S: IntoSymbol>(&self, name: S) -> Option<&$Repr> {
                    let name = name.into_symbol();
                    match self.map.get(&name) {
                        Some(&Value::$Thing(ref val)) => Some(val),
                        Some(_) => None,
                        None => None,
                    }
                }
            )*

            pub fn get_type<S: IntoSymbol>(&self, name: S) -> Option<Type> {
                self.map.get(&name.into_symbol()).map(|v| {
                    match v {
                        $( &Value::$Thing(_) => Type::$Thing, )*
                    }
                })
            }
        }

        impl AstEquiv for Value {
            fn ast_equiv(&self, other: &Self) -> bool {
                match (self, other) {
                    $(
                        (&Value::$Thing(ref x1),
                         &Value::$Thing(ref x2)) => {
                            x1.ast_equiv(x2)
                        },
                    )*
                    (_, _) => false,
                }
            }
        }
    };
}

define_binding_values! {
    Ident(Ident), add_ident, try_add_ident, ident, get_ident;
    Expr(P<Expr>), add_expr, try_add_expr, expr, get_expr;
    Pat(P<Pat>), add_pat, try_add_pat, pat, get_pat;
    Stmt(Stmt), add_stmt, try_add_stmt, stmt, get_stmt;
    Item(P<Item>), add_item, try_add_item, item, get_item;
}


pub trait IntoSymbol {
    fn into_symbol(self) -> Symbol;
}

impl IntoSymbol for Symbol {
    fn into_symbol(self) -> Symbol {
        self
    }
}

impl<'a> IntoSymbol for &'a str {
    fn into_symbol(self) -> Symbol {
        Symbol::intern(self)
    }
}

impl IntoSymbol for String {
    fn into_symbol(self) -> Symbol {
        Symbol::intern(&self)
    }
}
