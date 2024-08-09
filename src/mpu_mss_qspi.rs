#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    revision: Revision,
    configuration: Configuration,
    _reserved2: [u8; 0x08],
    interrupt_raw_status_set: InterruptRawStatusSet,
    interrupt_enabled_status_clear: InterruptEnabledStatusClear,
    interrupt_enable: InterruptEnable,
    interrupt_enable_clear: InterruptEnableClear,
    eoi: Eoi,
    interrupt_vector: InterruptVector,
    _reserved8: [u8; 0xd8],
    fixed_start_address: FixedStartAddress,
    fixed_end_address: FixedEndAddress,
    fixed_mppa: FixedMppa,
    _reserved11: [u8; 0xf4],
    programmable1start_address: Programmable1startAddress,
    programmable1end_address: Programmable1endAddress,
    programmable1mppa: Programmable1mppa,
    _reserved14: [u8; 0x04],
    programmable2start_address: Programmable2startAddress,
    programmable2end_address: Programmable2endAddress,
    programmable2mppa: Programmable2mppa,
    _reserved17: [u8; 0x04],
    programmable3start_address: Programmable3startAddress,
    programmable3end_address: Programmable3endAddress,
    programmable3mppa: Programmable3mppa,
    _reserved20: [u8; 0x04],
    programmable4start_address: Programmable4startAddress,
    programmable4end_address: Programmable4endAddress,
    programmable4mppa: Programmable4mppa,
    _reserved23: [u8; 0x04],
    programmable5start_address: Programmable5startAddress,
    programmable5end_address: Programmable5endAddress,
    programmable5mppa: Programmable5mppa,
    _reserved26: [u8; 0x04],
    programmable6start_address: Programmable6startAddress,
    programmable6end_address: Programmable6endAddress,
    programmable6mppa: Programmable6mppa,
    _reserved29: [u8; 0x04],
    programmable7start_address: Programmable7startAddress,
    programmable7end_address: Programmable7endAddress,
    programmable7mppa: Programmable7mppa,
    _reserved32: [u8; 0x04],
    programmable8start_address: Programmable8startAddress,
    programmable8end_address: Programmable8endAddress,
    programmable8mppa: Programmable8mppa,
    _reserved35: [u8; 0x04],
    programmable9start_address: Programmable9startAddress,
    programmable9end_address: Programmable9endAddress,
    programmable9mppa: Programmable9mppa,
    _reserved38: [u8; 0x04],
    programmable10start_address: Programmable10startAddress,
    programmable10end_address: Programmable10endAddress,
    programmable10mppa: Programmable10mppa,
    _reserved41: [u8; 0x04],
    programmable11start_address: Programmable11startAddress,
    programmable11end_address: Programmable11endAddress,
    programmable11mppa: Programmable11mppa,
    _reserved44: [u8; 0x04],
    programmable12start_address: Programmable12startAddress,
    programmable12end_address: Programmable12endAddress,
    programmable12mppa: Programmable12mppa,
    _reserved47: [u8; 0x04],
    programmable13start_address: Programmable13startAddress,
    programmable13end_address: Programmable13endAddress,
    programmable13mppa: Programmable13mppa,
    _reserved50: [u8; 0x04],
    programmable14start_address: Programmable14startAddress,
    programmable14end_address: Programmable14endAddress,
    programmable14mppa: Programmable14mppa,
    _reserved53: [u8; 0x04],
    programmable15start_address: Programmable15startAddress,
    programmable15end_address: Programmable15endAddress,
    programmable15mppa: Programmable15mppa,
    _reserved56: [u8; 0x04],
    programmable16start_address: Programmable16startAddress,
    programmable16end_address: Programmable16endAddress,
    programmable16mppa: Programmable16mppa,
    _reserved59: [u8; 0x04],
    fault_address: FaultAddress,
    fault_status: FaultStatus,
    fault_clear: FaultClear,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision"]
    #[inline(always)]
    pub const fn revision(&self) -> &Revision {
        &self.revision
    }
    #[doc = "0x04 - Configuration"]
    #[inline(always)]
    pub const fn configuration(&self) -> &Configuration {
        &self.configuration
    }
    #[doc = "0x10 - Interrupt Raw Status/Set"]
    #[inline(always)]
    pub const fn interrupt_raw_status_set(&self) -> &InterruptRawStatusSet {
        &self.interrupt_raw_status_set
    }
    #[doc = "0x14 - Interrupt Enabled Status/Clear"]
    #[inline(always)]
    pub const fn interrupt_enabled_status_clear(&self) -> &InterruptEnabledStatusClear {
        &self.interrupt_enabled_status_clear
    }
    #[doc = "0x18 - Interrupt Enable"]
    #[inline(always)]
    pub const fn interrupt_enable(&self) -> &InterruptEnable {
        &self.interrupt_enable
    }
    #[doc = "0x1c - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn interrupt_enable_clear(&self) -> &InterruptEnableClear {
        &self.interrupt_enable_clear
    }
    #[doc = "0x20 - EOI"]
    #[inline(always)]
    pub const fn eoi(&self) -> &Eoi {
        &self.eoi
    }
    #[doc = "0x24 - Interrupt Vector"]
    #[inline(always)]
    pub const fn interrupt_vector(&self) -> &InterruptVector {
        &self.interrupt_vector
    }
    #[doc = "0x100 - Fixed Start Address"]
    #[inline(always)]
    pub const fn fixed_start_address(&self) -> &FixedStartAddress {
        &self.fixed_start_address
    }
    #[doc = "0x104 - Fixed End Address"]
    #[inline(always)]
    pub const fn fixed_end_address(&self) -> &FixedEndAddress {
        &self.fixed_end_address
    }
    #[doc = "0x108 - Fixed MPPA"]
    #[inline(always)]
    pub const fn fixed_mppa(&self) -> &FixedMppa {
        &self.fixed_mppa
    }
    #[doc = "0x200 - Programmable 1 Start Address"]
    #[inline(always)]
    pub const fn programmable1start_address(&self) -> &Programmable1startAddress {
        &self.programmable1start_address
    }
    #[doc = "0x204 - Programmable 1 End Address"]
    #[inline(always)]
    pub const fn programmable1end_address(&self) -> &Programmable1endAddress {
        &self.programmable1end_address
    }
    #[doc = "0x208 - Programmable 1 MPPA"]
    #[inline(always)]
    pub const fn programmable1mppa(&self) -> &Programmable1mppa {
        &self.programmable1mppa
    }
    #[doc = "0x210 - Programmable 2 Start Address"]
    #[inline(always)]
    pub const fn programmable2start_address(&self) -> &Programmable2startAddress {
        &self.programmable2start_address
    }
    #[doc = "0x214 - Programmable 2 End Address"]
    #[inline(always)]
    pub const fn programmable2end_address(&self) -> &Programmable2endAddress {
        &self.programmable2end_address
    }
    #[doc = "0x218 - Programmable 2 MPPA"]
    #[inline(always)]
    pub const fn programmable2mppa(&self) -> &Programmable2mppa {
        &self.programmable2mppa
    }
    #[doc = "0x220 - Programmable 3 Start Address"]
    #[inline(always)]
    pub const fn programmable3start_address(&self) -> &Programmable3startAddress {
        &self.programmable3start_address
    }
    #[doc = "0x224 - Programmable 3 End Address"]
    #[inline(always)]
    pub const fn programmable3end_address(&self) -> &Programmable3endAddress {
        &self.programmable3end_address
    }
    #[doc = "0x228 - Programmable 3 MPPA"]
    #[inline(always)]
    pub const fn programmable3mppa(&self) -> &Programmable3mppa {
        &self.programmable3mppa
    }
    #[doc = "0x230 - Programmable 4 Start Address"]
    #[inline(always)]
    pub const fn programmable4start_address(&self) -> &Programmable4startAddress {
        &self.programmable4start_address
    }
    #[doc = "0x234 - Programmable 4 End Address"]
    #[inline(always)]
    pub const fn programmable4end_address(&self) -> &Programmable4endAddress {
        &self.programmable4end_address
    }
    #[doc = "0x238 - Programmable 4 MPPA"]
    #[inline(always)]
    pub const fn programmable4mppa(&self) -> &Programmable4mppa {
        &self.programmable4mppa
    }
    #[doc = "0x240 - Programmable 5 Start Address"]
    #[inline(always)]
    pub const fn programmable5start_address(&self) -> &Programmable5startAddress {
        &self.programmable5start_address
    }
    #[doc = "0x244 - Programmable 5 End Address"]
    #[inline(always)]
    pub const fn programmable5end_address(&self) -> &Programmable5endAddress {
        &self.programmable5end_address
    }
    #[doc = "0x248 - Programmable 5 MPPA"]
    #[inline(always)]
    pub const fn programmable5mppa(&self) -> &Programmable5mppa {
        &self.programmable5mppa
    }
    #[doc = "0x250 - Programmable 6 Start Address"]
    #[inline(always)]
    pub const fn programmable6start_address(&self) -> &Programmable6startAddress {
        &self.programmable6start_address
    }
    #[doc = "0x254 - Programmable 6 End Address"]
    #[inline(always)]
    pub const fn programmable6end_address(&self) -> &Programmable6endAddress {
        &self.programmable6end_address
    }
    #[doc = "0x258 - Programmable 6 MPPA"]
    #[inline(always)]
    pub const fn programmable6mppa(&self) -> &Programmable6mppa {
        &self.programmable6mppa
    }
    #[doc = "0x260 - Programmable 7 Start Address"]
    #[inline(always)]
    pub const fn programmable7start_address(&self) -> &Programmable7startAddress {
        &self.programmable7start_address
    }
    #[doc = "0x264 - Programmable 7 End Address"]
    #[inline(always)]
    pub const fn programmable7end_address(&self) -> &Programmable7endAddress {
        &self.programmable7end_address
    }
    #[doc = "0x268 - Programmable 7 MPPA"]
    #[inline(always)]
    pub const fn programmable7mppa(&self) -> &Programmable7mppa {
        &self.programmable7mppa
    }
    #[doc = "0x270 - Programmable 8 Start Address"]
    #[inline(always)]
    pub const fn programmable8start_address(&self) -> &Programmable8startAddress {
        &self.programmable8start_address
    }
    #[doc = "0x274 - Programmable 8 End Address"]
    #[inline(always)]
    pub const fn programmable8end_address(&self) -> &Programmable8endAddress {
        &self.programmable8end_address
    }
    #[doc = "0x278 - Programmable 8 MPPA"]
    #[inline(always)]
    pub const fn programmable8mppa(&self) -> &Programmable8mppa {
        &self.programmable8mppa
    }
    #[doc = "0x280 - Programmable 9 Start Address"]
    #[inline(always)]
    pub const fn programmable9start_address(&self) -> &Programmable9startAddress {
        &self.programmable9start_address
    }
    #[doc = "0x284 - Programmable 9 End Address"]
    #[inline(always)]
    pub const fn programmable9end_address(&self) -> &Programmable9endAddress {
        &self.programmable9end_address
    }
    #[doc = "0x288 - Programmable 9 MPPA"]
    #[inline(always)]
    pub const fn programmable9mppa(&self) -> &Programmable9mppa {
        &self.programmable9mppa
    }
    #[doc = "0x290 - Programmable 10 Start Address"]
    #[inline(always)]
    pub const fn programmable10start_address(&self) -> &Programmable10startAddress {
        &self.programmable10start_address
    }
    #[doc = "0x294 - Programmable 10 End Address"]
    #[inline(always)]
    pub const fn programmable10end_address(&self) -> &Programmable10endAddress {
        &self.programmable10end_address
    }
    #[doc = "0x298 - Programmable 10 MPPA"]
    #[inline(always)]
    pub const fn programmable10mppa(&self) -> &Programmable10mppa {
        &self.programmable10mppa
    }
    #[doc = "0x2a0 - Programmable 11 Start Address"]
    #[inline(always)]
    pub const fn programmable11start_address(&self) -> &Programmable11startAddress {
        &self.programmable11start_address
    }
    #[doc = "0x2a4 - Programmable 11 End Address"]
    #[inline(always)]
    pub const fn programmable11end_address(&self) -> &Programmable11endAddress {
        &self.programmable11end_address
    }
    #[doc = "0x2a8 - Programmable 11 MPPA"]
    #[inline(always)]
    pub const fn programmable11mppa(&self) -> &Programmable11mppa {
        &self.programmable11mppa
    }
    #[doc = "0x2b0 - Programmable 12 Start Address"]
    #[inline(always)]
    pub const fn programmable12start_address(&self) -> &Programmable12startAddress {
        &self.programmable12start_address
    }
    #[doc = "0x2b4 - Programmable 12 End Address"]
    #[inline(always)]
    pub const fn programmable12end_address(&self) -> &Programmable12endAddress {
        &self.programmable12end_address
    }
    #[doc = "0x2b8 - Programmable 12 MPPA"]
    #[inline(always)]
    pub const fn programmable12mppa(&self) -> &Programmable12mppa {
        &self.programmable12mppa
    }
    #[doc = "0x2c0 - Programmable 13 Start Address"]
    #[inline(always)]
    pub const fn programmable13start_address(&self) -> &Programmable13startAddress {
        &self.programmable13start_address
    }
    #[doc = "0x2c4 - Programmable 13 End Address"]
    #[inline(always)]
    pub const fn programmable13end_address(&self) -> &Programmable13endAddress {
        &self.programmable13end_address
    }
    #[doc = "0x2c8 - Programmable 13 MPPA"]
    #[inline(always)]
    pub const fn programmable13mppa(&self) -> &Programmable13mppa {
        &self.programmable13mppa
    }
    #[doc = "0x2d0 - Programmable 14 Start Address"]
    #[inline(always)]
    pub const fn programmable14start_address(&self) -> &Programmable14startAddress {
        &self.programmable14start_address
    }
    #[doc = "0x2d4 - Programmable 14 End Address"]
    #[inline(always)]
    pub const fn programmable14end_address(&self) -> &Programmable14endAddress {
        &self.programmable14end_address
    }
    #[doc = "0x2d8 - Programmable 14 MPPA"]
    #[inline(always)]
    pub const fn programmable14mppa(&self) -> &Programmable14mppa {
        &self.programmable14mppa
    }
    #[doc = "0x2e0 - Programmable 15 Start Address"]
    #[inline(always)]
    pub const fn programmable15start_address(&self) -> &Programmable15startAddress {
        &self.programmable15start_address
    }
    #[doc = "0x2e4 - Programmable 15 End Address"]
    #[inline(always)]
    pub const fn programmable15end_address(&self) -> &Programmable15endAddress {
        &self.programmable15end_address
    }
    #[doc = "0x2e8 - Programmable 15 MPPA"]
    #[inline(always)]
    pub const fn programmable15mppa(&self) -> &Programmable15mppa {
        &self.programmable15mppa
    }
    #[doc = "0x2f0 - Programmable 16 Start Address"]
    #[inline(always)]
    pub const fn programmable16start_address(&self) -> &Programmable16startAddress {
        &self.programmable16start_address
    }
    #[doc = "0x2f4 - Programmable 16 End Address"]
    #[inline(always)]
    pub const fn programmable16end_address(&self) -> &Programmable16endAddress {
        &self.programmable16end_address
    }
    #[doc = "0x2f8 - Programmable 16 MPPA"]
    #[inline(always)]
    pub const fn programmable16mppa(&self) -> &Programmable16mppa {
        &self.programmable16mppa
    }
    #[doc = "0x300 - Fault Address"]
    #[inline(always)]
    pub const fn fault_address(&self) -> &FaultAddress {
        &self.fault_address
    }
    #[doc = "0x304 - Fault Status"]
    #[inline(always)]
    pub const fn fault_status(&self) -> &FaultStatus {
        &self.fault_status
    }
    #[doc = "0x308 - Fault Clear"]
    #[inline(always)]
    pub const fn fault_clear(&self) -> &FaultClear {
        &self.fault_clear
    }
}
#[doc = "Revision (rw) register accessor: Revision\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`revision::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision`]
module"]
pub type Revision = crate::Reg<revision::RevisionSpec>;
#[doc = "Revision"]
pub mod revision;
#[doc = "Configuration (rw) register accessor: Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`configuration::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`configuration::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@configuration`]
module"]
pub type Configuration = crate::Reg<configuration::ConfigurationSpec>;
#[doc = "Configuration"]
pub mod configuration;
#[doc = "Interrupt Raw Status/Set (rw) register accessor: Interrupt Raw Status/Set\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_raw_status_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_raw_status_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_raw_status_set`]
module"]
#[doc(alias = "Interrupt Raw Status/Set")]
pub type InterruptRawStatusSet = crate::Reg<interrupt_raw_status_set::InterruptRawStatusSetSpec>;
#[doc = "Interrupt Raw Status/Set"]
pub mod interrupt_raw_status_set;
#[doc = "Interrupt Enabled Status/Clear (rw) register accessor: Interrupt Enabled Status/Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enabled_status_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enabled_status_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_enabled_status_clear`]
module"]
#[doc(alias = "Interrupt Enabled Status/Clear")]
pub type InterruptEnabledStatusClear =
    crate::Reg<interrupt_enabled_status_clear::InterruptEnabledStatusClearSpec>;
