#[doc = "Register `R5_CONTROL` reader"]
pub type R = crate::R<R5ControlSpec>;
#[doc = "Register `R5_CONTROL` writer"]
pub type W = crate::W<R5ControlSpec>;
#[doc = "Field `lock_step` reader - 2:0\\]
writing 3'b000 ensures R5 to be in Dual-Core mode. Note: The change happens after the R5SS reset assertion if R5_CONTROL_lock_step_switch_wait is set. Or else the switiching to Dual-core happens on the fly."]
pub type LockStepR = crate::FieldReader;
#[doc = "Field `lock_step` writer - 2:0\\]
writing 3'b000 ensures R5 to be in Dual-Core mode. Note: The change happens after the R5SS reset assertion if R5_CONTROL_lock_step_switch_wait is set. Or else the switiching to Dual-core happens on the fly."]
pub type LockStepW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `lock_step_switch_wait` reader - 10:8\\]
writing 3'b111 ensures switch happens only after R5SS reset. Orelse it will be a immediate switch."]
pub type LockStepSwitchWaitR = crate::FieldReader;
#[doc = "Field `lock_step_switch_wait` writer - 10:8\\]
writing 3'b111 ensures switch happens only after R5SS reset. Orelse it will be a immediate switch."]
pub type LockStepSwitchWaitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reset_fsm_trigger` reader - 18:16\\]
Write pulse bit field: writing 3'b111 will trigger the reset FSM. Reset FSM ensures reset to R5SS and inturn ensures the latching of lock_step and also mem_swap bit"]
pub type ResetFsmTriggerR = crate::FieldReader;
#[doc = "Field `reset_fsm_trigger` writer - 18:16\\]
Write pulse bit field: writing 3'b111 will trigger the reset FSM. Reset FSM ensures reset to R5SS and inturn ensures the latching of lock_step and also mem_swap bit"]
pub type ResetFsmTriggerW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rom_wait_state` reader - 26:24\\]
writing '111' enables a single cycle wait state with respect to CR5A_clk for rom access. This needs to be set when R5 clock is at 400MHZ and Interconnect-clk is at 200MHZ. (because it is a timing issue in this scenario)"]
pub type RomWaitStateR = crate::FieldReader;
#[doc = "Field `rom_wait_state` writer - 26:24\\]
writing '111' enables a single cycle wait state with respect to CR5A_clk for rom access. This needs to be set when R5 clock is at 400MHZ and Interconnect-clk is at 200MHZ. (because it is a timing issue in this scenario)"]
pub type RomWaitStateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
writing 3'b000 ensures R5 to be in Dual-Core mode. Note: The change happens after the R5SS reset assertion if R5_CONTROL_lock_step_switch_wait is set. Or else the switiching to Dual-core happens on the fly."]
    #[inline(always)]
    pub fn lock_step(&self) -> LockStepR {
        LockStepR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
writing 3'b111 ensures switch happens only after R5SS reset. Orelse it will be a immediate switch."]
    #[inline(always)]
    pub fn lock_step_switch_wait(&self) -> LockStepSwitchWaitR {
        LockStepSwitchWaitR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Write pulse bit field: writing 3'b111 will trigger the reset FSM. Reset FSM ensures reset to R5SS and inturn ensures the latching of lock_step and also mem_swap bit"]
    #[inline(always)]
    pub fn reset_fsm_trigger(&self) -> ResetFsmTriggerR {
        ResetFsmTriggerR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
writing '111' enables a single cycle wait state with respect to CR5A_clk for rom access. This needs to be set when R5 clock is at 400MHZ and Interconnect-clk is at 200MHZ. (because it is a timing issue in this scenario)"]
    #[inline(always)]
    pub fn rom_wait_state(&self) -> RomWaitStateR {
        RomWaitStateR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
writing 3'b000 ensures R5 to be in Dual-Core mode. Note: The change happens after the R5SS reset assertion if R5_CONTROL_lock_step_switch_wait is set. Or else the switiching to Dual-core happens on the fly."]
    #[inline(always)]
    #[must_use]
    pub fn lock_step(&mut self) -> LockStepW<R5ControlSpec> {
        LockStepW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
writing 3'b111 ensures switch happens only after R5SS reset. Orelse it will be a immediate switch."]
    #[inline(always)]
    #[must_use]
    pub fn lock_step_switch_wait(&mut self) -> LockStepSwitchWaitW<R5ControlSpec> {
        LockStepSwitchWaitW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Write pulse bit field: writing 3'b111 will trigger the reset FSM. Reset FSM ensures reset to R5SS and inturn ensures the latching of lock_step and also mem_swap bit"]
    #[inline(always)]
    #[must_use]
    pub fn reset_fsm_trigger(&mut self) -> ResetFsmTriggerW<R5ControlSpec> {
        ResetFsmTriggerW::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
writing '111' enables a single cycle wait state with respect to CR5A_clk for rom access. This needs to be set when R5 clock is at 400MHZ and Interconnect-clk is at 200MHZ. (because it is a timing issue in this scenario)"]
    #[inline(always)]
    #[must_use]
    pub fn rom_wait_state(&mut self) -> RomWaitStateW<R5ControlSpec> {
        RomWaitStateW::new(self, 24)
    }
}
#[doc = "R5_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5ControlSpec;
impl crate::RegisterSpec for R5ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5_control::R`](R) reader structure"]
impl crate::Readable for R5ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`r5_control::W`](W) writer structure"]
impl crate::Writable for R5ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R5_CONTROL to value 0"]
impl crate::Resettable for R5ControlSpec {
    const RESET_VALUE: u32 = 0;
}
