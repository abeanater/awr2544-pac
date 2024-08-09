#[doc = "Register `MDIO_CONTROL_REG` reader"]
pub type R = crate::R<MdioControlRegSpec>;
#[doc = "Register `MDIO_CONTROL_REG` writer"]
pub type W = crate::W<MdioControlRegSpec>;
#[doc = "Field `CLOCK_DIVIDER` reader - 15:0\\]
Clock divider"]
pub type ClockDividerR = crate::FieldReader<u16>;
#[doc = "Field `CLOCK_DIVIDER` writer - 15:0\\]
Clock divider"]
pub type ClockDividerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INTERRUPT_TEST_ENABLE` reader - 17:17\\]
Interrupt test enable"]
pub type InterruptTestEnableR = crate::BitReader;
#[doc = "Field `INTERRUPT_TEST_ENABLE` writer - 17:17\\]
Interrupt test enable"]
pub type InterruptTestEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT_DETECT_ENABLE` reader - 18:18\\]
Fault detect enable"]
pub type FaultDetectEnableR = crate::BitReader;
#[doc = "Field `FAULT_DETECT_ENABLE` writer - 18:18\\]
Fault detect enable"]
pub type FaultDetectEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT_INDICATOR` reader - 19:19\\]
Fault indicator"]
pub type FaultIndicatorR = crate::BitReader;
#[doc = "Field `FAULT_INDICATOR` writer - 19:19\\]
Fault indicator"]
pub type FaultIndicatorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREAMBLE_DISABLE` reader - 20:20\\]
Preamble disable"]
pub type PreambleDisableR = crate::BitReader;
#[doc = "Field `PREAMBLE_DISABLE` writer - 20:20\\]
Preamble disable"]
pub type PreambleDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGHEST_USER_CHANNEL` reader - 28:24\\]
Highest user channel"]
pub type HighestUserChannelR = crate::FieldReader;
#[doc = "Field `HIGHEST_USER_CHANNEL` writer - 28:24\\]
Highest user channel"]
pub type HighestUserChannelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ENABLE_CONTROL` reader - 30:30\\]
Enable control"]
pub type EnableControlR = crate::BitReader;
#[doc = "Field `ENABLE_CONTROL` writer - 30:30\\]
Enable control"]
pub type EnableControlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDIO_STATE_MACHINE` reader - 31:31\\]
MDIO state machine idle"]
pub type MdioStateMachineR = crate::BitReader;
#[doc = "Field `MDIO_STATE_MACHINE` writer - 31:31\\]
MDIO state machine idle"]
pub type MdioStateMachineW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Clock divider"]
    #[inline(always)]
    pub fn clock_divider(&self) -> ClockDividerR {
        ClockDividerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt test enable"]
    #[inline(always)]
    pub fn interrupt_test_enable(&self) -> InterruptTestEnableR {
        InterruptTestEnableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Fault detect enable"]
    #[inline(always)]
    pub fn fault_detect_enable(&self) -> FaultDetectEnableR {
        FaultDetectEnableR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Fault indicator"]
    #[inline(always)]
    pub fn fault_indicator(&self) -> FaultIndicatorR {
        FaultIndicatorR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Preamble disable"]
    #[inline(always)]
    pub fn preamble_disable(&self) -> PreambleDisableR {
        PreambleDisableR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Highest user channel"]
    #[inline(always)]
    pub fn highest_user_channel(&self) -> HighestUserChannelR {
        HighestUserChannelR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Enable control"]
    #[inline(always)]
    pub fn enable_control(&self) -> EnableControlR {
        EnableControlR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
MDIO state machine idle"]
    #[inline(always)]
    pub fn mdio_state_machine(&self) -> MdioStateMachineR {
        MdioStateMachineR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn clock_divider(&mut self) -> ClockDividerW<MdioControlRegSpec> {
        ClockDividerW::new(self, 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt test enable"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_test_enable(&mut self) -> InterruptTestEnableW<MdioControlRegSpec> {
        InterruptTestEnableW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Fault detect enable"]
    #[inline(always)]
    #[must_use]
    pub fn fault_detect_enable(&mut self) -> FaultDetectEnableW<MdioControlRegSpec> {
        FaultDetectEnableW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Fault indicator"]
    #[inline(always)]
    #[must_use]
    pub fn fault_indicator(&mut self) -> FaultIndicatorW<MdioControlRegSpec> {
        FaultIndicatorW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Preamble disable"]
    #[inline(always)]
    #[must_use]
    pub fn preamble_disable(&mut self) -> PreambleDisableW<MdioControlRegSpec> {
        PreambleDisableW::new(self, 20)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Highest user channel"]
    #[inline(always)]
    #[must_use]
    pub fn highest_user_channel(&mut self) -> HighestUserChannelW<MdioControlRegSpec> {
        HighestUserChannelW::new(self, 24)
    }
    #[doc = "Bit 30 - 30:30\\]
Enable control"]
    #[inline(always)]
    #[must_use]
    pub fn enable_control(&mut self) -> EnableControlW<MdioControlRegSpec> {
        EnableControlW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
MDIO state machine idle"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_state_machine(&mut self) -> MdioStateMachineW<MdioControlRegSpec> {
        MdioStateMachineW::new(self, 31)
    }
}
#[doc = "MDIO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioControlRegSpec;
impl crate::RegisterSpec for MdioControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_control_reg::R`](R) reader structure"]
impl crate::Readable for MdioControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_control_reg::W`](W) writer structure"]
impl crate::Writable for MdioControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_CONTROL_REG to value 0"]
impl crate::Resettable for MdioControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