#[doc = "Interrupt Enabled Status/Clear"]
pub mod interrupt_enabled_status_clear;
#[doc = "Interrupt Enable (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_enable`]
module"]
#[doc(alias = "Interrupt Enable")]
pub type InterruptEnable = crate::Reg<interrupt_enable::InterruptEnableSpec>;
#[doc = "Interrupt Enable"]
pub mod interrupt_enable;
#[doc = "Interrupt Enable Clear (rw) register accessor: Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enable_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enable_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_enable_clear`]
module"]
#[doc(alias = "Interrupt Enable Clear")]
pub type InterruptEnableClear = crate::Reg<interrupt_enable_clear::InterruptEnableClearSpec>;
#[doc = "Interrupt Enable Clear"]
pub mod interrupt_enable_clear;
#[doc = "EOI (rw) register accessor: EOI\n\nYou can [`read`](crate::Reg::read) this register and get [`eoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eoi`]
module"]
#[doc(alias = "EOI")]
pub type Eoi = crate::Reg<eoi::EoiSpec>;
#[doc = "EOI"]
pub mod eoi;
#[doc = "Interrupt Vector (rw) register accessor: Interrupt Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_vector::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_vector::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_vector`]
module"]
#[doc(alias = "Interrupt Vector")]
pub type InterruptVector = crate::Reg<interrupt_vector::InterruptVectorSpec>;
#[doc = "Interrupt Vector"]
pub mod interrupt_vector;
#[doc = "Fixed Start Address (rw) register accessor: Fixed Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`fixed_start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fixed_start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fixed_start_address`]
module"]
#[doc(alias = "Fixed Start Address")]
pub type FixedStartAddress = crate::Reg<fixed_start_address::FixedStartAddressSpec>;
#[doc = "Fixed Start Address"]
pub mod fixed_start_address;
#[doc = "Fixed End Address (rw) register accessor: Fixed End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`fixed_end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fixed_end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fixed_end_address`]
module"]
#[doc(alias = "Fixed End Address")]
pub type FixedEndAddress = crate::Reg<fixed_end_address::FixedEndAddressSpec>;
#[doc = "Fixed End Address"]
pub mod fixed_end_address;
#[doc = "Fixed MPPA (rw) register accessor: Fixed MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`fixed_mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fixed_mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fixed_mppa`]
module"]
#[doc(alias = "Fixed MPPA")]
pub type FixedMppa = crate::Reg<fixed_mppa::FixedMppaSpec>;
#[doc = "Fixed MPPA"]
pub mod fixed_mppa;
#[doc = "Programmable 1 Start Address (rw) register accessor: Programmable 1 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable1start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable1start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable1start_address`]
module"]
#[doc(alias = "Programmable 1 Start Address")]
pub type Programmable1startAddress =
    crate::Reg<programmable1start_address::Programmable1startAddressSpec>;
