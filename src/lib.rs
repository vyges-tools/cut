// SPDX-License-Identifier: Apache-2.0
//! `vyges_cut` — the part of the design a re-synthesis works on, and its way out to and back from
//! ABC.
//!
//! A library, not a command: `vyges-rmp` links it for `restructure` and the `resynth` family. The
//! design and the timer are reached through [`design::CutDesign`] and [`facts::Facts`], so nothing
//! here needs a database to be tested.

pub mod abc_library;
pub mod blif;
pub mod design;
pub mod facts;
pub mod logic_cut;
pub mod logic_extractor;
