//! Built-in sensor channel presets for common lab instruments.

use super::descriptor::{ChannelDesc, SampleKind};
use super::units::UnitCode;
use super::scaling::LinearScale;

#[derive(Debug, Clone, Copy)]
pub struct ChannelPreset {
    pub id: u32,
    pub name: &'static str,
    pub kind: SampleKind,
    pub unit: UnitCode,
    pub scale: LinearScale,
}

pub const PRESETS: &[ChannelPreset] = &[
    ChannelPreset {
        id: 100,
        name: "thermocouple_k-00",
        kind: SampleKind::Int32,
        unit: UnitCode::Celsius,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 101,
        name: "pressure_bar-01",
        kind: SampleKind::Float32,
        unit: UnitCode::Pascal,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 102,
        name: "flow_lpm-02",
        kind: SampleKind::Float32,
        unit: UnitCode::LitersPerMin,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 103,
        name: "ph_probe-03",
        kind: SampleKind::Float32,
        unit: UnitCode::Ph,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 104,
        name: "conductivity-04",
        kind: SampleKind::Float32,
        unit: UnitCode::SiemensPerM,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 105,
        name: "voltage_aux-05",
        kind: SampleKind::Float32,
        unit: UnitCode::Volt,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 106,
        name: "current_loop-06",
        kind: SampleKind::Float32,
        unit: UnitCode::Ampere,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 107,
        name: "digital_line-07",
        kind: SampleKind::BoolPack,
        unit: UnitCode::Dimensionless,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 108,
        name: "encoder_tick-08",
        kind: SampleKind::Int32,
        unit: UnitCode::Count,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 109,
        name: "load_cell-09",
        kind: SampleKind::Float32,
        unit: UnitCode::Newton,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 110,
        name: "thermocouple_k-10",
        kind: SampleKind::Int32,
        unit: UnitCode::Celsius,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 111,
        name: "pressure_bar-11",
        kind: SampleKind::Float32,
        unit: UnitCode::Pascal,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 112,
        name: "flow_lpm-12",
        kind: SampleKind::Float32,
        unit: UnitCode::LitersPerMin,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 113,
        name: "ph_probe-13",
        kind: SampleKind::Float32,
        unit: UnitCode::Ph,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 114,
        name: "conductivity-14",
        kind: SampleKind::Float32,
        unit: UnitCode::SiemensPerM,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 115,
        name: "voltage_aux-15",
        kind: SampleKind::Float32,
        unit: UnitCode::Volt,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 116,
        name: "current_loop-16",
        kind: SampleKind::Float32,
        unit: UnitCode::Ampere,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 117,
        name: "digital_line-17",
        kind: SampleKind::BoolPack,
        unit: UnitCode::Dimensionless,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 118,
        name: "encoder_tick-18",
        kind: SampleKind::Int32,
        unit: UnitCode::Count,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 119,
        name: "load_cell-19",
        kind: SampleKind::Float32,
        unit: UnitCode::Newton,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 120,
        name: "thermocouple_k-20",
        kind: SampleKind::Int32,
        unit: UnitCode::Celsius,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 121,
        name: "pressure_bar-21",
        kind: SampleKind::Float32,
        unit: UnitCode::Pascal,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 122,
        name: "flow_lpm-22",
        kind: SampleKind::Float32,
        unit: UnitCode::LitersPerMin,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 123,
        name: "ph_probe-23",
        kind: SampleKind::Float32,
        unit: UnitCode::Ph,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 124,
        name: "conductivity-24",
        kind: SampleKind::Float32,
        unit: UnitCode::SiemensPerM,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 125,
        name: "voltage_aux-25",
        kind: SampleKind::Float32,
        unit: UnitCode::Volt,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 126,
        name: "current_loop-26",
        kind: SampleKind::Float32,
        unit: UnitCode::Ampere,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 127,
        name: "digital_line-27",
        kind: SampleKind::BoolPack,
        unit: UnitCode::Dimensionless,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 128,
        name: "encoder_tick-28",
        kind: SampleKind::Int32,
        unit: UnitCode::Count,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 129,
        name: "load_cell-29",
        kind: SampleKind::Float32,
        unit: UnitCode::Newton,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 130,
        name: "thermocouple_k-30",
        kind: SampleKind::Int32,
        unit: UnitCode::Celsius,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 131,
        name: "pressure_bar-31",
        kind: SampleKind::Float32,
        unit: UnitCode::Pascal,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 132,
        name: "flow_lpm-32",
        kind: SampleKind::Float32,
        unit: UnitCode::LitersPerMin,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 133,
        name: "ph_probe-33",
        kind: SampleKind::Float32,
        unit: UnitCode::Ph,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 134,
        name: "conductivity-34",
        kind: SampleKind::Float32,
        unit: UnitCode::SiemensPerM,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 135,
        name: "voltage_aux-35",
        kind: SampleKind::Float32,
        unit: UnitCode::Volt,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 136,
        name: "current_loop-36",
        kind: SampleKind::Float32,
        unit: UnitCode::Ampere,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 137,
        name: "digital_line-37",
        kind: SampleKind::BoolPack,
        unit: UnitCode::Dimensionless,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 138,
        name: "encoder_tick-38",
        kind: SampleKind::Int32,
        unit: UnitCode::Count,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
    ChannelPreset {
        id: 139,
        name: "load_cell-39",
        kind: SampleKind::Float32,
        unit: UnitCode::Newton,
        scale: LinearScale { factor: 1.0, offset: 0.0 },
    },
];

pub fn lookup_preset(id: u32) -> Option<&'static ChannelPreset> {
    PRESETS.iter().find(|p| p.id == id)
}

pub fn preset_count() -> usize {
    PRESETS.len()
}

pub fn build_desc_from_preset(id: u32) -> Option<ChannelDesc> {
    lookup_preset(id).map(|p| ChannelDesc {
        id: p.id,
        kind: p.kind,
        unit: p.unit,
        scale: p.scale,
        name: alloc::string::String::from(p.name),
    })
}