#[doc = "Programmable 1 Start Address"]
pub mod programmable1start_address;
#[doc = "Programmable 1 End Address (rw) register accessor: Programmable 1 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable1end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable1end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable1end_address`]
module"]
#[doc(alias = "Programmable 1 End Address")]
pub type Programmable1endAddress =
    crate::Reg<programmable1end_address::Programmable1endAddressSpec>;
#[doc = "Programmable 1 End Address"]
pub mod programmable1end_address;
#[doc = "Programmable 1 MPPA (rw) register accessor: Programmable 1 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable1mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable1mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable1mppa`]
module"]
#[doc(alias = "Programmable 1 MPPA")]
pub type Programmable1mppa = crate::Reg<programmable1mppa::Programmable1mppaSpec>;
#[doc = "Programmable 1 MPPA"]
pub mod programmable1mppa;
#[doc = "Programmable 2 Start Address (rw) register accessor: Programmable 2 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable2start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable2start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable2start_address`]
module"]
#[doc(alias = "Programmable 2 Start Address")]
pub type Programmable2startAddress =
    crate::Reg<programmable2start_address::Programmable2startAddressSpec>;
#[doc = "Programmable 2 Start Address"]
pub mod programmable2start_address;
#[doc = "Programmable 2 End Address (rw) register accessor: Programmable 2 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable2end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable2end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable2end_address`]
module"]
#[doc(alias = "Programmable 2 End Address")]
pub type Programmable2endAddress =
    crate::Reg<programmable2end_address::Programmable2endAddressSpec>;
