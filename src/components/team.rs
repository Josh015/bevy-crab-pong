use crate::prelude::*;

#[derive(Clone, Component, Copy, Default, PartialEq, Debug)]
pub enum Team {
    #[default]
    Enemies,
    Allies,
}
