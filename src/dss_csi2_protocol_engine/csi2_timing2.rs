#[doc = "Register `CSI2_TIMING2` reader"]
pub type R = crate::R<Csi2Timing2Spec>;
#[doc = "Register `CSI2_TIMING2` writer"]
pub type W = crate::W<Csi2Timing2Spec>;
#[doc = "Field `LP_RX_TO_COUNTER` reader - 12:0\\]
LP_RX_TIMER counter. It indicates the number of CSI2_CLK function clock for the LP RX timer. The value is from 0 to 8191."]
pub type LpRxToCounterR = crate::FieldReader<u16>;
#[doc = "Field `LP_RX_TO_COUNTER` writer - 12:0\\]
LP_RX_TIMER counter. It indicates the number of CSI2_CLK function clock for the LP RX timer. The value is from 0 to 8191."]
pub type LpRxToCounterW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "13:13\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in LP_RX_COUNTER bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpRxToX4 {
    #[doc = "0: The number of CSI2_CLK functional clock cycles defined in LP_RX_TO_COUNTER is multiplied by 1x"]
    Disable = 0,
    #[doc = "1: The number of CSI2_CLK functional clock cycles defined in LP_RX_TO_COUNTER is multiplied by 4x"]
    Enable = 1,
}
impl From<LpRxToX4> for bool {
    #[inline(always)]
    fn from(variant: LpRxToX4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP_RX_TO_X4` reader - 13:13\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in LP_RX_COUNTER bit"]
pub type LpRxToX4R = crate::BitReader<LpRxToX4>;
impl LpRxToX4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpRxToX4 {
        match self.bits {
            false => LpRxToX4::Disable,
            true => LpRxToX4::Enable,
        }
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in LP_RX_TO_COUNTER is multiplied by 1x"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LpRxToX4::Disable
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in LP_RX_TO_COUNTER is multiplied by 4x"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LpRxToX4::Enable
    }
}
#[doc = "Field `LP_RX_TO_X4` writer - 13:13\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in LP_RX_COUNTER bit"]
pub type LpRxToX4W<'a, REG> = crate::BitWriter<'a, REG, LpRxToX4>;
impl<'a, REG> LpRxToX4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of CSI2_CLK functional clock cycles defined in LP_RX_TO_COUNTER is multiplied by 1x"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LpRxToX4::Disable)
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in LP_RX_TO_COUNTER is multiplied by 4x"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LpRxToX4::Enable)
    }
}
#[doc = "14:14\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in LP_RX_COUNTER bit-field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpRxToX16 {
    #[doc = "0: The number of CSI2_CLK functional clock cycles defined in LP_RX_TO_COUNTER is multiplied by 1x"]
    Disable = 0,
    #[doc = "1: The number of CSI2_CLK functional clock cycles defined in LP_RX_TO_COUNTER is multiplied by 16x"]
    Enable = 1,
}
impl From<LpRxToX16> for bool {
    #[inline(always)]
    fn from(variant: LpRxToX16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP_RX_TO_X16` reader - 14:14\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in LP_RX_COUNTER bit-field"]
pub type LpRxToX16R = crate::BitReader<LpRxToX16>;
impl LpRxToX16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpRxToX16 {
        match self.bits {
            false => LpRxToX16::Disable,
            true => LpRxToX16::Enable,
        }
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in LP_RX_TO_COUNTER is multiplied by 1x"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LpRxToX16::Disable
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in LP_RX_TO_COUNTER is multiplied by 16x"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LpRxToX16::Enable
    }
}
#[doc = "Field `LP_RX_TO_X16` writer - 14:14\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in LP_RX_COUNTER bit-field"]
pub type LpRxToX16W<'a, REG> = crate::BitWriter<'a, REG, LpRxToX16>;
impl<'a, REG> LpRxToX16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of CSI2_CLK functional clock cycles defined in LP_RX_TO_COUNTER is multiplied by 1x"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LpRxToX16::Disable)
    }
    #[doc = "The number of CSI2_CLK functional clock cycles defined in LP_RX_TO_COUNTER is multiplied by 16x"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LpRxToX16::Enable)
    }
}
#[doc = "15:15\\]
Enables the LP RX timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpRxTo {
    #[doc = "0: Turn-around counter is disabled."]
    Deassertion = 0,
    #[doc = "1: Turn-around counter is enabled (required to receive TA interrupt in case the turn-around procedure is not successful)."]
    Assertion = 1,
}
impl From<LpRxTo> for bool {
    #[inline(always)]
    fn from(variant: LpRxTo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP_RX_TO` reader - 15:15\\]
Enables the LP RX timer."]
pub type LpRxToR = crate::BitReader<LpRxTo>;
impl LpRxToR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpRxTo {
        match self.bits {
            false => LpRxTo::Deassertion,
            true => LpRxTo::Assertion,
        }
    }
    #[doc = "Turn-around counter is disabled."]
    #[inline(always)]
    pub fn is_deassertion(&self) -> bool {
        *self == LpRxTo::Deassertion
    }
    #[doc = "Turn-around counter is enabled (required to receive TA interrupt in case the turn-around procedure is not successful)."]
    #[inline(always)]
    pub fn is_assertion(&self) -> bool {
        *self == LpRxTo::Assertion
    }
}
#[doc = "Field `LP_RX_TO` writer - 15:15\\]
Enables the LP RX timer."]
pub type LpRxToW<'a, REG> = crate::BitWriter<'a, REG, LpRxTo>;
impl<'a, REG> LpRxToW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Turn-around counter is disabled."]
    #[inline(always)]
    pub fn deassertion(self) -> &'a mut crate::W<REG> {
        self.variant(LpRxTo::Deassertion)
    }
    #[doc = "Turn-around counter is enabled (required to receive TA interrupt in case the turn-around procedure is not successful)."]
    #[inline(always)]
    pub fn assertion(self) -> &'a mut crate::W<REG> {
        self.variant(LpRxTo::Assertion)
    }
}
#[doc = "Field `HS_TX_TO_COUNTER` reader - 28:16\\]
HS_TX_TIMER counter. It indicates the number of BYTE_CLK function clock for the HS TX timer. The value is from 0 to 8191."]
pub type HsTxToCounterR = crate::FieldReader<u16>;
#[doc = "Field `HS_TX_TO_COUNTER` writer - 28:16\\]
HS_TX_TIMER counter. It indicates the number of BYTE_CLK function clock for the HS TX timer. The value is from 0 to 8191."]
pub type HsTxToCounterW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "29:29\\]
Multiplication factor for the number of BYTE_CLK functional clock cycles defined in HS_TX_COUNTER bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsTxToX16 {
    #[doc = "0: The number of BYTE_CLK functional clock cycles defined in HS_TX_TO_COUNTER is multiplied by 1x"]
    Disable = 0,
    #[doc = "1: The number of BYTE_CLK functional clock cycles defined in HS_TX_TO_COUNTER is multiplied by 16x"]
    Enable = 1,
}
impl From<HsTxToX16> for bool {
    #[inline(always)]
    fn from(variant: HsTxToX16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_TX_TO_X16` reader - 29:29\\]
Multiplication factor for the number of BYTE_CLK functional clock cycles defined in HS_TX_COUNTER bit"]
pub type HsTxToX16R = crate::BitReader<HsTxToX16>;
impl HsTxToX16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsTxToX16 {
        match self.bits {
            false => HsTxToX16::Disable,
            true => HsTxToX16::Enable,
        }
    }
    #[doc = "The number of BYTE_CLK functional clock cycles defined in HS_TX_TO_COUNTER is multiplied by 1x"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HsTxToX16::Disable
    }
    #[doc = "The number of BYTE_CLK functional clock cycles defined in HS_TX_TO_COUNTER is multiplied by 16x"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HsTxToX16::Enable
    }
}
#[doc = "Field `HS_TX_TO_X16` writer - 29:29\\]
Multiplication factor for the number of BYTE_CLK functional clock cycles defined in HS_TX_COUNTER bit"]
pub type HsTxToX16W<'a, REG> = crate::BitWriter<'a, REG, HsTxToX16>;
impl<'a, REG> HsTxToX16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of BYTE_CLK functional clock cycles defined in HS_TX_TO_COUNTER is multiplied by 1x"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HsTxToX16::Disable)
    }
    #[doc = "The number of BYTE_CLK functional clock cycles defined in HS_TX_TO_COUNTER is multiplied by 16x"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HsTxToX16::Enable)
    }
}
#[doc = "30:30\\]
Multiplication factor for the number of BYTE_CLK functional clock cycles defined in HS_TX_COUNTER bit-field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsTxToX64 {
    #[doc = "0: The number of BYTE_CLK functional clock cycles defined in HS_TX_TO_COUNTER is multiplied by 1x"]
    Disable = 0,
    #[doc = "1: The number of BYTE_CLK functional clock cycles defined in HS_TX_TO_COUNTER is multiplied by 64x"]
    Enable = 1,
}
impl From<HsTxToX64> for bool {
    #[inline(always)]
    fn from(variant: HsTxToX64) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_TX_TO_X64` reader - 30:30\\]
Multiplication factor for the number of BYTE_CLK functional clock cycles defined in HS_TX_COUNTER bit-field"]
pub type HsTxToX64R = crate::BitReader<HsTxToX64>;
impl HsTxToX64R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsTxToX64 {
        match self.bits {
            false => HsTxToX64::Disable,
            true => HsTxToX64::Enable,
        }
    }
    #[doc = "The number of BYTE_CLK functional clock cycles defined in HS_TX_TO_COUNTER is multiplied by 1x"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HsTxToX64::Disable
    }
    #[doc = "The number of BYTE_CLK functional clock cycles defined in HS_TX_TO_COUNTER is multiplied by 64x"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HsTxToX64::Enable
    }
}
#[doc = "Field `HS_TX_TO_X64` writer - 30:30\\]
Multiplication factor for the number of BYTE_CLK functional clock cycles defined in HS_TX_COUNTER bit-field"]
pub type HsTxToX64W<'a, REG> = crate::BitWriter<'a, REG, HsTxToX64>;
impl<'a, REG> HsTxToX64W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of BYTE_CLK functional clock cycles defined in HS_TX_TO_COUNTER is multiplied by 1x"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HsTxToX64::Disable)
    }
    #[doc = "The number of BYTE_CLK functional clock cycles defined in HS_TX_TO_COUNTER is multiplied by 64x"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HsTxToX64::Enable)
    }
}
#[doc = "31:31\\]
Enables the HS TX timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsTxTo {
    #[doc = "0: Time-out counter is disabled."]
    Deassertion = 0,
    #[doc = "1: Time-out counter is enabled (required to receive TA interrupt in case the turn-around procedure is not successful)."]
    Assertion = 1,
}
impl From<HsTxTo> for bool {
    #[inline(always)]
    fn from(variant: HsTxTo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_TX_TO` reader - 31:31\\]
