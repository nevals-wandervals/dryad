pub mod dirt;
pub mod live;
pub mod sand;
pub mod stone;
pub mod water;

use crate::impl_match_block_properties;

use super::traits::properties;

#[derive(Debug, Clone, Copy)]
pub enum Block {
    Live(live::Live),
    Dirt(dirt::Dirt),
    Stone(stone::Stone),
    Sand(sand::Sand),
    Water(water::Water),
}

impl properties::PhysicalProperties for Block {
    fn temperature(&self) -> crate::math::units::Temperature {
        impl_match_block_properties!(self: temperature)
    }

    fn pressure(&self) -> crate::math::units::Pressure {
        impl_match_block_properties!(self: pressure)
    }

    fn mass(&self) -> crate::math::units::Mass {
        impl_match_block_properties!(self: mass)
    }

    fn volume(&self) -> crate::math::units::Volume {
        impl_match_block_properties!(self: volume)
    }

    fn get_mut_temperature(&mut self) -> &mut crate::math::units::Temperature {
        impl_match_block_properties!(self: get_mut_temperature)
    }

    fn get_mut_pressure(&mut self) -> &mut crate::math::units::Pressure {
        impl_match_block_properties!(self: get_mut_pressure)
    }

    fn get_mut_mass(&mut self) -> &mut crate::math::units::Mass {
        impl_match_block_properties!(self: get_mut_mass)
    }

    fn get_mut_volume(&mut self) -> &mut crate::math::units::Volume {
        impl_match_block_properties!(self: get_mut_volume)
    }
}

impl properties::ChemicalProperties for Block {}

impl properties::Composition for Block {
    fn water_fraction(&self) -> f32 {
        impl_match_block_properties!(self: water_fraction)
    }

    fn solid_fraction(&self) -> f32 {
        impl_match_block_properties!(self: solid_fraction)
    }

    fn gas_fraction(&self) -> f32 {
        impl_match_block_properties!(self: gas_fraction)
    }

    fn total_mass(&self) -> crate::math::units::Mass {
        impl_match_block_properties!(self: total_mass)
    }
}

#[macro_export]
macro_rules! impl_match_block_properties {
    ($self:ident: $method:ident) => {
        match $self {
            Block::Live(live) => live.$method(),
            Block::Dirt(dirt) => dirt.$method(),
            Block::Stone(stone) => stone.$method(),
            Block::Sand(sand) => sand.$method(),
            Block::Water(water) => water.$method(),
        }
    };
}
