pub mod air;
pub mod block;
pub mod traits;

use crate::entities::{air::Air, block::Block, traits::properties};
use crate::impl_match_entity_properties;

#[derive(Debug, Clone, Copy)]
pub enum Entity {
    Air(Air),
    Block(Block),
}
impl properties::PhysicalProperties for Entity {
    fn temperature(&self) -> crate::math::units::Temperature {
        impl_match_entity_properties!(self: temperature)
    }

    fn pressure(&self) -> crate::math::units::Pressure {
        impl_match_entity_properties!(self: pressure)
    }

    fn mass(&self) -> crate::math::units::Mass {
        impl_match_entity_properties!(self: mass)
    }

    fn volume(&self) -> crate::math::units::Volume {
        impl_match_entity_properties!(self: volume)
    }

    fn get_mut_temperature(&mut self) -> &mut crate::math::units::Temperature {
        impl_match_entity_properties!(self: get_mut_temperature)
    }

    fn get_mut_pressure(&mut self) -> &mut crate::math::units::Pressure {
        impl_match_entity_properties!(self: get_mut_pressure)
    }

    fn get_mut_mass(&mut self) -> &mut crate::math::units::Mass {
        impl_match_entity_properties!(self: get_mut_mass)
    }

    fn get_mut_volume(&mut self) -> &mut crate::math::units::Volume {
        impl_match_entity_properties!(self: get_mut_volume)
    }
}

impl properties::ChemicalProperties for Entity {}

impl properties::Composition for Entity {
    fn water_fraction(&self) -> f32 {
        impl_match_entity_properties!(self: water_fraction)
    }

    fn solid_fraction(&self) -> f32 {
        impl_match_entity_properties!(self: solid_fraction)
    }

    fn gas_fraction(&self) -> f32 {
        impl_match_entity_properties!(self: gas_fraction)
    }

    fn total_mass(&self) -> crate::math::units::Mass {
        impl_match_entity_properties!(self: total_mass)
    }
}

#[macro_export]
macro_rules! impl_match_entity_properties {
    ($self:ident: $method:ident) => {
        match $self {
            Entity::Air(air) => air.$method(),
            Entity::Block(block) => block.$method(),
        }
    };
}
