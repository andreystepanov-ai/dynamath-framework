/// Entity representation E = ⟨e⃝, S(t), R, F⟩ describing embedding, state, relations and functions.
#[derive(Debug, Clone)]
pub struct Entity {
    pub embedding: Vec<f32>,
    pub state: State,
    pub relations: Relations,
    pub functions: Vec<Operator>,
}

/// Placeholder for temporal state representation S(t).
#[derive(Debug, Clone)]
pub struct State {
    // TODO: define state fields.
}

/// Placeholder for relations graph R.
#[derive(Debug, Clone)]
pub struct Relations {
    // TODO: define relations structure.
}

/// Placeholder for operators F.
#[derive(Debug, Clone)]
pub struct Operator {
    // TODO: define operator properties.
}
