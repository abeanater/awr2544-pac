#[doc = "Register `CSI2_SYSSTATUS` reader"]
pub type R = crate::R<Csi2SysstatusSpec>;
#[doc = "Register `CSI2_SYSSTATUS` writer"]
pub type W = crate::W<Csi2SysstatusSpec>;
#[doc = "0:0\\]
Internal reset monitoring\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetDone {
    #[doc = "0: Internal module reset is on going."]
    Resetongoing = 0,
    #[doc = "1: Reset completed."]
    Resetcompleted = 1,
}
impl From<ResetDone> for bool {
    #[inline(always)]
    fn from(variant: ResetDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET_DONE` reader - 0:0\\]
Internal reset monitoring"]
pub type ResetDoneR = crate::BitReader<ResetDone>;
impl ResetDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResetDone {
        match self.bits {
            false => ResetDone::Resetongoing,
            true => ResetDone::Resetcompleted,
        }
    }
    #[doc = "Internal module reset is on going."]
    #[inline(always)]
    pub fn is_resetongoing(&self) -> bool {
        *self == ResetDone::Resetongoing
    }
    #[doc = "Reset completed."]
    #[inline(always)]
    pub fn is_resetcompleted(&self) -> bool {
        *self == ResetDone::Resetcompleted
    }
}
#[doc = "Field `RESET_DONE` writer - 0:0\\]
Internal reset monitoring"]
pub type ResetDoneW<'a, REG> = crate::BitWriter<'a, REG, ResetDone>;
impl<'a, REG> ResetDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal module reset is on going."]
    #[inline(always)]
    pub fn resetongoing(self) -> &'a mut crate::W<REG> {
        self.variant(ResetDone::Resetongoing)
    }
    #[doc = "Reset completed."]
    #[inline(always)]
    pub fn resetcompleted(self) -> &'a mut crate::W<REG> {
        self.variant(ResetDone::Resetcompleted)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal reset monitoring"]
    #[inline(always)]
    pub fn reset_done(&self) -> ResetDoneR {
        ResetDoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal reset monitoring"]
    #[inline(always)]
    #[must_use]
    pub fn reset_done(&mut self) -> ResetDoneW<Csi2SysstatusSpec> {
        ResetDoneW::new(self, 0)
    }
}
#[doc = "SYSTEM STATUS REGISTER This register provides status information about the module, excluding the interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_sysstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_sysstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2SysstatusSpec;
impl crate::RegisterSpec for Csi2SysstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_sysstatus::R`](R) reader structure"]
impl crate::Readable for Csi2SysstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_sysstatus::W`](W) writer structure"]
impl crate::Writable for Csi2SysstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_SYSSTATUS to value 0"]
impl crate::Resettable for Csi2SysstatusSpec {
    const RESET_VALUE: u32 = 0;
}
