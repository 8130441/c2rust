use std::collections::hash_map::{HashMap, Entry};
use std::fmt::Debug;
use std::result;
use syntax::ast::{Ident, Expr, Pat, Stmt, Item, Crate, Mac};
use syntax::fold::{self, Folder};
use syntax::symbol::Symbol;
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;

use bindings::Bindings;
use fold::Fold;
use matcher::MatchCtxt;
use util;
use util::Lone;


struct SubstFolder<'a> {
    bindings: &'a Bindings,
}

impl<'a> Folder for SubstFolder<'a> {
    fn fold_ident(&mut self, i: Ident) -> Ident {
        // The `Ident` case is a bit different from the others.  If `fold_stmt` finds a non-`Stmt`
        // in `self.bindings`, it can ignore the problem and hope `fold_expr` or `fold_ident` will
        // find an `Expr`/`Ident` for the symbol later on.  If `fold_ident` fails, there is no
        // lower-level construct to try.  So we report an error if a binding exists at this point
        // but is not an `Ident`.

        if let Some(sym) = util::ident_sym(&i) {
            if let Some(ident) = self.bindings.get_ident(sym) {
                return ident.clone();
            } else if let Some(ty) = self.bindings.get_type(sym) {
                panic!("binding {:?} has wrong type for hole", sym);
            }
            // Otherwise, fall through
        }
        fold::noop_fold_ident(i, self)
    }

    fn fold_expr(&mut self, e: P<Expr>) -> P<Expr> {
        if let Some(expr) = util::expr_sym(&e).and_then(|sym| self.bindings.get_expr(sym)) {
            expr.clone()
        } else {
            e.map(|e| fold::noop_fold_expr(e, self))
        }
    }

    fn fold_pat(&mut self, p: P<Pat>) -> P<Pat> {
        if let Some(pat) = util::pat_sym(&p).and_then(|sym| self.bindings.get_pat(sym)) {
            pat.clone()
        } else {
            fold::noop_fold_pat(p, self)
        }
    }

    fn fold_stmt(&mut self, s: Stmt) -> SmallVector<Stmt> {
        if let Some(stmt) = util::stmt_sym(&s).and_then(|sym| self.bindings.get_stmt(sym)) {
            SmallVector::one(stmt.clone())
        } else {
            fold::noop_fold_stmt(s, self)
        }
    }

    fn fold_item(&mut self, i: P<Item>) -> SmallVector<P<Item>> {
        if let Some(item) = util::item_sym(&i).and_then(|sym| self.bindings.get_item(sym)) {
            SmallVector::one(item.clone())
        } else {
            fold::noop_fold_item(i, self)
        }
    }
}


pub trait Subst {
    fn subst(self, bindings: &Bindings) -> Self;
}

macro_rules! subst_impl {
    ($ty:ty, $fold_func:ident) => {
        impl Subst for $ty {
            fn subst(self, bindings: &Bindings) -> Self {
                let mut f = SubstFolder { bindings: bindings };
                let result = self.fold(&mut f);
                result.lone()
            }
        }
    };
}

subst_impl!(Ident, fold_ident);
subst_impl!(P<Expr>, fold_expr);
subst_impl!(P<Pat>, fold_pat);
subst_impl!(Stmt, fold_stmt);
subst_impl!(P<Item>, fold_item);

impl<T: Subst> Subst for Vec<T> {
    fn subst(self, bindings: &Bindings) -> Vec<T> {
        self.into_iter().map(|x| x.subst(bindings)).collect()
    }
}
