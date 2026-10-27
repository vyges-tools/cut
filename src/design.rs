// SPDX-License-Identifier: Apache-2.0
//! The design as a re-synthesis edits it.

use crate::blif::BlifDesign;

/// What a re-synthesis asks of the design beyond the BLIF reader's questions.
pub trait CutDesign: BlifDesign {
    /// `dbNet::getId`.
    fn net_id(&self, net: &str) -> u32;
    /// The net a top-level port is on.
    fn bterm_net(&self, bterm: &str) -> Option<String>;
    /// `dbInst::isDoNotTouch` / `dbNet::isDoNotTouch`.
    fn inst_dont_touch(&self, inst: &str) -> bool;
    fn net_dont_touch(&self, net: &str) -> bool;
    /// `dbBlock::makeNewNetName(top, base, ALWAYS)`: `base` with the block's next free unique net
    /// index appended (the index is the database's own and advances).
    fn new_net_name(&mut self, base: &str) -> Result<String, String>;
}