#[doc = "Programmable 2 End Address"]
pub mod programmable2end_address;
#[doc = "Programmable 2 MPPA (rw) register accessor: Programmable 2 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable2mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable2mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable2mppa`]
module"]
#[doc(alias = "Programmable 2 MPPA")]
pub type Programmable2mppa = crate::Reg<programmable2mppa::Programmable2mppaSpec>;
#[doc = "Programmable 2 MPPA"]
pub mod programmable2mppa;
#[doc = "Programmable 3 Start Address (rw) register accessor: Programmable 3 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable3start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable3start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable3start_address`]
module"]
#[doc(alias = "Programmable 3 Start Address")]
pub type Programmable3startAddress =
    crate::Reg<programmable3start_address::Programmable3startAddressSpec>;
#[doc = "Programmable 3 Start Address"]
pub mod programmable3start_address;
#[doc = "Programmable 3 End Address (rw) register accessor: Programmable 3 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable3end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable3end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable3end_address`]
module"]
#[doc(alias = "Programmable 3 End Address")]
pub type Programmable3endAddress =
    crate::Reg<programmable3end_address::Programmable3endAddressSpec>;
#[doc = "Programmable 3 End Address"]
pub mod programmable3end_address;
#[doc = "Programmable 3 MPPA (rw) register accessor: Programmable 3 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable3mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable3mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable3mppa`]
module"]
#[doc(alias = "Programmable 3 MPPA")]
pub type Programmable3mppa = crate::Reg<programmable3mppa::Programmable3mppaSpec>;
#[doc = "Programmable 3 MPPA"]
pub mod programmable3mppa;
#[doc = "Programmable 4 Start Address (rw) register accessor: Programmable 4 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable4start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable4start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable4start_address`]
module"]
#[doc(alias = "Programmable 4 Start Address")]
pub type Programmable4startAddress =
    crate::Reg<programmable4start_address::Programmable4startAddressSpec>;
