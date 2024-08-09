#[doc = "Register `CSI2_TIMING1` reader"]
pub type R = crate::R<Csi2Timing1Spec>;
#[doc = "Register `CSI2_TIMING1` writer"]
pub type W = crate::W<Csi2Timing1Spec>;
#[doc = "Field `STOP_STATE_COUNTER_IO` reader - 12:0\\]
Stop state counter. It indicates the number of CSI2_CLK function clock to assert ForceTXStopMode signal. The value is from 0 to 8191."]
pub type StopStateCounterIoR = crate::FieldReader<u16>;
#[doc = "Field `STOP_STATE_COUNTER_IO` writer - 12:0\\]
Stop state counter. It indicates the number of CSI2_CLK function clock to assert ForceTXStopMode signal. The value is from 0 to 8191."]
pub type StopStateCounterIoW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "13:13\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in STOP_STATE_COUNTER_IO bit-field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopStateX4Io {
    #[doc = "0: The number of CSI2_CLK functional clock cycles defined in STOP_STATE _COUNTER is multiplied by 1x"]
    Disable = 0,
    #[doc = "1: The number of CSI2_CLK functional clock cycles defined in STOP_STATE _COUNTER_IO is multiplied by 4x"]
    Enable = 1,
}
impl From<StopStateX4Io> for bool {
    #[inline(always)]
    fn from(variant: StopStateX4Io) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_STATE_X4_IO` reader - 13:13\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in STOP_STATE_COUNTER_IO bit-field"]
pub type StopStateX4IoR = crate::BitReader<StopStateX4Io>;
impl StopStateX4IoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StopStateX4Io {
        match self.bits {
            false => StopStateX4Io::Disable,
            true => StopStateX4Io::Enable,
        }
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in STOP_STATE _COUNTER is multiplied by 1x"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == StopStateX4Io::Disable
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in STOP_STATE _COUNTER_IO is multiplied by 4x"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == StopStateX4Io::Enable
    }
}
#[doc = "Field `STOP_STATE_X4_IO` writer - 13:13\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in STOP_STATE_COUNTER_IO bit-field"]
pub type StopStateX4IoW<'a, REG> = crate::BitWriter<'a, REG, StopStateX4Io>;
impl<'a, REG> StopStateX4IoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of CSI2_CLK functional clock cycles defined in STOP_STATE _COUNTER is multiplied by 1x"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(StopStateX4Io::Disable)
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in STOP_STATE _COUNTER_IO is multiplied by 4x"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(StopStateX4Io::Enable)
    }
}
#[doc = "14:14\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in STOP_STATE_COUNTER_IO bit-field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopStateX16Io {
    #[doc = "0: The number of CSI2_CLK functional clock cycles defined in STOP_STATE _COUNTER_IO is multiplied by 1x"]
    Disable = 0,
    #[doc = "1: The number of CSI2_CLK functional clock cycles defined in STOP_STATE _COUNTER_IO is multiplied by 16x"]
    Enable = 1,
}
impl From<StopStateX16Io> for bool {
    #[inline(always)]
    fn from(variant: StopStateX16Io) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_STATE_X16_IO` reader - 14:14\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in STOP_STATE_COUNTER_IO bit-field"]
pub type StopStateX16IoR = crate::BitReader<StopStateX16Io>;
impl StopStateX16IoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StopStateX16Io {
        match self.bits {
            false => StopStateX16Io::Disable,
            true => StopStateX16Io::Enable,
        }
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in STOP_STATE _COUNTER_IO is multiplied by 1x"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == StopStateX16Io::Disable
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in STOP_STATE _COUNTER_IO is multiplied by 16x"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == StopStateX16Io::Enable
    }
}
#[doc = "Field `STOP_STATE_X16_IO` writer - 14:14\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in STOP_STATE_COUNTER_IO bit-field"]
pub type StopStateX16IoW<'a, REG> = crate::BitWriter<'a, REG, StopStateX16Io>;
impl<'a, REG> StopStateX16IoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of CSI2_CLK functional clock cycles defined in STOP_STATE _COUNTER_IO is multiplied by 1x"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(StopStateX16Io::Disable)
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in STOP_STATE _COUNTER_IO is multiplied by 16x"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(StopStateX16Io::Enable)
    }
}
#[doc = "15:15\\]
Control of ForceTxStopMode signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceTxStopModeIo {
    #[doc = "0: De-assertion of ForceTxStopMode. The HW reset the bit at the end of the ForceTXStopMode assertion. The SW can reset the bit in order to stop the assertion of the ForceTXStopMode signal prior to the completion of the period."]
    Deassertion = 0,
    #[doc = "1: Assertion of ForceTxStopMode"]
    Assertion = 1,
}
impl From<ForceTxStopModeIo> for bool {
    #[inline(always)]
    fn from(variant: ForceTxStopModeIo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE_TX_STOP_MODE_IO` reader - 15:15\\]
Control of ForceTxStopMode signal"]
pub type ForceTxStopModeIoR = crate::BitReader<ForceTxStopModeIo>;
impl ForceTxStopModeIoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForceTxStopModeIo {
        match self.bits {
            false => ForceTxStopModeIo::Deassertion,
            true => ForceTxStopModeIo::Assertion,
        }
    }
    #[doc = "De-assertion of ForceTxStopMode. The HW reset the bit at the end of the ForceTXStopMode assertion. The SW can reset the bit in order to stop the assertion of the ForceTXStopMode signal prior to the completion of the period."]
    #[inline(always)]
    pub fn is_deassertion(&self) -> bool {
        *self == ForceTxStopModeIo::Deassertion
    }
    #[doc = "Assertion of ForceTxStopMode"]
    #[inline(always)]
    pub fn is_assertion(&self) -> bool {
        *self == ForceTxStopModeIo::Assertion
    }
}
#[doc = "Field `FORCE_TX_STOP_MODE_IO` writer - 15:15\\]
Control of ForceTxStopMode signal"]
pub type ForceTxStopModeIoW<'a, REG> = crate::BitWriter<'a, REG, ForceTxStopModeIo>;
impl<'a, REG> ForceTxStopModeIoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assertion of ForceTxStopMode. The HW reset the bit at the end of the ForceTXStopMode assertion. The SW can reset the bit in order to stop the assertion of the ForceTXStopMode signal prior to the completion of the period."]
    #[inline(always)]
    pub fn deassertion(self) -> &'a mut crate::W<REG> {
        self.variant(ForceTxStopModeIo::Deassertion)
    }
    #[doc = "Assertion of ForceTxStopMode"]
    #[inline(always)]
    pub fn assertion(self) -> &'a mut crate::W<REG> {
        self.variant(ForceTxStopModeIo::Assertion)
    }
}
#[doc = "Field `TA_TO_COUNTER` reader - 28:16\\]
Turn around counter. It indicates the number of CSI2_CLK function clock to wait for the change of the Direction PPI signal according to the TurnRequest signal The value is from 0 to 8191."]
pub type TaToCounterR = crate::FieldReader<u16>;
#[doc = "Field `TA_TO_COUNTER` writer - 28:16\\]
Turn around counter. It indicates the number of CSI2_CLK function clock to wait for the change of the Direction PPI signal according to the TurnRequest signal The value is from 0 to 8191."]
pub type TaToCounterW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "29:29\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER bit-field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaToX8 {
    #[doc = "0: The number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER is multiplied by 1x"]
    Disable = 0,
    #[doc = "1: The number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER is multiplied by 8x"]
    Enable = 1,
}
impl From<TaToX8> for bool {
    #[inline(always)]
    fn from(variant: TaToX8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TA_TO_X8` reader - 29:29\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER bit-field"]
pub type TaToX8R = crate::BitReader<TaToX8>;
impl TaToX8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TaToX8 {
        match self.bits {
            false => TaToX8::Disable,
            true => TaToX8::Enable,
        }
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER is multiplied by 1x"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TaToX8::Disable
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER is multiplied by 8x"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TaToX8::Enable
    }
}
#[doc = "Field `TA_TO_X8` writer - 29:29\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER bit-field"]
pub type TaToX8W<'a, REG> = crate::BitWriter<'a, REG, TaToX8>;
impl<'a, REG> TaToX8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER is multiplied by 1x"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TaToX8::Disable)
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER is multiplied by 8x"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TaToX8::Enable)
    }
}
#[doc = "30:30\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER bit-field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaToX16 {
    #[doc = "0: The number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER is multiplied by 1x"]
    Disable = 0,
    #[doc = "1: The number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER is multiplied by 16x"]
    Enable = 1,
}
impl From<TaToX16> for bool {
    #[inline(always)]
    fn from(variant: TaToX16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TA_TO_X16` reader - 30:30\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER bit-field"]
pub type TaToX16R = crate::BitReader<TaToX16>;
impl TaToX16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TaToX16 {
        match self.bits {
            false => TaToX16::Disable,
            true => TaToX16::Enable,
        }
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER is multiplied by 1x"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TaToX16::Disable
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER is multiplied by 16x"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TaToX16::Enable
    }
}
#[doc = "Field `TA_TO_X16` writer - 30:30\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER bit-field"]
pub type TaToX16W<'a, REG> = crate::BitWriter<'a, REG, TaToX16>;
impl<'a, REG> TaToX16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER is multiplied by 1x"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TaToX16::Disable)
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER is multiplied by 16x"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TaToX16::Enable)
    }
}
#[doc = "31:31\\]
Enables the turn-around timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaTo {
    #[doc = "0: Turn-around counter is disabled."]
    Deassertion = 0,
    #[doc = "1: Turn-around counter is enabled (required to receive TA interrupt in case the turn-around procedure is not successful)."]
    Assertion = 1,
}
impl From<TaTo> for bool {
    #[inline(always)]
    fn from(variant: TaTo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TA_TO` reader - 31:31\\]
Enables the turn-around timer"]
pub type TaToR = crate::BitReader<TaTo>;
impl TaToR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TaTo {
        match self.bits {
            false => TaTo::Deassertion,
            true => TaTo::Assertion,
        }
    }
    #[doc = "Turn-around counter is disabled."]
    #[inline(always)]
    pub fn is_deassertion(&self) -> bool {
        *self == TaTo::Deassertion
    }
    #[doc = "Turn-around counter is enabled (required to receive TA interrupt in case the turn-around procedure is not successful)."]
    #[inline(always)]
    pub fn is_assertion(&self) -> bool {
        *self == TaTo::Assertion
    }
}
#[doc = "Field `TA_TO` writer - 31:31\\]
Enables the turn-around timer"]
pub type TaToW<'a, REG> = crate::BitWriter<'a, REG, TaTo>;
impl<'a, REG> TaToW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Turn-around counter is disabled."]
    #[inline(always)]
    pub fn deassertion(self) -> &'a mut crate::W<REG> {
        self.variant(TaTo::Deassertion)
    }
    #[doc = "Turn-around counter is enabled (required to receive TA interrupt in case the turn-around procedure is not successful)."]
    #[inline(always)]
    pub fn assertion(self) -> &'a mut crate::W<REG> {
        self.variant(TaTo::Assertion)
    }
}
impl R {
    #[doc = "Bits 0:12 - 12:0\\]
Stop state counter. It indicates the number of CSI2_CLK function clock to assert ForceTXStopMode signal. The value is from 0 to 8191."]
    #[inline(always)]
    pub fn stop_state_counter_io(&self) -> StopStateCounterIoR {
        StopStateCounterIoR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - 13:13\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in STOP_STATE_COUNTER_IO bit-field"]
    #[inline(always)]
    pub fn stop_state_x4_io(&self) -> StopStateX4IoR {
        StopStateX4IoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in STOP_STATE_COUNTER_IO bit-field"]
    #[inline(always)]
    pub fn stop_state_x16_io(&self) -> StopStateX16IoR {
        StopStateX16IoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Control of ForceTxStopMode signal"]
    #[inline(always)]
    pub fn force_tx_stop_mode_io(&self) -> ForceTxStopModeIoR {
        ForceTxStopModeIoR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:28 - 28:16\\]
Turn around counter. It indicates the number of CSI2_CLK function clock to wait for the change of the Direction PPI signal according to the TurnRequest signal The value is from 0 to 8191."]
    #[inline(always)]
    pub fn ta_to_counter(&self) -> TaToCounterR {
        TaToCounterR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bit 29 - 29:29\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER bit-field"]
    #[inline(always)]
    pub fn ta_to_x8(&self) -> TaToX8R {
        TaToX8R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER bit-field"]
    #[inline(always)]
    pub fn ta_to_x16(&self) -> TaToX16R {
        TaToX16R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Enables the turn-around timer"]
    #[inline(always)]
    pub fn ta_to(&self) -> TaToR {
        TaToR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - 12:0\\]
Stop state counter. It indicates the number of CSI2_CLK function clock to assert ForceTXStopMode signal. The value is from 0 to 8191."]
    #[inline(always)]
    #[must_use]
    pub fn stop_state_counter_io(&mut self) -> StopStateCounterIoW<Csi2Timing1Spec> {
        StopStateCounterIoW::new(self, 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in STOP_STATE_COUNTER_IO bit-field"]
    #[inline(always)]
    #[must_use]
    pub fn stop_state_x4_io(&mut self) -> StopStateX4IoW<Csi2Timing1Spec> {
        StopStateX4IoW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in STOP_STATE_COUNTER_IO bit-field"]
    #[inline(always)]
    #[must_use]
    pub fn stop_state_x16_io(&mut self) -> StopStateX16IoW<Csi2Timing1Spec> {
        StopStateX16IoW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Control of ForceTxStopMode signal"]
    #[inline(always)]
    #[must_use]
    pub fn force_tx_stop_mode_io(&mut self) -> ForceTxStopModeIoW<Csi2Timing1Spec> {
        ForceTxStopModeIoW::new(self, 15)
    }
    #[doc = "Bits 16:28 - 28:16\\]
Turn around counter. It indicates the number of CSI2_CLK function clock to wait for the change of the Direction PPI signal according to the TurnRequest signal The value is from 0 to 8191."]
    #[inline(always)]
    #[must_use]
    pub fn ta_to_counter(&mut self) -> TaToCounterW<Csi2Timing1Spec> {
        TaToCounterW::new(self, 16)
    }
    #[doc = "Bit 29 - 29:29\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER bit-field"]
    #[inline(always)]
    #[must_use]
    pub fn ta_to_x8(&mut self) -> TaToX8W<Csi2Timing1Spec> {
        TaToX8W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in TA_TO_COUNTER bit-field"]
    #[inline(always)]
    #[must_use]
    pub fn ta_to_x16(&mut self) -> TaToX16W<Csi2Timing1Spec> {
        TaToX16W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Enables the turn-around timer"]
    #[inline(always)]
    #[must_use]
    pub fn ta_to(&mut self) -> TaToW<Csi2Timing1Spec> {
        TaToW::new(self, 31)
    }
}
#[doc = "TIMING1 REGISTER This register controls the CSI2 Protocol Engine module timers. Any bit-field can be modified while CSI2_CTRL.IF_EN is set to '1'. It is used to indicate the number of CSI2_CLK functional clock cycles for the timers FORCE_TX_STOP_TIMER and TA_TO_TIMER\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_timing1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_timing1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2Timing1Spec;
impl crate::RegisterSpec for Csi2Timing1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_timing1::R`](R) reader structure"]
impl crate::Readable for Csi2Timing1Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_timing1::W`](W) writer structure"]
impl crate::Writable for Csi2Timing1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_TIMING1 to value 0"]
impl crate::Resettable for Csi2Timing1Spec {
    const RESET_VALUE: u32 = 0;
}
