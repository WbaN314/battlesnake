use std::ops::{Deref, DerefMut};

pub enum SimulationState<T> {
    Alive(T),
    ChickenAlive(T),
    Dead,
    TimedOut,
}

impl<T> Deref for SimulationState<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            SimulationState::Alive(t) => t,
            SimulationState::ChickenAlive(t) => t,
            SimulationState::Dead => panic!("Cannot deref dead state"),
            SimulationState::TimedOut => panic!("Cannot deref timed out state"),
        }
    }
}

impl<T> DerefMut for SimulationState<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            SimulationState::Alive(t) => t,
            SimulationState::ChickenAlive(t) => t,
            SimulationState::Dead => panic!("Cannot deref dead state"),
            SimulationState::TimedOut => panic!("Cannot deref timed out state"),
        }
    }
}