#[doc = "Programmable 4 Start Address"]
pub mod programmable4start_address;
#[doc = "Programmable 4 End Address (rw) register accessor: Programmable 4 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable4end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable4end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable4end_address`]
module"]
#[doc(alias = "Programmable 4 End Address")]
pub type Programmable4endAddress =
    crate::Reg<programmable4end_address::Programmable4endAddressSpec>;
#[doc = "Programmable 4 End Address"]
pub mod programmable4end_address;
#[doc = "Programmable 4 MPPA (rw) register accessor: Programmable 4 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable4mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable4mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable4mppa`]
module"]
#[doc(alias = "Programmable 4 MPPA")]
pub type Programmable4mppa = crate::Reg<programmable4mppa::Programmable4mppaSpec>;
#[doc = "Programmable 4 MPPA"]
pub mod programmable4mppa;
#[doc = "Programmable 5 Start Address (rw) register accessor: Programmable 5 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable5start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable5start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable5start_address`]
module"]
#[doc(alias = "Programmable 5 Start Address")]
pub type Programmable5startAddress =
    crate::Reg<programmable5start_address::Programmable5startAddressSpec>;
#[doc = "Programmable 5 Start Address"]
pub mod programmable5start_address;
#[doc = "Programmable 5 End Address (rw) register accessor: Programmable 5 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable5end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable5end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable5end_address`]
module"]
#[doc(alias = "Programmable 5 End Address")]
pub type Programmable5endAddress =
    crate::Reg<programmable5end_address::Programmable5endAddressSpec>;