Enables the HS TX timer."]
pub type HsTxToR = crate::BitReader<HsTxTo>;
impl HsTxToR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsTxTo {
        match self.bits {
            false => HsTxTo::Deassertion,
            true => HsTxTo::Assertion,
        }
    }
    #[doc = "Time-out counter is disabled."]
    #[inline(always)]
    pub fn is_deassertion(&self) -> bool {
        *self == HsTxTo::Deassertion
    }
    #[doc = "Time-out counter is enabled (required to receive TA interrupt in case the turn-around procedure is not successful)."]
    #[inline(always)]
    pub fn is_assertion(&self) -> bool {
        *self == HsTxTo::Assertion
    }
}
#[doc = "Field `HS_TX_TO` writer - 31:31\\]
Enables the HS TX timer."]
pub type HsTxToW<'a, REG> = crate::BitWriter<'a, REG, HsTxTo>;
impl<'a, REG> HsTxToW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Time-out counter is disabled."]
    #[inline(always)]
    pub fn deassertion(self) -> &'a mut crate::W<REG> {
        self.variant(HsTxTo::Deassertion)
    }
    #[doc = "Time-out counter is enabled (required to receive TA interrupt in case the turn-around procedure is not successful)."]
    #[inline(always)]
    pub fn assertion(self) -> &'a mut crate::W<REG> {
        self.variant(HsTxTo::Assertion)
    }
}
impl R {
    #[doc = "Bits 0:12 - 12:0\\]
LP_RX_TIMER counter. It indicates the number of CSI2_CLK function clock for the LP RX timer. The value is from 0 to 8191."]
    #[inline(always)]
    pub fn lp_rx_to_counter(&self) -> LpRxToCounterR {
        LpRxToCounterR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - 13:13\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in LP_RX_COUNTER bit"]
    #[inline(always)]
    pub fn lp_rx_to_x4(&self) -> LpRxToX4R {
        LpRxToX4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in LP_RX_COUNTER bit-field"]
    #[inline(always)]
    pub fn lp_rx_to_x16(&self) -> LpRxToX16R {
        LpRxToX16R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Enables the LP RX timer."]
    #[inline(always)]
    pub fn lp_rx_to(&self) -> LpRxToR {
        LpRxToR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:28 - 28:16\\]
HS_TX_TIMER counter. It indicates the number of BYTE_CLK function clock for the HS TX timer. The value is from 0 to 8191."]
    #[inline(always)]
    pub fn hs_tx_to_counter(&self) -> HsTxToCounterR {
        HsTxToCounterR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bit 29 - 29:29\\]
Multiplication factor for the number of BYTE_CLK functional clock cycles defined in HS_TX_COUNTER bit"]
    #[inline(always)]
    pub fn hs_tx_to_x16(&self) -> HsTxToX16R {
        HsTxToX16R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Multiplication factor for the number of BYTE_CLK functional clock cycles defined in HS_TX_COUNTER bit-field"]
    #[inline(always)]
    pub fn hs_tx_to_x64(&self) -> HsTxToX64R {
        HsTxToX64R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Enables the HS TX timer."]
    #[inline(always)]
    pub fn hs_tx_to(&self) -> HsTxToR {
        HsTxToR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - 12:0\\]
LP_RX_TIMER counter. It indicates the number of CSI2_CLK function clock for the LP RX timer. The value is from 0 to 8191."]
    #[inline(always)]
    #[must_use]
    pub fn lp_rx_to_counter(&mut self) -> LpRxToCounterW<Csi2Timing2Spec> {
        LpRxToCounterW::new(self, 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in LP_RX_COUNTER bit"]
    #[inline(always)]
    #[must_use]
    pub fn lp_rx_to_x4(&mut self) -> LpRxToX4W<Csi2Timing2Spec> {
        LpRxToX4W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Multiplication factor for the number of CSI2_CLK functional clock cycles defined in LP_RX_COUNTER bit-field"]
    #[inline(always)]
    #[must_use]
    pub fn lp_rx_to_x16(&mut self) -> LpRxToX16W<Csi2Timing2Spec> {
        LpRxToX16W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Enables the LP RX timer."]
    #[inline(always)]
    #[must_use]
    pub fn lp_rx_to(&mut self) -> LpRxToW<Csi2Timing2Spec> {
        LpRxToW::new(self, 15)
    }
    #[doc = "Bits 16:28 - 28:16\\]
HS_TX_TIMER counter. It indicates the number of BYTE_CLK function clock for the HS TX timer. The value is from 0 to 8191."]
    #[inline(always)]
    #[must_use]
    pub fn hs_tx_to_counter(&mut self) -> HsTxToCounterW<Csi2Timing2Spec> {
        HsTxToCounterW::new(self, 16)
    }
    #[doc = "Bit 29 - 29:29\\]
Multiplication factor for the number of BYTE_CLK functional clock cycles defined in HS_TX_COUNTER bit"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tx_to_x16(&mut self) -> HsTxToX16W<Csi2Timing2Spec> {
        HsTxToX16W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Multiplication factor for the number of BYTE_CLK functional clock cycles defined in HS_TX_COUNTER bit-field"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tx_to_x64(&mut self) -> HsTxToX64W<Csi2Timing2Spec> {
        HsTxToX64W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Enables the HS TX timer."]
    #[inline(always)]
    #[must_use]
    pub fn hs_tx_to(&mut self) -> HsTxToW<Csi2Timing2Spec> {
        HsTxToW::new(self, 31)
    }
}
#[doc = "TIMING2 REGISTER This register controls the CSI2 Protocol Engine module timers. Any bit-field can be modified while CSI2_CTRL.IF_EN is set to '1'. It is used to indicate the number of CSI2_CLK functional clock cycles for the timers HS_TX_TIMER and LP_RX_TIMER\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_timing2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_timing2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2Timing2Spec;
impl crate::RegisterSpec for Csi2Timing2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_timing2::R`](R) reader structure"]
impl crate::Readable for Csi2Timing2Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_timing2::W`](W) writer structure"]
impl crate::Writable for Csi2Timing2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_TIMING2 to value 0"]
impl crate::Resettable for Csi2Timing2Spec {
    const RESET_VALUE: u32 = 0;
}
