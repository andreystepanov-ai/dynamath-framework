//! Basic types for Dynamath: Entity, State, Relations, Operator.

pub struct Entity {
    pub embedding: Vec<f32>,
    pub state: State,
    pub relations: Relations,
    pub operators: Vec<Operator>,
}

pub struct State {
    // TODO: fields for dynamic state
}

pub struct Relations {
    // TODO: describe relations between entities
}

pub struct Operator {
    // TODO: define operator metadata
}
