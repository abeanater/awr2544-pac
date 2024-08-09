#[doc = "Register `REGISTER15` reader"]
pub type R = crate::R<Register15Spec>;
#[doc = "Register `REGISTER15` writer"]
pub type W = crate::W<Register15Spec>;
#[doc = "Field `REGNOTRANSITIONCTRL` reader - 2:0\\]
Default: 011"]
pub type RegnotransitionctrlR = crate::FieldReader;
#[doc = "Field `REGNOTRANSITIONCTRL` writer - 2:0\\]
Default: 011"]
pub type RegnotransitionctrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OVRRDNOTRANSITIONCTRL` reader - 3:3\\]
1: Override with register bits 0: Default"]
pub type OvrrdnotransitionctrlR = crate::BitReader;
#[doc = "Field `OVRRDNOTRANSITIONCTRL` writer - 3:3\\]
1: Override with register bits 0: Default"]
pub type OvrrdnotransitionctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTRANSITIONDISABLE` reader - 8:4\\]
1: NoTransition disabled 0: NoTransition controlled by FSM Default: 00000"]
pub type NotransitiondisableR = crate::FieldReader;
#[doc = "Field `NOTRANSITIONDISABLE` writer - 8:4\\]
1: NoTransition disabled 0: NoTransition controlled by FSM Default: 00000"]
pub type NotransitiondisableW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EMPTY` reader - 20:9\\]
Reserved"]
pub type EmptyR = crate::FieldReader<u16>;
#[doc = "Field `EMPTY` writer - 20:9\\]
Reserved"]
pub type EmptyW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `REG_THSTXEN` reader - 24:21\\]
REG_THSTXEN is timing parameter for HSTXEN deassertion staggering. staggered time = REG_THSTXEN * (4 * Reg_Tlpxby2) Default : 1010 1000 ns 0000 : Reserved"]
pub type RegThstxenR = crate::FieldReader;
#[doc = "Field `REG_THSTXEN` writer - 24:21\\]
REG_THSTXEN is timing parameter for HSTXEN deassertion staggering. staggered time = REG_THSTXEN * (4 * Reg_Tlpxby2) Default : 1010 1000 ns 0000 : Reserved"]
pub type RegThstxenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REGLDOHIZEN` reader - 30:25\\]
1: LDO in Hiz Mode 0: LDO not in Hiz mode"]
pub type RegldohizenR = crate::FieldReader;
#[doc = "Field `REGLDOHIZEN` writer - 30:25\\]
1: LDO in Hiz Mode 0: LDO not in Hiz mode"]
pub type RegldohizenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `OVRRDLDOHIZEN` reader - 31:31\\]
1: Override with register bit 0: Default"]
pub type OvrrdldohizenR = crate::BitReader;
#[doc = "Field `OVRRDLDOHIZEN` writer - 31:31\\]
1: Override with register bit 0: Default"]
pub type OvrrdldohizenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Default: 011"]
    #[inline(always)]
    pub fn regnotransitionctrl(&self) -> RegnotransitionctrlR {
        RegnotransitionctrlR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
1: Override with register bits 0: Default"]
    #[inline(always)]
    pub fn ovrrdnotransitionctrl(&self) -> OvrrdnotransitionctrlR {
        OvrrdnotransitionctrlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - 8:4\\]
1: NoTransition disabled 0: NoTransition controlled by FSM Default: 00000"]
    #[inline(always)]
    pub fn notransitiondisable(&self) -> NotransitiondisableR {
        NotransitiondisableR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:20 - 20:9\\]
Reserved"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 9) & 0x0fff) as u16)
    }
    #[doc = "Bits 21:24 - 24:21\\]
REG_THSTXEN is timing parameter for HSTXEN deassertion staggering. staggered time = REG_THSTXEN * (4 * Reg_Tlpxby2) Default : 1010 1000 ns 0000 : Reserved"]
    #[inline(always)]
    pub fn reg_thstxen(&self) -> RegThstxenR {
        RegThstxenR::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 25:30 - 30:25\\]
1: LDO in Hiz Mode 0: LDO not in Hiz mode"]
    #[inline(always)]
    pub fn regldohizen(&self) -> RegldohizenR {
        RegldohizenR::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
1: Override with register bit 0: Default"]
    #[inline(always)]
    pub fn ovrrdldohizen(&self) -> OvrrdldohizenR {
        OvrrdldohizenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Default: 011"]
    #[inline(always)]
    #[must_use]
    pub fn regnotransitionctrl(&mut self) -> RegnotransitionctrlW<Register15Spec> {
        RegnotransitionctrlW::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
1: Override with register bits 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdnotransitionctrl(&mut self) -> OvrrdnotransitionctrlW<Register15Spec> {
        OvrrdnotransitionctrlW::new(self, 3)
    }
    #[doc = "Bits 4:8 - 8:4\\]
1: NoTransition disabled 0: NoTransition controlled by FSM Default: 00000"]
    #[inline(always)]
    #[must_use]
    pub fn notransitiondisable(&mut self) -> NotransitiondisableW<Register15Spec> {
        NotransitiondisableW::new(self, 4)
    }
    #[doc = "Bits 9:20 - 20:9\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<Register15Spec> {
        EmptyW::new(self, 9)
    }
    #[doc = "Bits 21:24 - 24:21\\]
REG_THSTXEN is timing parameter for HSTXEN deassertion staggering. staggered time = REG_THSTXEN * (4 * Reg_Tlpxby2) Default : 1010 1000 ns 0000 : Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_thstxen(&mut self) -> RegThstxenW<Register15Spec> {
        RegThstxenW::new(self, 21)
    }
    #[doc = "Bits 25:30 - 30:25\\]
1: LDO in Hiz Mode 0: LDO not in Hiz mode"]
    #[inline(always)]
    #[must_use]
    pub fn regldohizen(&mut self) -> RegldohizenW<Register15Spec> {
        RegldohizenW::new(self, 25)
    }
    #[doc = "Bit 31 - 31:31\\]
1: Override with register bit 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdldohizen(&mut self) -> OvrrdldohizenW<Register15Spec> {
        OvrrdldohizenW::new(self, 31)
    }
}
#[doc = "REGISTER15\n\nYou can [`read`](crate::Reg::read) this register and get [`register15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register15Spec;
impl crate::RegisterSpec for Register15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register15::R`](R) reader structure"]
impl crate::Readable for Register15Spec {}
#[doc = "`write(|w| ..)` method takes [`register15::W`](W) writer structure"]
impl crate::Writable for Register15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER15 to value 0"]
impl crate::Resettable for Register15Spec {
    const RESET_VALUE: u32 = 0;
}
