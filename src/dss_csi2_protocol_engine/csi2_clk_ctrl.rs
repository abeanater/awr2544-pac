#[doc = "Register `CSI2_CLK_CTRL` reader"]
pub type R = crate::R<Csi2ClkCtrlSpec>;
#[doc = "Register `CSI2_CLK_CTRL` writer"]
pub type W = crate::W<Csi2ClkCtrlSpec>;
#[doc = "Field `LP_CLK_DIVISOR` reader - 12:0\\]
Defines the ratio to be used for the generation of the Low Power mode clock from CSI2 functional clock. The supported values are from 1 to 8191(the value 0 is invalid). The output frequency shall be in the range between 20 MHz and 32 kHz."]
pub type LpClkDivisorR = crate::FieldReader<u16>;
#[doc = "Field `LP_CLK_DIVISOR` writer - 12:0\\]
Defines the ratio to be used for the generation of the Low Power mode clock from CSI2 functional clock. The supported values are from 1 to 8191(the value 0 is invalid). The output frequency shall be in the range between 20 MHz and 32 kHz."]
pub type LpClkDivisorW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "13:13\\]
Defines if the DDR clock is also sent when there is no HS packets sent to the peripheral (low power mode). So TXRequest for the clock lane is not de-asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DdrClkAlwaysOn {
    #[doc = "0: Disabled. The DDR clock is only provided when HS packets are sent."]
    Disable = 0,
    #[doc = "1: Enabled. The DDR clock is always sent to the peripheral regardless of the state of the data lanes (HS or LS mode)."]
    Enable = 1,
}
impl From<DdrClkAlwaysOn> for bool {
    #[inline(always)]
    fn from(variant: DdrClkAlwaysOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDR_CLK_ALWAYS_ON` reader - 13:13\\]
Defines if the DDR clock is also sent when there is no HS packets sent to the peripheral (low power mode). So TXRequest for the clock lane is not de-asserted."]
pub type DdrClkAlwaysOnR = crate::BitReader<DdrClkAlwaysOn>;
impl DdrClkAlwaysOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DdrClkAlwaysOn {
        match self.bits {
            false => DdrClkAlwaysOn::Disable,
            true => DdrClkAlwaysOn::Enable,
        }
    }
    #[doc = "Disabled. The DDR clock is only provided when HS packets are sent."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DdrClkAlwaysOn::Disable
    }
    #[doc = "Enabled. The DDR clock is always sent to the peripheral regardless of the state of the data lanes (HS or LS mode)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DdrClkAlwaysOn::Enable
    }
}
#[doc = "Field `DDR_CLK_ALWAYS_ON` writer - 13:13\\]
Defines if the DDR clock is also sent when there is no HS packets sent to the peripheral (low power mode). So TXRequest for the clock lane is not de-asserted."]
pub type DdrClkAlwaysOnW<'a, REG> = crate::BitWriter<'a, REG, DdrClkAlwaysOn>;
impl<'a, REG> DdrClkAlwaysOnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The DDR clock is only provided when HS packets are sent."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DdrClkAlwaysOn::Disable)
    }
    #[doc = "Enabled. The DDR clock is always sent to the peripheral regardless of the state of the data lanes (HS or LS mode)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DdrClkAlwaysOn::Enable)
    }
}
#[doc = "14:14\\]
Gates SCPClk clock provided to CSI2-PHY and PLL-CTRL module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CioClkIcg {
    #[doc = "0: Disabled. SCPClk is not generated. It remains at 0."]
    Disable = 0,
    #[doc = "1: Enabled. SCPClk is generated (OCP_CLK/4)"]
    Enable = 1,
}
impl From<CioClkIcg> for bool {
    #[inline(always)]
    fn from(variant: CioClkIcg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIO_CLK_ICG` reader - 14:14\\]
Gates SCPClk clock provided to CSI2-PHY and PLL-CTRL module."]
pub type CioClkIcgR = crate::BitReader<CioClkIcg>;
impl CioClkIcgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CioClkIcg {
        match self.bits {
            false => CioClkIcg::Disable,
            true => CioClkIcg::Enable,
        }
    }
    #[doc = "Disabled. SCPClk is not generated. It remains at 0."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CioClkIcg::Disable
    }
    #[doc = "Enabled. SCPClk is generated (OCP_CLK/4)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CioClkIcg::Enable
    }
}
#[doc = "Field `CIO_CLK_ICG` writer - 14:14\\]
Gates SCPClk clock provided to CSI2-PHY and PLL-CTRL module."]
pub type CioClkIcgW<'a, REG> = crate::BitWriter<'a, REG, CioClkIcg>;
impl<'a, REG> CioClkIcgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. SCPClk is not generated. It remains at 0."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CioClkIcg::Disable)
    }
    #[doc = "Enabled. SCPClk is generated (OCP_CLK/4)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CioClkIcg::Enable)
    }
}
#[doc = "15:15\\]
Enables the generation of NULL packet in low speed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpClkNullPacketEnable {
    #[doc = "0: Disabled. The NULL packet is not sent in LP mode after the last LP packet."]
    Disable = 0,
    #[doc = "1: Enabled. The NULL packet is sent in LP mode after the last LP packet."]
    Enable = 1,
}
impl From<LpClkNullPacketEnable> for bool {
    #[inline(always)]
    fn from(variant: LpClkNullPacketEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP_CLK_NULL_PACKET_ENABLE` reader - 15:15\\]
Enables the generation of NULL packet in low speed."]
pub type LpClkNullPacketEnableR = crate::BitReader<LpClkNullPacketEnable>;
impl LpClkNullPacketEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpClkNullPacketEnable {
        match self.bits {
            false => LpClkNullPacketEnable::Disable,
            true => LpClkNullPacketEnable::Enable,
        }
    }
    #[doc = "Disabled. The NULL packet is not sent in LP mode after the last LP packet."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LpClkNullPacketEnable::Disable
    }
    #[doc = "Enabled. The NULL packet is sent in LP mode after the last LP packet."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LpClkNullPacketEnable::Enable
    }
}
#[doc = "Field `LP_CLK_NULL_PACKET_ENABLE` writer - 15:15\\]
Enables the generation of NULL packet in low speed."]
pub type LpClkNullPacketEnableW<'a, REG> = crate::BitWriter<'a, REG, LpClkNullPacketEnable>;
impl<'a, REG> LpClkNullPacketEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The NULL packet is not sent in LP mode after the last LP packet."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LpClkNullPacketEnable::Disable)
    }
    #[doc = "Enabled. The NULL packet is sent in LP mode after the last LP packet."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LpClkNullPacketEnable::Enable)
    }
}
#[doc = "Field `LP_CLK_NULL_PACKET_SIZE` reader - 17:16\\]
Indicates the size of LP NULL Packets to be sent automatically when after the last LP packet transfer. It is used by the receiver to drain its internal pipeline. The valid values are from 0 to 3 bytes for the payload size."]
pub type LpClkNullPacketSizeR = crate::FieldReader;
#[doc = "Field `LP_CLK_NULL_PACKET_SIZE` writer - 17:16\\]
Indicates the size of LP NULL Packets to be sent automatically when after the last LP packet transfer. It is used by the receiver to drain its internal pipeline. The valid values are from 0 to 3 bytes for the payload size."]
pub type LpClkNullPacketSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "18:18\\]
Enables the automatic assertion/de-assertion of CSI2StopClk signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsAutoStopEnable {
    #[doc = "0: Auto mode disabled."]
    Disable = 0,
    #[doc = "1: Auto mode enabled."]
    Enable = 1,
}
impl From<HsAutoStopEnable> for bool {
    #[inline(always)]
    fn from(variant: HsAutoStopEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_AUTO_STOP_ENABLE` reader - 18:18\\]
Enables the automatic assertion/de-assertion of CSI2StopClk signal."]
pub type HsAutoStopEnableR = crate::BitReader<HsAutoStopEnable>;
impl HsAutoStopEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsAutoStopEnable {
        match self.bits {
            false => HsAutoStopEnable::Disable,
            true => HsAutoStopEnable::Enable,
        }
    }
    #[doc = "Auto mode disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HsAutoStopEnable::Disable
    }
    #[doc = "Auto mode enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HsAutoStopEnable::Enable
    }
}
#[doc = "Field `HS_AUTO_STOP_ENABLE` writer - 18:18\\]
Enables the automatic assertion/de-assertion of CSI2StopClk signal."]
pub type HsAutoStopEnableW<'a, REG> = crate::BitWriter<'a, REG, HsAutoStopEnable>;
impl<'a, REG> HsAutoStopEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HsAutoStopEnable::Disable)
    }
    #[doc = "Auto mode enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HsAutoStopEnable::Enable)
    }
}
#[doc = "19:19\\]
In case HS_AUTO_STOP_ENABLE=0, the bit-field allows manual control of the assertion/de-assertion of the signal CSI2StopClk by the user.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsManualStopCtrl {
    #[doc = "0: CSI2StopClk de-assertion unconditionally."]
    Deassertion = 0,
    #[doc = "1: CSI2StopClk assertion unconditionally."]
    Assertion = 1,
}
impl From<HsManualStopCtrl> for bool {
    #[inline(always)]
    fn from(variant: HsManualStopCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_MANUAL_STOP_CTRL` reader - 19:19\\]
In case HS_AUTO_STOP_ENABLE=0, the bit-field allows manual control of the assertion/de-assertion of the signal CSI2StopClk by the user."]
pub type HsManualStopCtrlR = crate::BitReader<HsManualStopCtrl>;
impl HsManualStopCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsManualStopCtrl {
        match self.bits {
            false => HsManualStopCtrl::Deassertion,
            true => HsManualStopCtrl::Assertion,
        }
    }
    #[doc = "CSI2StopClk de-assertion unconditionally."]
    #[inline(always)]
    pub fn is_deassertion(&self) -> bool {
        *self == HsManualStopCtrl::Deassertion
    }
    #[doc = "CSI2StopClk assertion unconditionally."]
    #[inline(always)]
    pub fn is_assertion(&self) -> bool {
        *self == HsManualStopCtrl::Assertion
    }
}
#[doc = "Field `HS_MANUAL_STOP_CTRL` writer - 19:19\\]
In case HS_AUTO_STOP_ENABLE=0, the bit-field allows manual control of the assertion/de-assertion of the signal CSI2StopClk by the user."]
pub type HsManualStopCtrlW<'a, REG> = crate::BitWriter<'a, REG, HsManualStopCtrl>;
impl<'a, REG> HsManualStopCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSI2StopClk de-assertion unconditionally."]
    #[inline(always)]
    pub fn deassertion(self) -> &'a mut crate::W<REG> {
        self.variant(HsManualStopCtrl::Deassertion)
    }
    #[doc = "CSI2StopClk assertion unconditionally."]
    #[inline(always)]
    pub fn assertion(self) -> &'a mut crate::W<REG> {
        self.variant(HsManualStopCtrl::Assertion)
    }
}
#[doc = "20:20\\]
Controls the gating of the TXCLKESC clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpClkEnable {
    #[doc = "0: Disabled. The clock is not generated. The value of LP_CLK_DIVISOR is not used and does not need to be programmed."]
    Disable = 0,
    #[doc = "1: Enabled. The clock is generated. The value of LP_CLK_DIVISOR is used and needs to be programmed."]
    Enable = 1,
}
impl From<LpClkEnable> for bool {
    #[inline(always)]
    fn from(variant: LpClkEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP_CLK_ENABLE` reader - 20:20\\]
Controls the gating of the TXCLKESC clock."]
pub type LpClkEnableR = crate::BitReader<LpClkEnable>;
impl LpClkEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpClkEnable {
        match self.bits {
            false => LpClkEnable::Disable,
            true => LpClkEnable::Enable,
        }
    }
    #[doc = "Disabled. The clock is not generated. The value of LP_CLK_DIVISOR is not used and does not need to be programmed."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LpClkEnable::Disable
    }
    #[doc = "Enabled. The clock is generated. The value of LP_CLK_DIVISOR is used and needs to be programmed."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LpClkEnable::Enable
    }
}
#[doc = "Field `LP_CLK_ENABLE` writer - 20:20\\]
Controls the gating of the TXCLKESC clock."]
pub type LpClkEnableW<'a, REG> = crate::BitWriter<'a, REG, LpClkEnable>;
impl<'a, REG> LpClkEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The clock is not generated. The value of LP_CLK_DIVISOR is not used and does not need to be programmed."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LpClkEnable::Disable)
    }
    #[doc = "Enabled. The clock is generated. The value of LP_CLK_DIVISOR is used and needs to be programmed."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LpClkEnable::Enable)
    }
}
#[doc = "21:21\\]
Defines if the functional is higher or lower than 30 MHz. The information is used to define synchronization to be used for RxValidEsc.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpRxSynchroEnable {
    #[doc = "0: The CSI2 functional clock is equal or slower than 30 MHz. The synchronization is falling/rising."]
    Lowspeed = 0,
    #[doc = "1: The CSI2 functional clock is higher than 30 MHz. The synchronization is rising/rising."]
    Highspeed = 1,
}
impl From<LpRxSynchroEnable> for bool {
    #[inline(always)]
    fn from(variant: LpRxSynchroEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP_RX_SYNCHRO_ENABLE` reader - 21:21\\]
Defines if the functional is higher or lower than 30 MHz. The information is used to define synchronization to be used for RxValidEsc."]
pub type LpRxSynchroEnableR = crate::BitReader<LpRxSynchroEnable>;
impl LpRxSynchroEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpRxSynchroEnable {
        match self.bits {
            false => LpRxSynchroEnable::Lowspeed,
            true => LpRxSynchroEnable::Highspeed,
        }
    }
    #[doc = "The CSI2 functional clock is equal or slower than 30 MHz. The synchronization is falling/rising."]
    #[inline(always)]
    pub fn is_lowspeed(&self) -> bool {
        *self == LpRxSynchroEnable::Lowspeed
    }
    #[doc = "The CSI2 functional clock is higher than 30 MHz. The synchronization is rising/rising."]
    #[inline(always)]
    pub fn is_highspeed(&self) -> bool {
        *self == LpRxSynchroEnable::Highspeed
    }
}
#[doc = "Field `LP_RX_SYNCHRO_ENABLE` writer - 21:21\\]
Defines if the functional is higher or lower than 30 MHz. The information is used to define synchronization to be used for RxValidEsc."]
pub type LpRxSynchroEnableW<'a, REG> = crate::BitWriter<'a, REG, LpRxSynchroEnable>;
impl<'a, REG> LpRxSynchroEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The CSI2 functional clock is equal or slower than 30 MHz. The synchronization is falling/rising."]
    #[inline(always)]
    pub fn lowspeed(self) -> &'a mut crate::W<REG> {
        self.variant(LpRxSynchroEnable::Lowspeed)
    }
    #[doc = "The CSI2 functional clock is higher than 30 MHz. The synchronization is rising/rising."]
    #[inline(always)]
    pub fn highspeed(self) -> &'a mut crate::W<REG> {
        self.variant(LpRxSynchroEnable::Highspeed)
    }
}
#[doc = "29:28\\]
Status of the power control of the CSI2 PLL Control module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PllPwrStatus {
    #[doc = "0: CSI2 PLL Control module in OFF state"]
    StateOff = 0,
    #[doc = "1: CSI2 PLL Control module in ON state for PLL only (HSDIVISER is OFF)"]
    StateOnHsclk = 1,
    #[doc = "2: CSI2 PLL Control module in ON state for both PLL and HSDIVISER"]
    StateOnAll = 2,
    #[doc = "3: CSI2 PLL Control module in ON state for both PLL and HSDIVISER (no clock output to the CSI2 complex IO)"]
    StateOnDiv = 3,
}
impl From<PllPwrStatus> for u8 {
    #[inline(always)]
    fn from(variant: PllPwrStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PllPwrStatus {
    type Ux = u8;
}
impl crate::IsEnum for PllPwrStatus {}
#[doc = "Field `PLL_PWR_STATUS` reader - 29:28\\]
Status of the power control of the CSI2 PLL Control module"]
pub type PllPwrStatusR = crate::FieldReader<PllPwrStatus>;
impl PllPwrStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllPwrStatus {
        match self.bits {
            0 => PllPwrStatus::StateOff,
            1 => PllPwrStatus::StateOnHsclk,
            2 => PllPwrStatus::StateOnAll,
            3 => PllPwrStatus::StateOnDiv,
            _ => unreachable!(),
        }
    }
    #[doc = "CSI2 PLL Control module in OFF state"]
    #[inline(always)]
    pub fn is_state_off(&self) -> bool {
        *self == PllPwrStatus::StateOff
    }
    #[doc = "CSI2 PLL Control module in ON state for PLL only (HSDIVISER is OFF)"]
    #[inline(always)]
    pub fn is_state_on_hsclk(&self) -> bool {
        *self == PllPwrStatus::StateOnHsclk
    }
    #[doc = "CSI2 PLL Control module in ON state for both PLL and HSDIVISER"]
    #[inline(always)]
    pub fn is_state_on_all(&self) -> bool {
        *self == PllPwrStatus::StateOnAll
    }
    #[doc = "CSI2 PLL Control module in ON state for both PLL and HSDIVISER (no clock output to the CSI2 complex IO)"]
    #[inline(always)]
    pub fn is_state_on_div(&self) -> bool {
        *self == PllPwrStatus::StateOnDiv
    }
}
#[doc = "Field `PLL_PWR_STATUS` writer - 29:28\\]
Status of the power control of the CSI2 PLL Control module"]
pub type PllPwrStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2, PllPwrStatus, crate::Safe>;
impl<'a, REG> PllPwrStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSI2 PLL Control module in OFF state"]
    #[inline(always)]
    pub fn state_off(self) -> &'a mut crate::W<REG> {
        self.variant(PllPwrStatus::StateOff)
    }
    #[doc = "CSI2 PLL Control module in ON state for PLL only (HSDIVISER is OFF)"]
    #[inline(always)]
    pub fn state_on_hsclk(self) -> &'a mut crate::W<REG> {
        self.variant(PllPwrStatus::StateOnHsclk)
    }
    #[doc = "CSI2 PLL Control module in ON state for both PLL and HSDIVISER"]
    #[inline(always)]
    pub fn state_on_all(self) -> &'a mut crate::W<REG> {
        self.variant(PllPwrStatus::StateOnAll)
    }
    #[doc = "CSI2 PLL Control module in ON state for both PLL and HSDIVISER (no clock output to the CSI2 complex IO)"]
    #[inline(always)]
    pub fn state_on_div(self) -> &'a mut crate::W<REG> {
        self.variant(PllPwrStatus::StateOnDiv)
    }
}
#[doc = "31:30\\]
Command for power control of the CSI2 PLL Control module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PllPwrCmd {
    #[doc = "0: Command to change to OFF state"]
    StateOff = 0,
    #[doc = "1: Command to change to ON state for PLL only (HSDIVISER is OFF)"]
    StateOnHsclk = 1,
    #[doc = "2: Command to change to ON state for both PLL and HSDIVISER"]
    StateOnAll = 2,
    #[doc = "3: Command to change to ON state for both PLL and HSDIVISER (no clock output to the CSI2 complex IO)"]
    StateOnDiv = 3,
}
impl From<PllPwrCmd> for u8 {
    #[inline(always)]
    fn from(variant: PllPwrCmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PllPwrCmd {
    type Ux = u8;
}
impl crate::IsEnum for PllPwrCmd {}
#[doc = "Field `PLL_PWR_CMD` reader - 31:30\\]
Command for power control of the CSI2 PLL Control module"]
pub type PllPwrCmdR = crate::FieldReader<PllPwrCmd>;
impl PllPwrCmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllPwrCmd {
        match self.bits {
            0 => PllPwrCmd::StateOff,
            1 => PllPwrCmd::StateOnHsclk,
            2 => PllPwrCmd::StateOnAll,
            3 => PllPwrCmd::StateOnDiv,
            _ => unreachable!(),
        }
    }
    #[doc = "Command to change to OFF state"]
    #[inline(always)]
    pub fn is_state_off(&self) -> bool {
        *self == PllPwrCmd::StateOff
    }
    #[doc = "Command to change to ON state for PLL only (HSDIVISER is OFF)"]
    #[inline(always)]
    pub fn is_state_on_hsclk(&self) -> bool {
        *self == PllPwrCmd::StateOnHsclk
    }
    #[doc = "Command to change to ON state for both PLL and HSDIVISER"]
    #[inline(always)]
    pub fn is_state_on_all(&self) -> bool {
        *self == PllPwrCmd::StateOnAll
    }
    #[doc = "Command to change to ON state for both PLL and HSDIVISER (no clock output to the CSI2 complex IO)"]
    #[inline(always)]
    pub fn is_state_on_div(&self) -> bool {
        *self == PllPwrCmd::StateOnDiv
    }
}
#[doc = "Field `PLL_PWR_CMD` writer - 31:30\\]
Command for power control of the CSI2 PLL Control module"]
pub type PllPwrCmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, PllPwrCmd, crate::Safe>;
impl<'a, REG> PllPwrCmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Command to change to OFF state"]
    #[inline(always)]
    pub fn state_off(self) -> &'a mut crate::W<REG> {
        self.variant(PllPwrCmd::StateOff)
    }
    #[doc = "Command to change to ON state for PLL only (HSDIVISER is OFF)"]
    #[inline(always)]
    pub fn state_on_hsclk(self) -> &'a mut crate::W<REG> {
        self.variant(PllPwrCmd::StateOnHsclk)
    }
    #[doc = "Command to change to ON state for both PLL and HSDIVISER"]
    #[inline(always)]
    pub fn state_on_all(self) -> &'a mut crate::W<REG> {
        self.variant(PllPwrCmd::StateOnAll)
    }
    #[doc = "Command to change to ON state for both PLL and HSDIVISER (no clock output to the CSI2 complex IO)"]
    #[inline(always)]
    pub fn state_on_div(self) -> &'a mut crate::W<REG> {
        self.variant(PllPwrCmd::StateOnDiv)
    }
}
impl R {
    #[doc = "Bits 0:12 - 12:0\\]
Defines the ratio to be used for the generation of the Low Power mode clock from CSI2 functional clock. The supported values are from 1 to 8191(the value 0 is invalid). The output frequency shall be in the range between 20 MHz and 32 kHz."]
    #[inline(always)]
    pub fn lp_clk_divisor(&self) -> LpClkDivisorR {
        LpClkDivisorR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - 13:13\\]
Defines if the DDR clock is also sent when there is no HS packets sent to the peripheral (low power mode). So TXRequest for the clock lane is not de-asserted."]
    #[inline(always)]
    pub fn ddr_clk_always_on(&self) -> DdrClkAlwaysOnR {
        DdrClkAlwaysOnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Gates SCPClk clock provided to CSI2-PHY and PLL-CTRL module."]
    #[inline(always)]
    pub fn cio_clk_icg(&self) -> CioClkIcgR {
        CioClkIcgR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Enables the generation of NULL packet in low speed."]
    #[inline(always)]
    pub fn lp_clk_null_packet_enable(&self) -> LpClkNullPacketEnableR {
        LpClkNullPacketEnableR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Indicates the size of LP NULL Packets to be sent automatically when after the last LP packet transfer. It is used by the receiver to drain its internal pipeline. The valid values are from 0 to 3 bytes for the payload size."]
    #[inline(always)]
    pub fn lp_clk_null_packet_size(&self) -> LpClkNullPacketSizeR {
        LpClkNullPacketSizeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
Enables the automatic assertion/de-assertion of CSI2StopClk signal."]
    #[inline(always)]
    pub fn hs_auto_stop_enable(&self) -> HsAutoStopEnableR {
        HsAutoStopEnableR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
In case HS_AUTO_STOP_ENABLE=0, the bit-field allows manual control of the assertion/de-assertion of the signal CSI2StopClk by the user."]
    #[inline(always)]
    pub fn hs_manual_stop_ctrl(&self) -> HsManualStopCtrlR {
        HsManualStopCtrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Controls the gating of the TXCLKESC clock."]
    #[inline(always)]
    pub fn lp_clk_enable(&self) -> LpClkEnableR {
        LpClkEnableR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Defines if the functional is higher or lower than 30 MHz. The information is used to define synchronization to be used for RxValidEsc."]
    #[inline(always)]
    pub fn lp_rx_synchro_enable(&self) -> LpRxSynchroEnableR {
        LpRxSynchroEnableR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Status of the power control of the CSI2 PLL Control module"]
    #[inline(always)]
    pub fn pll_pwr_status(&self) -> PllPwrStatusR {
        PllPwrStatusR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Command for power control of the CSI2 PLL Control module"]
    #[inline(always)]
    pub fn pll_pwr_cmd(&self) -> PllPwrCmdR {
        PllPwrCmdR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - 12:0\\]
Defines the ratio to be used for the generation of the Low Power mode clock from CSI2 functional clock. The supported values are from 1 to 8191(the value 0 is invalid). The output frequency shall be in the range between 20 MHz and 32 kHz."]
    #[inline(always)]
    #[must_use]
    pub fn lp_clk_divisor(&mut self) -> LpClkDivisorW<Csi2ClkCtrlSpec> {
        LpClkDivisorW::new(self, 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Defines if the DDR clock is also sent when there is no HS packets sent to the peripheral (low power mode). So TXRequest for the clock lane is not de-asserted."]
    #[inline(always)]
    #[must_use]
    pub fn ddr_clk_always_on(&mut self) -> DdrClkAlwaysOnW<Csi2ClkCtrlSpec> {
        DdrClkAlwaysOnW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Gates SCPClk clock provided to CSI2-PHY and PLL-CTRL module."]
    #[inline(always)]
    #[must_use]
    pub fn cio_clk_icg(&mut self) -> CioClkIcgW<Csi2ClkCtrlSpec> {
        CioClkIcgW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Enables the generation of NULL packet in low speed."]
    #[inline(always)]
    #[must_use]
    pub fn lp_clk_null_packet_enable(&mut self) -> LpClkNullPacketEnableW<Csi2ClkCtrlSpec> {
        LpClkNullPacketEnableW::new(self, 15)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Indicates the size of LP NULL Packets to be sent automatically when after the last LP packet transfer. It is used by the receiver to drain its internal pipeline. The valid values are from 0 to 3 bytes for the payload size."]
    #[inline(always)]
    #[must_use]
    pub fn lp_clk_null_packet_size(&mut self) -> LpClkNullPacketSizeW<Csi2ClkCtrlSpec> {
        LpClkNullPacketSizeW::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
Enables the automatic assertion/de-assertion of CSI2StopClk signal."]
    #[inline(always)]
    #[must_use]
    pub fn hs_auto_stop_enable(&mut self) -> HsAutoStopEnableW<Csi2ClkCtrlSpec> {
        HsAutoStopEnableW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
In case HS_AUTO_STOP_ENABLE=0, the bit-field allows manual control of the assertion/de-assertion of the signal CSI2StopClk by the user."]
    #[inline(always)]
    #[must_use]
    pub fn hs_manual_stop_ctrl(&mut self) -> HsManualStopCtrlW<Csi2ClkCtrlSpec> {
        HsManualStopCtrlW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Controls the gating of the TXCLKESC clock."]
    #[inline(always)]
    #[must_use]
    pub fn lp_clk_enable(&mut self) -> LpClkEnableW<Csi2ClkCtrlSpec> {
        LpClkEnableW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Defines if the functional is higher or lower than 30 MHz. The information is used to define synchronization to be used for RxValidEsc."]
    #[inline(always)]
    #[must_use]
    pub fn lp_rx_synchro_enable(&mut self) -> LpRxSynchroEnableW<Csi2ClkCtrlSpec> {
        LpRxSynchroEnableW::new(self, 21)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Status of the power control of the CSI2 PLL Control module"]
    #[inline(always)]
    #[must_use]
    pub fn pll_pwr_status(&mut self) -> PllPwrStatusW<Csi2ClkCtrlSpec> {
        PllPwrStatusW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Command for power control of the CSI2 PLL Control module"]
    #[inline(always)]
    #[must_use]
    pub fn pll_pwr_cmd(&mut self) -> PllPwrCmdW<Csi2ClkCtrlSpec> {
        PllPwrCmdW::new(self, 30)
    }
}
#[doc = "CLOCK CONTROL This register controls the CLOCK GENERATION. The register can be modified only when IF_EN is reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_clk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_clk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2ClkCtrlSpec;
impl crate::RegisterSpec for Csi2ClkCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for Csi2ClkCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for Csi2ClkCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_CLK_CTRL to value 0"]
impl crate::Resettable for Csi2ClkCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