#[doc = "Programmable 5 End Address"]
pub mod programmable5end_address;
#[doc = "Programmable 5 MPPA (rw) register accessor: Programmable 5 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable5mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable5mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable5mppa`]
module"]
#[doc(alias = "Programmable 5 MPPA")]
pub type Programmable5mppa = crate::Reg<programmable5mppa::Programmable5mppaSpec>;
#[doc = "Programmable 5 MPPA"]
pub mod programmable5mppa;
#[doc = "Programmable 6 Start Address (rw) register accessor: Programmable 6 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable6start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable6start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable6start_address`]
module"]
#[doc(alias = "Programmable 6 Start Address")]
pub type Programmable6startAddress =
    crate::Reg<programmable6start_address::Programmable6startAddressSpec>;
#[doc = "Programmable 6 Start Address"]
pub mod programmable6start_address;
#[doc = "Programmable 6 End Address (rw) register accessor: Programmable 6 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable6end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable6end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable6end_address`]
module"]
#[doc(alias = "Programmable 6 End Address")]
pub type Programmable6endAddress =
    crate::Reg<programmable6end_address::Programmable6endAddressSpec>;
#[doc = "Programmable 6 End Address"]
pub mod programmable6end_address;
#[doc = "Programmable 6 MPPA (rw) register accessor: Programmable 6 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable6mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable6mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable6mppa`]
module"]
#[doc(alias = "Programmable 6 MPPA")]
pub type Programmable6mppa = crate::Reg<programmable6mppa::Programmable6mppaSpec>;
#[doc = "Programmable 6 MPPA"]
pub mod programmable6mppa;
#[doc = "Programmable 7 Start Address (rw) register accessor: Programmable 7 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable7start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable7start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable7start_address`]
module"]
#[doc(alias = "Programmable 7 Start Address")]
pub type Programmable7startAddress =
    crate::Reg<programmable7start_address::Programmable7startAddressSpec>;
#[doc = "Programmable 7 Start Address"]
pub mod programmable7start_address;
#[doc = "Programmable 7 End Address (rw) register accessor: Programmable 7 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable7end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable7end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable7end_address`]
module"]
#[doc(alias = "Programmable 7 End Address")]
pub type Programmable7endAddress =
    crate::Reg<programmable7end_address::Programmable7endAddressSpec>;
#[doc = "Programmable 7 End Address"]
pub mod programmable7end_address;
#[doc = "Programmable 7 MPPA (rw) register accessor: Programmable 7 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable7mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable7mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable7mppa`]
module"]
#[doc(alias = "Programmable 7 MPPA")]
pub type Programmable7mppa = crate::Reg<programmable7mppa::Programmable7mppaSpec>;
#[doc = "Programmable 7 MPPA"]
pub mod programmable7mppa;
#[doc = "Programmable 8 Start Address (rw) register accessor: Programmable 8 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable8start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable8start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable8start_address`]
module"]
#[doc(alias = "Programmable 8 Start Address")]
pub type Programmable8startAddress =
    crate::Reg<programmable8start_address::Programmable8startAddressSpec>;
#[doc = "Programmable 8 Start Address"]
pub mod programmable8start_address;
#[doc = "Programmable 8 End Address (rw) register accessor: Programmable 8 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable8end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable8end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable8end_address`]
module"]
#[doc(alias = "Programmable 8 End Address")]
pub type Programmable8endAddress =
    crate::Reg<programmable8end_address::Programmable8endAddressSpec>;
#[doc = "Programmable 8 End Address"]
pub mod programmable8end_address;
#[doc = "Programmable 8 MPPA (rw) register accessor: Programmable 8 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable8mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable8mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable8mppa`]
module"]
#[doc(alias = "Programmable 8 MPPA")]
pub type Programmable8mppa = crate::Reg<programmable8mppa::Programmable8mppaSpec>;
#[doc = "Programmable 8 MPPA"]
pub mod programmable8mppa;
#[doc = "Programmable 9 Start Address (rw) register accessor: Programmable 9 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable9start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable9start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable9start_address`]
module"]
#[doc(alias = "Programmable 9 Start Address")]
pub type Programmable9startAddress =
    crate::Reg<programmable9start_address::Programmable9startAddressSpec>;
