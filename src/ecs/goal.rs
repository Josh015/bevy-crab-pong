use bevy::prelude::Component;

#[derive(Clone, Component, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Goal {
    Top,
    Right,
    Bottom,
    Left,
}
