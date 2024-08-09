#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    register0: Register0,
    register1: Register1,
    register2: Register2,
    register3: Register3,
    register4: Register4,
    register5: Register5,
    register6: Register6,
    register7: Register7,
    register8: Register8,
    register9: Register9,
    register10: Register10,
    register11: Register11,
    register12: Register12,
    register13: Register13,
    register14: Register14,
    register15: Register15,
}
impl RegisterBlock {
    #[doc = "0x00 - First Register"]
    #[inline(always)]
    pub const fn register0(&self) -> &Register0 {
        &self.register0
    }
    #[doc = "0x04 - REGISTER1"]
    #[inline(always)]
    pub const fn register1(&self) -> &Register1 {
        &self.register1
    }
    #[doc = "0x08 - REGISTER2"]
    #[inline(always)]
    pub const fn register2(&self) -> &Register2 {
        &self.register2
    }
    #[doc = "0x0c - REGISTER3"]
    #[inline(always)]
    pub const fn register3(&self) -> &Register3 {
        &self.register3
    }
    #[doc = "0x10 - REGISTER4"]
    #[inline(always)]
    pub const fn register4(&self) -> &Register4 {
        &self.register4
    }
    #[doc = "0x14 - REGISTER5"]
    #[inline(always)]
    pub const fn register5(&self) -> &Register5 {
        &self.register5
    }
    #[doc = "0x18 - REGISTER6"]
    #[inline(always)]
    pub const fn register6(&self) -> &Register6 {
        &self.register6
    }
    #[doc = "0x1c - REGISTER7"]
    #[inline(always)]
    pub const fn register7(&self) -> &Register7 {
        &self.register7
    }
    #[doc = "0x20 - REGISTER8"]
    #[inline(always)]
    pub const fn register8(&self) -> &Register8 {
        &self.register8
    }
    #[doc = "0x24 - REGISTER9"]
    #[inline(always)]
    pub const fn register9(&self) -> &Register9 {
        &self.register9
    }
    #[doc = "0x28 - REGISTER10"]
    #[inline(always)]
    pub const fn register10(&self) -> &Register10 {
        &self.register10
    }
    #[doc = "0x2c - REGISTER11"]
    #[inline(always)]
    pub const fn register11(&self) -> &Register11 {
        &self.register11
    }
    #[doc = "0x30 - REGISTER12"]
    #[inline(always)]
    pub const fn register12(&self) -> &Register12 {
        &self.register12
    }
    #[doc = "0x34 - REGISTER13"]
    #[inline(always)]
    pub const fn register13(&self) -> &Register13 {
        &self.register13
    }
    #[doc = "0x38 - REGISTER14"]
    #[inline(always)]
    pub const fn register14(&self) -> &Register14 {
        &self.register14
    }
    #[doc = "0x3c - REGISTER15"]
    #[inline(always)]
    pub const fn register15(&self) -> &Register15 {
        &self.register15
    }
}
#[doc = "REGISTER0 (rw) register accessor: First Register\n\nYou can [`read`](crate::Reg::read) this register and get [`register0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register0`]
module"]
#[doc(alias = "REGISTER0")]
pub type Register0 = crate::Reg<register0::Register0Spec>;
#[doc = "First Register"]
pub mod register0;
#[doc = "REGISTER1 (rw) register accessor: REGISTER1\n\nYou can [`read`](crate::Reg::read) this register and get [`register1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register1`]
module"]
#[doc(alias = "REGISTER1")]
pub type Register1 = crate::Reg<register1::Register1Spec>;
#[doc = "REGISTER1"]
pub mod register1;
#[doc = "REGISTER2 (rw) register accessor: REGISTER2\n\nYou can [`read`](crate::Reg::read) this register and get [`register2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register2`]
module"]
#[doc(alias = "REGISTER2")]
pub type Register2 = crate::Reg<register2::Register2Spec>;
#[doc = "REGISTER2"]
pub mod register2;
#[doc = "REGISTER3 (rw) register accessor: REGISTER3\n\nYou can [`read`](crate::Reg::read) this register and get [`register3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register3`]
module"]
#[doc(alias = "REGISTER3")]
pub type Register3 = crate::Reg<register3::Register3Spec>;
#[doc = "REGISTER3"]
pub mod register3;
#[doc = "REGISTER4 (rw) register accessor: REGISTER4\n\nYou can [`read`](crate::Reg::read) this register and get [`register4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register4`]
module"]
#[doc(alias = "REGISTER4")]
pub type Register4 = crate::Reg<register4::Register4Spec>;
#[doc = "REGISTER4"]
pub mod register4;
#[doc = "REGISTER5 (rw) register accessor: REGISTER5\n\nYou can [`read`](crate::Reg::read) this register and get [`register5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register5`]
module"]
#[doc(alias = "REGISTER5")]
pub type Register5 = crate::Reg<register5::Register5Spec>;
#[doc = "REGISTER5"]
pub mod register5;
#[doc = "REGISTER6 (rw) register accessor: REGISTER6\n\nYou can [`read`](crate::Reg::read) this register and get [`register6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register6`]
module"]
#[doc(alias = "REGISTER6")]
pub type Register6 = crate::Reg<register6::Register6Spec>;
#[doc = "REGISTER6"]
pub mod register6;
#[doc = "REGISTER7 (rw) register accessor: REGISTER7\n\nYou can [`read`](crate::Reg::read) this register and get [`register7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register7`]
module"]
#[doc(alias = "REGISTER7")]
pub type Register7 = crate::Reg<register7::Register7Spec>;
#[doc = "REGISTER7"]
pub mod register7;
#[doc = "REGISTER8 (rw) register accessor: REGISTER8\n\nYou can [`read`](crate::Reg::read) this register and get [`register8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register8`]
module"]
#[doc(alias = "REGISTER8")]
pub type Register8 = crate::Reg<register8::Register8Spec>;
#[doc = "REGISTER8"]
pub mod register8;
#[doc = "REGISTER9 (rw) register accessor: REGISTER9\n\nYou can [`read`](crate::Reg::read) this register and get [`register9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register9`]
module"]
#[doc(alias = "REGISTER9")]
pub type Register9 = crate::Reg<register9::Register9Spec>;
#[doc = "REGISTER9"]
pub mod register9;
#[doc = "REGISTER10 (rw) register accessor: REGISTER10\n\nYou can [`read`](crate::Reg::read) this register and get [`register10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register10`]
module"]
#[doc(alias = "REGISTER10")]
pub type Register10 = crate::Reg<register10::Register10Spec>;
#[doc = "REGISTER10"]
pub mod register10;
#[doc = "REGISTER11 (rw) register accessor: REGISTER11\n\nYou can [`read`](crate::Reg::read) this register and get [`register11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register11`]
module"]
#[doc(alias = "REGISTER11")]
pub type Register11 = crate::Reg<register11::Register11Spec>;
#[doc = "REGISTER11"]
pub mod register11;
#[doc = "REGISTER12 (rw) register accessor: REGISTER12\n\nYou can [`read`](crate::Reg::read) this register and get [`register12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register12`]
module"]
#[doc(alias = "REGISTER12")]
pub type Register12 = crate::Reg<register12::Register12Spec>;
#[doc = "REGISTER12"]
pub mod register12;
#[doc = "REGISTER13 (rw) register accessor: REGISTER13\n\nYou can [`read`](crate::Reg::read) this register and get [`register13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register13`]
module"]
#[doc(alias = "REGISTER13")]
pub type Register13 = crate::Reg<register13::Register13Spec>;
#[doc = "REGISTER13"]
pub mod register13;
#[doc = "REGISTER14 (rw) register accessor: REGISTER14\n\nYou can [`read`](crate::Reg::read) this register and get [`register14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register14`]
module"]
#[doc(alias = "REGISTER14")]
pub type Register14 = crate::Reg<register14::Register14Spec>;
#[doc = "REGISTER14"]
pub mod register14;
#[doc = "REGISTER15 (rw) register accessor: REGISTER15\n\nYou can [`read`](crate::Reg::read) this register and get [`register15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register15`]
module"]
#[doc(alias = "REGISTER15")]
pub type Register15 = crate::Reg<register15::Register15Spec>;
#[doc = "REGISTER15"]
pub mod register15;