#[doc = "Programmable 9 Start Address"]
pub mod programmable9start_address;
#[doc = "Programmable 9 End Address (rw) register accessor: Programmable 9 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable9end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable9end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable9end_address`]
module"]
#[doc(alias = "Programmable 9 End Address")]
pub type Programmable9endAddress =
    crate::Reg<programmable9end_address::Programmable9endAddressSpec>;
#[doc = "Programmable 9 End Address"]
pub mod programmable9end_address;
#[doc = "Programmable 9 MPPA (rw) register accessor: Programmable 9 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable9mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable9mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable9mppa`]
module"]
#[doc(alias = "Programmable 9 MPPA")]
pub type Programmable9mppa = crate::Reg<programmable9mppa::Programmable9mppaSpec>;
#[doc = "Programmable 9 MPPA"]
pub mod programmable9mppa;
#[doc = "Programmable 10 Start Address (rw) register accessor: Programmable 10 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable10start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable10start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable10start_address`]
module"]
#[doc(alias = "Programmable 10 Start Address")]
pub type Programmable10startAddress =
    crate::Reg<programmable10start_address::Programmable10startAddressSpec>;
#[doc = "Programmable 10 Start Address"]
pub mod programmable10start_address;
#[doc = "Programmable 10 End Address (rw) register accessor: Programmable 10 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable10end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable10end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable10end_address`]
module"]
#[doc(alias = "Programmable 10 End Address")]
pub type Programmable10endAddress =
    crate::Reg<programmable10end_address::Programmable10endAddressSpec>;
#[doc = "Programmable 10 End Address"]
pub mod programmable10end_address;
#[doc = "Programmable 10 MPPA (rw) register accessor: Programmable 10 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable10mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable10mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable10mppa`]
module"]
#[doc(alias = "Programmable 10 MPPA")]
pub type Programmable10mppa = crate::Reg<programmable10mppa::Programmable10mppaSpec>;
#[doc = "Programmable 10 MPPA"]
pub mod programmable10mppa;
#[doc = "Programmable 11 Start Address (rw) register accessor: Programmable 11 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable11start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable11start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable11start_address`]
module"]
#[doc(alias = "Programmable 11 Start Address")]
pub type Programmable11startAddress =
    crate::Reg<programmable11start_address::Programmable11startAddressSpec>;
#[doc = "Programmable 11 Start Address"]
pub mod programmable11start_address;
#[doc = "Programmable 11 End Address (rw) register accessor: Programmable 11 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable11end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable11end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable11end_address`]
module"]
#[doc(alias = "Programmable 11 End Address")]
pub type Programmable11endAddress =
    crate::Reg<programmable11end_address::Programmable11endAddressSpec>;
#[doc = "Programmable 11 End Address"]
pub mod programmable11end_address;
#[doc = "Programmable 11 MPPA (rw) register accessor: Programmable 11 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable11mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable11mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable11mppa`]
module"]
#[doc(alias = "Programmable 11 MPPA")]
pub type Programmable11mppa = crate::Reg<programmable11mppa::Programmable11mppaSpec>;
#[doc = "Programmable 11 MPPA"]
pub mod programmable11mppa;
#[doc = "Programmable 12 Start Address (rw) register accessor: Programmable 12 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable12start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable12start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable12start_address`]
module"]
#[doc(alias = "Programmable 12 Start Address")]
pub type Programmable12startAddress =
    crate::Reg<programmable12start_address::Programmable12startAddressSpec>;
#[doc = "Programmable 12 Start Address"]
pub mod programmable12start_address;
#[doc = "Programmable 12 End Address (rw) register accessor: Programmable 12 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable12end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable12end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable12end_address`]
module"]
#[doc(alias = "Programmable 12 End Address")]
pub type Programmable12endAddress =
    crate::Reg<programmable12end_address::Programmable12endAddressSpec>;
#[doc = "Programmable 12 End Address"]
pub mod programmable12end_address;
#[doc = "Programmable 12 MPPA (rw) register accessor: Programmable 12 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable12mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable12mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable12mppa`]
module"]
#[doc(alias = "Programmable 12 MPPA")]
pub type Programmable12mppa = crate::Reg<programmable12mppa::Programmable12mppaSpec>;
#[doc = "Programmable 12 MPPA"]
pub mod programmable12mppa;
#[doc = "Programmable 13 Start Address (rw) register accessor: Programmable 13 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable13start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable13start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable13start_address`]
module"]
#[doc(alias = "Programmable 13 Start Address")]
pub type Programmable13startAddress =
    crate::Reg<programmable13start_address::Programmable13startAddressSpec>;
#[doc = "Programmable 13 Start Address"]
pub mod programmable13start_address;
#[doc = "Programmable 13 End Address (rw) register accessor: Programmable 13 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable13end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable13end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable13end_address`]
module"]
#[doc(alias = "Programmable 13 End Address")]
pub type Programmable13endAddress =
    crate::Reg<programmable13end_address::Programmable13endAddressSpec>;
