#![allow(non_snake_case)]

mod footer;
mod form;
mod navbar;
mod select_days;
mod solution;
mod solver;

pub use self::{
    footer::Footer, form::SolverForm, navbar::Navbar, select_days::SelectDays, solution::Solution,
    solver::Solver,
};
