#[doc = "Register `HWA_SAFETY_EN` reader"]
pub type R = crate::R<HwaSafetyEnSpec>;
#[doc = "Register `HWA_SAFETY_EN` writer"]
pub type W = crate::W<HwaSafetyEnSpec>;
#[doc = "Field `cfg_fsm_lockstep_en` reader - 0:0\\]
Writing 1'b1 would enable the lockstep logic for FSM"]
pub type CfgFsmLockstepEnR = crate::BitReader;
#[doc = "Field `cfg_fsm_lockstep_en` writer - 0:0\\]
Writing 1'b1 would enable the lockstep logic for FSM"]
pub type CfgFsmLockstepEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_fsm_lockstep_inv_en` reader - 1:1\\]
Writing 1'b1 will invert the redundant FSM outputs. This can be used for selftest of FSM lockstep error interrupt. This bit is self clearing bit"]
pub type CfgFsmLockstepInvEnR = crate::BitReader;
#[doc = "Field `cfg_fsm_lockstep_inv_en` writer - 1:1\\]
Writing 1'b1 will invert the redundant FSM outputs. This can be used for selftest of FSM lockstep error interrupt. This bit is self clearing bit"]
pub type CfgFsmLockstepInvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_window_ram_parity_en` reader - 2:2\\]
Writing 1'b1 enables parity for windowing RAM"]
pub type CfgWindowRamParityEnR = crate::BitReader;
#[doc = "Field `cfg_window_ram_parity_en` writer - 2:2\\]
Writing 1'b1 enables parity for windowing RAM"]
pub type CfgWindowRamParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_dmem_parity_en` reader - 3:3\\]
Writing 1'b1 would enable the parity chekcer for the 8 DMEM memories"]
pub type CfgDmemParityEnR = crate::BitReader;
#[doc = "Field `cfg_dmem_parity_en` writer - 3:3\\]
Writing 1'b1 would enable the parity chekcer for the 8 DMEM memories"]
pub type CfgDmemParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 1'b1 would enable the lockstep logic for FSM"]
    #[inline(always)]
    pub fn cfg_fsm_lockstep_en(&self) -> CfgFsmLockstepEnR {
        CfgFsmLockstepEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 1'b1 will invert the redundant FSM outputs. This can be used for selftest of FSM lockstep error interrupt. This bit is self clearing bit"]
    #[inline(always)]
    pub fn cfg_fsm_lockstep_inv_en(&self) -> CfgFsmLockstepInvEnR {
        CfgFsmLockstepInvEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 1'b1 enables parity for windowing RAM"]
    #[inline(always)]
    pub fn cfg_window_ram_parity_en(&self) -> CfgWindowRamParityEnR {
        CfgWindowRamParityEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 1'b1 would enable the parity chekcer for the 8 DMEM memories"]
    #[inline(always)]
    pub fn cfg_dmem_parity_en(&self) -> CfgDmemParityEnR {
        CfgDmemParityEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing 1'b1 would enable the lockstep logic for FSM"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_fsm_lockstep_en(&mut self) -> CfgFsmLockstepEnW<HwaSafetyEnSpec> {
        CfgFsmLockstepEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 1'b1 will invert the redundant FSM outputs. This can be used for selftest of FSM lockstep error interrupt. This bit is self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_fsm_lockstep_inv_en(&mut self) -> CfgFsmLockstepInvEnW<HwaSafetyEnSpec> {
        CfgFsmLockstepInvEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 1'b1 enables parity for windowing RAM"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_window_ram_parity_en(&mut self) -> CfgWindowRamParityEnW<HwaSafetyEnSpec> {
        CfgWindowRamParityEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 1'b1 would enable the parity chekcer for the 8 DMEM memories"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_dmem_parity_en(&mut self) -> CfgDmemParityEnW<HwaSafetyEnSpec> {
        CfgDmemParityEnW::new(self, 3)
    }
}
#[doc = "HWA_SAFETY_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwaSafetyEnSpec;
impl crate::RegisterSpec for HwaSafetyEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwa_safety_en::R`](R) reader structure"]
impl crate::Readable for HwaSafetyEnSpec {}
#[doc = "`write(|w| ..)` method takes [`hwa_safety_en::W`](W) writer structure"]
impl crate::Writable for HwaSafetyEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWA_SAFETY_EN to value 0"]
impl crate::Resettable for HwaSafetyEnSpec {
    const RESET_VALUE: u32 = 0;
}