#[doc = "Programmable 13 End Address"]
pub mod programmable13end_address;
#[doc = "Programmable 13 MPPA (rw) register accessor: Programmable 13 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable13mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable13mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable13mppa`]
module"]
#[doc(alias = "Programmable 13 MPPA")]
pub type Programmable13mppa = crate::Reg<programmable13mppa::Programmable13mppaSpec>;
#[doc = "Programmable 13 MPPA"]
pub mod programmable13mppa;
#[doc = "Programmable 14 Start Address (rw) register accessor: Programmable 14 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable14start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable14start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable14start_address`]
module"]
#[doc(alias = "Programmable 14 Start Address")]
pub type Programmable14startAddress =
    crate::Reg<programmable14start_address::Programmable14startAddressSpec>;
#[doc = "Programmable 14 Start Address"]
pub mod programmable14start_address;
#[doc = "Programmable 14 End Address (rw) register accessor: Programmable 14 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable14end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable14end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable14end_address`]
module"]
#[doc(alias = "Programmable 14 End Address")]
pub type Programmable14endAddress =
    crate::Reg<programmable14end_address::Programmable14endAddressSpec>;
#[doc = "Programmable 14 End Address"]
pub mod programmable14end_address;
#[doc = "Programmable 14 MPPA (rw) register accessor: Programmable 14 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable14mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable14mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable14mppa`]
module"]
#[doc(alias = "Programmable 14 MPPA")]
pub type Programmable14mppa = crate::Reg<programmable14mppa::Programmable14mppaSpec>;
#[doc = "Programmable 14 MPPA"]
pub mod programmable14mppa;
#[doc = "Programmable 15 Start Address (rw) register accessor: Programmable 15 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable15start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable15start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable15start_address`]
module"]
#[doc(alias = "Programmable 15 Start Address")]
pub type Programmable15startAddress =
    crate::Reg<programmable15start_address::Programmable15startAddressSpec>;
#[doc = "Programmable 15 Start Address"]
pub mod programmable15start_address;
#[doc = "Programmable 15 End Address (rw) register accessor: Programmable 15 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable15end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable15end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable15end_address`]
module"]
#[doc(alias = "Programmable 15 End Address")]
pub type Programmable15endAddress =
    crate::Reg<programmable15end_address::Programmable15endAddressSpec>;
#[doc = "Programmable 15 End Address"]
pub mod programmable15end_address;
#[doc = "Programmable 15 MPPA (rw) register accessor: Programmable 15 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable15mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable15mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable15mppa`]
module"]
#[doc(alias = "Programmable 15 MPPA")]
pub type Programmable15mppa = crate::Reg<programmable15mppa::Programmable15mppaSpec>;
#[doc = "Programmable 15 MPPA"]
pub mod programmable15mppa;
#[doc = "Programmable 16 Start Address (rw) register accessor: Programmable 16 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable16start_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable16start_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable16start_address`]
module"]
#[doc(alias = "Programmable 16 Start Address")]
pub type Programmable16startAddress =
    crate::Reg<programmable16start_address::Programmable16startAddressSpec>;
#[doc = "Programmable 16 Start Address"]
pub mod programmable16start_address;
#[doc = "Programmable 16 End Address (rw) register accessor: Programmable 16 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable16end_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable16end_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable16end_address`]
module"]
#[doc(alias = "Programmable 16 End Address")]
pub type Programmable16endAddress =
    crate::Reg<programmable16end_address::Programmable16endAddressSpec>;
#[doc = "Programmable 16 End Address"]
pub mod programmable16end_address;
#[doc = "Programmable 16 MPPA (rw) register accessor: Programmable 16 MPPA\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable16mppa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable16mppa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@programmable16mppa`]
module"]
#[doc(alias = "Programmable 16 MPPA")]
pub type Programmable16mppa = crate::Reg<programmable16mppa::Programmable16mppaSpec>;
#[doc = "Programmable 16 MPPA"]
pub mod programmable16mppa;
#[doc = "Fault Address (rw) register accessor: Fault Address\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_address`]
module"]
#[doc(alias = "Fault Address")]
pub type FaultAddress = crate::Reg<fault_address::FaultAddressSpec>;
#[doc = "Fault Address"]
pub mod fault_address;
#[doc = "Fault Status (rw) register accessor: Fault Status\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_status`]
module"]
#[doc(alias = "Fault Status")]
pub type FaultStatus = crate::Reg<fault_status::FaultStatusSpec>;
#[doc = "Fault Status"]
pub mod fault_status;
#[doc = "Fault Clear (rw) register accessor: Fault Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_clear`]
module"]
#[doc(alias = "Fault Clear")]
pub type FaultClear = crate::Reg<fault_clear::FaultClearSpec>;
#[doc = "Fault Clear"]
pub mod fault_clear;
