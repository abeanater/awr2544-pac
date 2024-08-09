#[doc = "Register `HWA_SAFETY_ERR_STATUS` reader"]
pub type R = crate::R<HwaSafetyErrStatusSpec>;
#[doc = "Register `HWA_SAFETY_ERR_STATUS` writer"]
pub type W = crate::W<HwaSafetyErrStatusSpec>;
#[doc = "Field `dmem0` reader - 0:0\\]
Indicates the parity error in dmem0 (Masked status)"]
pub type Dmem0R = crate::BitReader;
#[doc = "Field `dmem0` writer - 0:0\\]
Indicates the parity error in dmem0 (Masked status)"]
pub type Dmem0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmem1` reader - 1:1\\]
Indicates the parity error in dmem1 (Masked status)"]
pub type Dmem1R = crate::BitReader;
#[doc = "Field `dmem1` writer - 1:1\\]
Indicates the parity error in dmem1 (Masked status)"]
pub type Dmem1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmem2` reader - 2:2\\]
Indicates the parity error in dmem2 (Masked status)"]
pub type Dmem2R = crate::BitReader;
#[doc = "Field `dmem2` writer - 2:2\\]
Indicates the parity error in dmem2 (Masked status)"]
pub type Dmem2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmem3` reader - 3:3\\]
Indicates the parity error in dmem3 (Masked status)"]
pub type Dmem3R = crate::BitReader;
#[doc = "Field `dmem3` writer - 3:3\\]
Indicates the parity error in dmem3 (Masked status)"]
pub type Dmem3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `window_ram` reader - 8:8\\]
Indicates the parity error in window RAM (Masked status)"]
pub type WindowRamR = crate::BitReader;
#[doc = "Field `window_ram` writer - 8:8\\]
Indicates the parity error in window RAM (Masked status)"]
pub type WindowRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fsm_lockstep` reader - 9:9\\]
Indicates the FSM lockstep error (Masked status)"]
pub type FsmLockstepR = crate::BitReader;
#[doc = "Field `fsm_lockstep` writer - 9:9\\]
Indicates the FSM lockstep error (Masked status)"]
pub type FsmLockstepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the parity error in dmem0 (Masked status)"]
    #[inline(always)]
    pub fn dmem0(&self) -> Dmem0R {
        Dmem0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the parity error in dmem1 (Masked status)"]
    #[inline(always)]
    pub fn dmem1(&self) -> Dmem1R {
        Dmem1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates the parity error in dmem2 (Masked status)"]
    #[inline(always)]
    pub fn dmem2(&self) -> Dmem2R {
        Dmem2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the parity error in dmem3 (Masked status)"]
    #[inline(always)]
    pub fn dmem3(&self) -> Dmem3R {
        Dmem3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates the parity error in window RAM (Masked status)"]
    #[inline(always)]
    pub fn window_ram(&self) -> WindowRamR {
        WindowRamR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates the FSM lockstep error (Masked status)"]
    #[inline(always)]
    pub fn fsm_lockstep(&self) -> FsmLockstepR {
        FsmLockstepR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the parity error in dmem0 (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn dmem0(&mut self) -> Dmem0W<HwaSafetyErrStatusSpec> {
        Dmem0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the parity error in dmem1 (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn dmem1(&mut self) -> Dmem1W<HwaSafetyErrStatusSpec> {
        Dmem1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates the parity error in dmem2 (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn dmem2(&mut self) -> Dmem2W<HwaSafetyErrStatusSpec> {
        Dmem2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the parity error in dmem3 (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn dmem3(&mut self) -> Dmem3W<HwaSafetyErrStatusSpec> {
        Dmem3W::new(self, 3)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates the parity error in window RAM (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn window_ram(&mut self) -> WindowRamW<HwaSafetyErrStatusSpec> {
        WindowRamW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates the FSM lockstep error (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn fsm_lockstep(&mut self) -> FsmLockstepW<HwaSafetyErrStatusSpec> {
        FsmLockstepW::new(self, 9)
    }
}
#[doc = "HWA_SAFETY_ERR_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_err_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_err_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwaSafetyErrStatusSpec;
impl crate::RegisterSpec for HwaSafetyErrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwa_safety_err_status::R`](R) reader structure"]
impl crate::Readable for HwaSafetyErrStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hwa_safety_err_status::W`](W) writer structure"]
impl crate::Writable for HwaSafetyErrStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWA_SAFETY_ERR_STATUS to value 0"]
impl crate::Resettable for HwaSafetyErrStatusSpec {
    const RESET_VALUE: u32 = 0;
}
