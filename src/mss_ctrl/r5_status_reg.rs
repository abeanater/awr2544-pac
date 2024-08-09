#[doc = "Register `R5_STATUS_REG` reader"]
pub type R = crate::R<R5StatusRegSpec>;
#[doc = "Register `R5_STATUS_REG` writer"]
pub type W = crate::W<R5StatusRegSpec>;
#[doc = "Field `memswap` reader - 0:0\\]
reading 1: confirms ROM is Eclipsed from with RAM for R5."]
pub type MemswapR = crate::BitReader;
#[doc = "Field `memswap` writer - 0:0\\]
reading 1: confirms ROM is Eclipsed from with RAM for R5."]
pub type MemswapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lock_step` reader - 8:8\\]
Reading 1: confirms R5SS is in lockstep mode. Reading 0: confirms R5SS is in Dual-core mode."]
pub type LockStepR = crate::BitReader;
#[doc = "Field `lock_step` writer - 8:8\\]
Reading 1: confirms R5SS is in lockstep mode. Reading 0: confirms R5SS is in Dual-core mode."]
pub type LockStepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
reading 1: confirms ROM is Eclipsed from with RAM for R5."]
    #[inline(always)]
    pub fn memswap(&self) -> MemswapR {
        MemswapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Reading 1: confirms R5SS is in lockstep mode. Reading 0: confirms R5SS is in Dual-core mode."]
    #[inline(always)]
    pub fn lock_step(&self) -> LockStepR {
        LockStepR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
reading 1: confirms ROM is Eclipsed from with RAM for R5."]
    #[inline(always)]
    #[must_use]
    pub fn memswap(&mut self) -> MemswapW<R5StatusRegSpec> {
        MemswapW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Reading 1: confirms R5SS is in lockstep mode. Reading 0: confirms R5SS is in Dual-core mode."]
    #[inline(always)]
    #[must_use]
    pub fn lock_step(&mut self) -> LockStepW<R5StatusRegSpec> {
        LockStepW::new(self, 8)
    }
}
#[doc = "R5_STATUS_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_status_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_status_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5StatusRegSpec;
impl crate::RegisterSpec for R5StatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5_status_reg::R`](R) reader structure"]
impl crate::Readable for R5StatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`r5_status_reg::W`](W) writer structure"]
impl crate::Writable for R5StatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R5_STATUS_REG to value 0"]
impl crate::Resettable for R5StatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
