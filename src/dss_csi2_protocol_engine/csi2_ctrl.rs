#[doc = "Register `CSI2_CTRL` reader"]
pub type R = crate::R<Csi2CtrlSpec>;
#[doc = "Register `CSI2_CTRL` writer"]
pub type W = crate::W<Csi2CtrlSpec>;
#[doc = "0:0\\]
Enables the module. When the module is disabled the signals from the complex IO are gated (no updates of the interrupt status register). It is not possible to change the bit-fields in the register CSI2_CTRL except IF_EN when it is enabled. All the other registers can be changed except the ones that require CSI2_VC_CTRL.VC_EN to be equal to 0 to be modified.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfEn {
    #[doc = "0: The interface is disabled. If one of the virtual channel uses the video mode with the video port to receive the data, the CSI2 protocol engines is disabled when the next VSYNC is received and all the data in the FIFO for the other virtual channels in command mode are sent to the peripherals (if BTA_EN is enabled, the CSI2 protocol needs to wait for the response and BTA from the peripheral before disabling all the internal logic since an acknowledge is requested)."]
    Disable = 0,
    #[doc = "1: The interface is enabled immediately, the data acquisition on the video port starts on the next VSYNC (video mode) or first data received in the Slave port FIFO (command mode)."]
    Enable = 1,
}
impl From<IfEn> for bool {
    #[inline(always)]
    fn from(variant: IfEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF_EN` reader - 0:0\\]
Enables the module. When the module is disabled the signals from the complex IO are gated (no updates of the interrupt status register). It is not possible to change the bit-fields in the register CSI2_CTRL except IF_EN when it is enabled. All the other registers can be changed except the ones that require CSI2_VC_CTRL.VC_EN to be equal to 0 to be modified."]
pub type IfEnR = crate::BitReader<IfEn>;
impl IfEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IfEn {
        match self.bits {
            false => IfEn::Disable,
            true => IfEn::Enable,
        }
    }
    #[doc = "The interface is disabled. If one of the virtual channel uses the video mode with the video port to receive the data, the CSI2 protocol engines is disabled when the next VSYNC is received and all the data in the FIFO for the other virtual channels in command mode are sent to the peripherals (if BTA_EN is enabled, the CSI2 protocol needs to wait for the response and BTA from the peripheral before disabling all the internal logic since an acknowledge is requested)."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IfEn::Disable
    }
    #[doc = "The interface is enabled immediately, the data acquisition on the video port starts on the next VSYNC (video mode) or first data received in the Slave port FIFO (command mode)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IfEn::Enable
    }
}
#[doc = "Field `IF_EN` writer - 0:0\\]
Enables the module. When the module is disabled the signals from the complex IO are gated (no updates of the interrupt status register). It is not possible to change the bit-fields in the register CSI2_CTRL except IF_EN when it is enabled. All the other registers can be changed except the ones that require CSI2_VC_CTRL.VC_EN to be equal to 0 to be modified."]
pub type IfEnW<'a, REG> = crate::BitWriter<'a, REG, IfEn>;
impl<'a, REG> IfEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interface is disabled. If one of the virtual channel uses the video mode with the video port to receive the data, the CSI2 protocol engines is disabled when the next VSYNC is received and all the data in the FIFO for the other virtual channels in command mode are sent to the peripherals (if BTA_EN is enabled, the CSI2 protocol needs to wait for the response and BTA from the peripheral before disabling all the internal logic since an acknowledge is requested)."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IfEn::Disable)
    }
    #[doc = "The interface is enabled immediately, the data acquisition on the video port starts on the next VSYNC (video mode) or first data received in the Slave port FIFO (command mode)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IfEn::Enable)
    }
}
#[doc = "1:1\\]
Enables the checksum check for the received payload (long packet only for all virtual channel ids).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsRxEn {
    #[doc = "0: Disabled"]
    Disable = 0,
    #[doc = "1: Enabled"]
    Enable = 1,
}
impl From<CsRxEn> for bool {
    #[inline(always)]
    fn from(variant: CsRxEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS_RX_EN` reader - 1:1\\]
Enables the checksum check for the received payload (long packet only for all virtual channel ids)."]
pub type CsRxEnR = crate::BitReader<CsRxEn>;
impl CsRxEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CsRxEn {
        match self.bits {
            false => CsRxEn::Disable,
            true => CsRxEn::Enable,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CsRxEn::Disable
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CsRxEn::Enable
    }
}
#[doc = "Field `CS_RX_EN` writer - 1:1\\]
Enables the checksum check for the received payload (long packet only for all virtual channel ids)."]
pub type CsRxEnW<'a, REG> = crate::BitWriter<'a, REG, CsRxEn>;
impl<'a, REG> CsRxEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CsRxEn::Disable)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CsRxEn::Enable)
    }
}
#[doc = "2:2\\]
Enables the Error Correction Code check for the received header (short and long packets for all virtual channel ids).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EccRxEn {
    #[doc = "0: Disabled"]
    Disable = 0,
    #[doc = "1: Enabled"]
    Enable = 1,
}
impl From<EccRxEn> for bool {
    #[inline(always)]
    fn from(variant: EccRxEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECC_RX_EN` reader - 2:2\\]
Enables the Error Correction Code check for the received header (short and long packets for all virtual channel ids)."]
pub type EccRxEnR = crate::BitReader<EccRxEn>;
impl EccRxEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EccRxEn {
        match self.bits {
            false => EccRxEn::Disable,
            true => EccRxEn::Enable,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EccRxEn::Disable
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EccRxEn::Enable
    }
}
#[doc = "Field `ECC_RX_EN` writer - 2:2\\]
Enables the Error Correction Code check for the received header (short and long packets for all virtual channel ids)."]
pub type EccRxEnW<'a, REG> = crate::BitWriter<'a, REG, EccRxEn>;
impl<'a, REG> EccRxEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EccRxEn::Disable)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EccRxEn::Enable)
    }
}
#[doc = "3:3\\]
Defines the arbitration scheme for granting the virtual channel pending ready requests in the TX FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxFifoArbitration {
    #[doc = "0: Round-Robin Scheme is used"]
    Roundrobin = 0,
    #[doc = "1: Sequential Scheme is used"]
    Sequential = 1,
}
impl From<TxFifoArbitration> for bool {
    #[inline(always)]
    fn from(variant: TxFifoArbitration) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_FIFO_ARBITRATION` reader - 3:3\\]
Defines the arbitration scheme for granting the virtual channel pending ready requests in the TX FIFO"]
pub type TxFifoArbitrationR = crate::BitReader<TxFifoArbitration>;
impl TxFifoArbitrationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxFifoArbitration {
        match self.bits {
            false => TxFifoArbitration::Roundrobin,
            true => TxFifoArbitration::Sequential,
        }
    }
    #[doc = "Round-Robin Scheme is used"]
    #[inline(always)]
    pub fn is_roundrobin(&self) -> bool {
        *self == TxFifoArbitration::Roundrobin
    }
    #[doc = "Sequential Scheme is used"]
    #[inline(always)]
    pub fn is_sequential(&self) -> bool {
        *self == TxFifoArbitration::Sequential
    }
}
#[doc = "Field `TX_FIFO_ARBITRATION` writer - 3:3\\]
Defines the arbitration scheme for granting the virtual channel pending ready requests in the TX FIFO"]
pub type TxFifoArbitrationW<'a, REG> = crate::BitWriter<'a, REG, TxFifoArbitration>;
impl<'a, REG> TxFifoArbitrationW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Round-Robin Scheme is used"]
    #[inline(always)]
    pub fn roundrobin(self) -> &'a mut crate::W<REG> {
        self.variant(TxFifoArbitration::Roundrobin)
    }
    #[doc = "Sequential Scheme is used"]
    #[inline(always)]
    pub fn sequential(self) -> &'a mut crate::W<REG> {
        self.variant(TxFifoArbitration::Sequential)
    }
}
#[doc = "4:4\\]
The field indicates the clock ratio between VP.CLK and VP.PCLK. The clock VP.PCLK is generated from VP.CLK. It is divided down. The information is only used when the video port is used to provide data in command mode. In the case of video mode, it is not used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpClkRatio {
    #[doc = "0: The clock VP.PCLK is the clock VP.CLK divided by 2. The duty cycle of VP.PCLK is 50/50."]
    Ratio2 = 0,
    #[doc = "1: The clock VP.PCLK is the clock VP.CLK divided by 3 or more. The duty cycle of VP.PCLK is not 50/50 for odd ratio numbers (3,5,7,...)."]
    Ratio3andhigher = 1,
}
impl From<VpClkRatio> for bool {
    #[inline(always)]
    fn from(variant: VpClkRatio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_CLK_RATIO` reader - 4:4\\]
The field indicates the clock ratio between VP.CLK and VP.PCLK. The clock VP.PCLK is generated from VP.CLK. It is divided down. The information is only used when the video port is used to provide data in command mode. In the case of video mode, it is not used."]
pub type VpClkRatioR = crate::BitReader<VpClkRatio>;
impl VpClkRatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpClkRatio {
        match self.bits {
            false => VpClkRatio::Ratio2,
            true => VpClkRatio::Ratio3andhigher,
        }
    }
    #[doc = "The clock VP.PCLK is the clock VP.CLK divided by 2. The duty cycle of VP.PCLK is 50/50."]
    #[inline(always)]
    pub fn is_ratio2(&self) -> bool {
        *self == VpClkRatio::Ratio2
    }
    #[doc = "The clock VP.PCLK is the clock VP.CLK divided by 3 or more. The duty cycle of VP.PCLK is not 50/50 for odd ratio numbers (3,5,7,...)."]
    #[inline(always)]
    pub fn is_ratio3andhigher(&self) -> bool {
        *self == VpClkRatio::Ratio3andhigher
    }
}
#[doc = "Field `VP_CLK_RATIO` writer - 4:4\\]
The field indicates the clock ratio between VP.CLK and VP.PCLK. The clock VP.PCLK is generated from VP.CLK. It is divided down. The information is only used when the video port is used to provide data in command mode. In the case of video mode, it is not used."]
pub type VpClkRatioW<'a, REG> = crate::BitWriter<'a, REG, VpClkRatio>;
impl<'a, REG> VpClkRatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock VP.PCLK is the clock VP.CLK divided by 2. The duty cycle of VP.PCLK is 50/50."]
    #[inline(always)]
    pub fn ratio2(self) -> &'a mut crate::W<REG> {
        self.variant(VpClkRatio::Ratio2)
    }
    #[doc = "The clock VP.PCLK is the clock VP.CLK divided by 3 or more. The duty cycle of VP.PCLK is not 50/50 for odd ratio numbers (3,5,7,...)."]
    #[inline(always)]
    pub fn ratio3andhigher(self) -> &'a mut crate::W<REG> {
        self.variant(VpClkRatio::Ratio3andhigher)
    }
}
#[doc = "5:5\\]
Send the reset trigger to the peripheral.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriggerReset {
    #[doc = "0: READS: Reset trigger generation is completed. It is reset by HW when it is completed. WRITES: Cancellation of the request for Reset trigger generation (maybe too late since it is already on going)"]
    Disable = 0,
    #[doc = "1: READS: Generation of the reset trigger has been requested by user (could be on going but not completed yet). WRITES: Request for Reset trigger to be sent to the peripheral."]
    Enable = 1,
}
impl From<TriggerReset> for bool {
    #[inline(always)]
    fn from(variant: TriggerReset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGER_RESET` reader - 5:5\\]
Send the reset trigger to the peripheral."]
pub type TriggerResetR = crate::BitReader<TriggerReset>;
impl TriggerResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TriggerReset {
        match self.bits {
            false => TriggerReset::Disable,
            true => TriggerReset::Enable,
        }
    }
    #[doc = "READS: Reset trigger generation is completed. It is reset by HW when it is completed. WRITES: Cancellation of the request for Reset trigger generation (maybe too late since it is already on going)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TriggerReset::Disable
    }
    #[doc = "READS: Generation of the reset trigger has been requested by user (could be on going but not completed yet). WRITES: Request for Reset trigger to be sent to the peripheral."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TriggerReset::Enable
    }
}
#[doc = "Field `TRIGGER_RESET` writer - 5:5\\]
Send the reset trigger to the peripheral."]
pub type TriggerResetW<'a, REG> = crate::BitWriter<'a, REG, TriggerReset>;
impl<'a, REG> TriggerResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Reset trigger generation is completed. It is reset by HW when it is completed. WRITES: Cancellation of the request for Reset trigger generation (maybe too late since it is already on going)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TriggerReset::Disable)
    }
    #[doc = "READS: Generation of the reset trigger has been requested by user (could be on going but not completed yet). WRITES: Request for Reset trigger to be sent to the peripheral."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TriggerReset::Enable)
    }
}
#[doc = "7:6\\]
Defines the size of the video port data bus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VpDataBusWidth {
    #[doc = "0: 16-bits data width (LSB of the 24-bit video port data bus)"]
    F16 = 0,
    #[doc = "1: 18-bits data width (LSB of the 24-bit video port data bus)"]
    F18 = 1,
    #[doc = "2: 24-bits data width (LSB of the 24-bit video port data bus)"]
    F24 = 2,
}
impl From<VpDataBusWidth> for u8 {
    #[inline(always)]
    fn from(variant: VpDataBusWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VpDataBusWidth {
    type Ux = u8;
}
impl crate::IsEnum for VpDataBusWidth {}
#[doc = "Field `VP_DATA_BUS_WIDTH` reader - 7:6\\]
Defines the size of the video port data bus"]
pub type VpDataBusWidthR = crate::FieldReader<VpDataBusWidth>;
impl VpDataBusWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VpDataBusWidth> {
        match self.bits {
            0 => Some(VpDataBusWidth::F16),
            1 => Some(VpDataBusWidth::F18),
            2 => Some(VpDataBusWidth::F24),
            _ => None,
        }
    }
    #[doc = "16-bits data width (LSB of the 24-bit video port data bus)"]
    #[inline(always)]
    pub fn is_f16(&self) -> bool {
        *self == VpDataBusWidth::F16
    }
    #[doc = "18-bits data width (LSB of the 24-bit video port data bus)"]
    #[inline(always)]
    pub fn is_f18(&self) -> bool {
        *self == VpDataBusWidth::F18
    }
    #[doc = "24-bits data width (LSB of the 24-bit video port data bus)"]
    #[inline(always)]
    pub fn is_f24(&self) -> bool {
        *self == VpDataBusWidth::F24
    }
}
#[doc = "Field `VP_DATA_BUS_WIDTH` writer - 7:6\\]
Defines the size of the video port data bus"]
pub type VpDataBusWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, VpDataBusWidth>;
impl<'a, REG> VpDataBusWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bits data width (LSB of the 24-bit video port data bus)"]
    #[inline(always)]
    pub fn f16(self) -> &'a mut crate::W<REG> {
        self.variant(VpDataBusWidth::F16)
    }
    #[doc = "18-bits data width (LSB of the 24-bit video port data bus)"]
    #[inline(always)]
    pub fn f18(self) -> &'a mut crate::W<REG> {
        self.variant(VpDataBusWidth::F18)
    }
    #[doc = "24-bits data width (LSB of the 24-bit video port data bus)"]
    #[inline(always)]
    pub fn f24(self) -> &'a mut crate::W<REG> {
        self.variant(VpDataBusWidth::F24)
    }
}
#[doc = "8:8\\]
VP clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpClkPol {
    #[doc = "0: The CSI2 Protocol Engine module captures the data on the VP on the pixel clock falling edge. The module connected to the VP shall drive the data on the pixel clock rising edge."]
    Falling = 0,
    #[doc = "1: The CSI2 Protocol Engine module captures the data on the VP on the pixel clock raising edge. The module connected to the VP shall drive the data on the pixel clock falling edge."]
    Rising = 1,
}
impl From<VpClkPol> for bool {
    #[inline(always)]
    fn from(variant: VpClkPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_CLK_POL` reader - 8:8\\]
VP clock polarity"]
pub type VpClkPolR = crate::BitReader<VpClkPol>;
impl VpClkPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpClkPol {
        match self.bits {
            false => VpClkPol::Falling,
            true => VpClkPol::Rising,
        }
    }
    #[doc = "The CSI2 Protocol Engine module captures the data on the VP on the pixel clock falling edge. The module connected to the VP shall drive the data on the pixel clock rising edge."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == VpClkPol::Falling
    }
    #[doc = "The CSI2 Protocol Engine module captures the data on the VP on the pixel clock raising edge. The module connected to the VP shall drive the data on the pixel clock falling edge."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == VpClkPol::Rising
    }
}
#[doc = "Field `VP_CLK_POL` writer - 8:8\\]
VP clock polarity"]
pub type VpClkPolW<'a, REG> = crate::BitWriter<'a, REG, VpClkPol>;
impl<'a, REG> VpClkPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The CSI2 Protocol Engine module captures the data on the VP on the pixel clock falling edge. The module connected to the VP shall drive the data on the pixel clock rising edge."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(VpClkPol::Falling)
    }
    #[doc = "The CSI2 Protocol Engine module captures the data on the VP on the pixel clock raising edge. The module connected to the VP shall drive the data on the pixel clock falling edge."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(VpClkPol::Rising)
    }
}
#[doc = "9:9\\]
VP data enable signal polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpDePol {
    #[doc = "0: DE signal on the video port is active low."]
    Low = 0,
    #[doc = "1: DE signal on the video port is active high."]
    High = 1,
}
impl From<VpDePol> for bool {
    #[inline(always)]
    fn from(variant: VpDePol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_DE_POL` reader - 9:9\\]
VP data enable signal polarity"]
pub type VpDePolR = crate::BitReader<VpDePol>;
impl VpDePolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpDePol {
        match self.bits {
            false => VpDePol::Low,
            true => VpDePol::High,
        }
    }
    #[doc = "DE signal on the video port is active low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == VpDePol::Low
    }
    #[doc = "DE signal on the video port is active high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == VpDePol::High
    }
}
#[doc = "Field `VP_DE_POL` writer - 9:9\\]
VP data enable signal polarity"]
pub type VpDePolW<'a, REG> = crate::BitWriter<'a, REG, VpDePol>;
impl<'a, REG> VpDePolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DE signal on the video port is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(VpDePol::Low)
    }
    #[doc = "DE signal on the video port is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(VpDePol::High)
    }
}
#[doc = "10:10\\]
VP horizontal synchronization signal polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpHsyncPol {
    #[doc = "0: HSYNC signal on the video port is active low."]
    Low = 0,
    #[doc = "1: HSYNC signal on the video port is active high."]
    High = 1,
}
impl From<VpHsyncPol> for bool {
    #[inline(always)]
    fn from(variant: VpHsyncPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_HSYNC_POL` reader - 10:10\\]
VP horizontal synchronization signal polarity"]
pub type VpHsyncPolR = crate::BitReader<VpHsyncPol>;
impl VpHsyncPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpHsyncPol {
        match self.bits {
            false => VpHsyncPol::Low,
            true => VpHsyncPol::High,
        }
    }
    #[doc = "HSYNC signal on the video port is active low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == VpHsyncPol::Low
    }
    #[doc = "HSYNC signal on the video port is active high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == VpHsyncPol::High
    }
}
#[doc = "Field `VP_HSYNC_POL` writer - 10:10\\]
VP horizontal synchronization signal polarity"]
pub type VpHsyncPolW<'a, REG> = crate::BitWriter<'a, REG, VpHsyncPol>;
impl<'a, REG> VpHsyncPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSYNC signal on the video port is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(VpHsyncPol::Low)
    }
    #[doc = "HSYNC signal on the video port is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(VpHsyncPol::High)
    }
}
#[doc = "11:11\\]
VP vertical synchronization signal polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpVsyncPol {
    #[doc = "0: VSYNC signal on the video port is active low."]
    Low = 0,
    #[doc = "1: VSYNC signal on the video port is active high."]
    High = 1,
}
impl From<VpVsyncPol> for bool {
    #[inline(always)]
    fn from(variant: VpVsyncPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_VSYNC_POL` reader - 11:11\\]
VP vertical synchronization signal polarity"]
pub type VpVsyncPolR = crate::BitReader<VpVsyncPol>;
impl VpVsyncPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpVsyncPol {
        match self.bits {
            false => VpVsyncPol::Low,
            true => VpVsyncPol::High,
        }
    }
    #[doc = "VSYNC signal on the video port is active low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == VpVsyncPol::Low
    }
    #[doc = "VSYNC signal on the video port is active high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == VpVsyncPol::High
    }
}
#[doc = "Field `VP_VSYNC_POL` writer - 11:11\\]
VP vertical synchronization signal polarity"]
pub type VpVsyncPolW<'a, REG> = crate::BitWriter<'a, REG, VpVsyncPol>;
impl<'a, REG> VpVsyncPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VSYNC signal on the video port is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(VpVsyncPol::Low)
    }
    #[doc = "VSYNC signal on the video port is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(VpVsyncPol::High)
    }
}
#[doc = "13:12\\]
Number of line buffers to be used while receiving data on the video port. The valid values are from 0 to CSI2_GNQ.VP1_NB_LINE_BUFFER.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LineBuffer {
    #[doc = "0: No line buffer"]
    F0 = 0,
    #[doc = "1: 1 line buffer"]
    F1 = 1,
    #[doc = "2: 2 line buffers"]
    F2 = 2,
}
impl From<LineBuffer> for u8 {
    #[inline(always)]
    fn from(variant: LineBuffer) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LineBuffer {
    type Ux = u8;
}
impl crate::IsEnum for LineBuffer {}
#[doc = "Field `LINE_BUFFER` reader - 13:12\\]
Number of line buffers to be used while receiving data on the video port. The valid values are from 0 to CSI2_GNQ.VP1_NB_LINE_BUFFER."]
pub type LineBufferR = crate::FieldReader<LineBuffer>;
impl LineBufferR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LineBuffer> {
        match self.bits {
            0 => Some(LineBuffer::F0),
            1 => Some(LineBuffer::F1),
            2 => Some(LineBuffer::F2),
            _ => None,
        }
    }
    #[doc = "No line buffer"]
    #[inline(always)]
    pub fn is_f0(&self) -> bool {
        *self == LineBuffer::F0
    }
    #[doc = "1 line buffer"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == LineBuffer::F1
    }
    #[doc = "2 line buffers"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == LineBuffer::F2
    }
}
#[doc = "Field `LINE_BUFFER` writer - 13:12\\]
Number of line buffers to be used while receiving data on the video port. The valid values are from 0 to CSI2_GNQ.VP1_NB_LINE_BUFFER."]
pub type LineBufferW<'a, REG> = crate::FieldWriter<'a, REG, 2, LineBuffer>;
impl<'a, REG> LineBufferW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No line buffer"]
    #[inline(always)]
    pub fn f0(self) -> &'a mut crate::W<REG> {
        self.variant(LineBuffer::F0)
    }
    #[doc = "1 line buffer"]
    #[inline(always)]
    pub fn f1(self) -> &'a mut crate::W<REG> {
        self.variant(LineBuffer::F1)
    }
    #[doc = "2 line buffers"]
    #[inline(always)]
    pub fn f2(self) -> &'a mut crate::W<REG> {
        self.variant(LineBuffer::F2)
    }
}
#[doc = "14:14\\]
Selection of the trigger reset mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriggerResetMode {
    #[doc = "0: Synchronized: the mode is only valid if there is virtual channel using the video mode and it is active. The principal is to wait for the current video frame to be transferred on the link. Any data received after the VSYNC are ignored."]
    Synchronized = 0,
    #[doc = "1: Immediate: all pending requests in TX FIFO are taken into account for transfer scheduling, the RX FIFO is ignored, and the data from video port are ignored as soon as possible. Only the current transfer on CSI2 link and already scheduled ones are transmitted. All the other transfers are discarded."]
    Immediate = 1,
}
impl From<TriggerResetMode> for bool {
    #[inline(always)]
    fn from(variant: TriggerResetMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGER_RESET_MODE` reader - 14:14\\]
Selection of the trigger reset mode"]
pub type TriggerResetModeR = crate::BitReader<TriggerResetMode>;
impl TriggerResetModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TriggerResetMode {
        match self.bits {
            false => TriggerResetMode::Synchronized,
            true => TriggerResetMode::Immediate,
        }
    }
    #[doc = "Synchronized: the mode is only valid if there is virtual channel using the video mode and it is active. The principal is to wait for the current video frame to be transferred on the link. Any data received after the VSYNC are ignored."]
    #[inline(always)]
    pub fn is_synchronized(&self) -> bool {
        *self == TriggerResetMode::Synchronized
    }
    #[doc = "Immediate: all pending requests in TX FIFO are taken into account for transfer scheduling, the RX FIFO is ignored, and the data from video port are ignored as soon as possible. Only the current transfer on CSI2 link and already scheduled ones are transmitted. All the other transfers are discarded."]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == TriggerResetMode::Immediate
    }
}
#[doc = "Field `TRIGGER_RESET_MODE` writer - 14:14\\]
Selection of the trigger reset mode"]
pub type TriggerResetModeW<'a, REG> = crate::BitWriter<'a, REG, TriggerResetMode>;
impl<'a, REG> TriggerResetModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Synchronized: the mode is only valid if there is virtual channel using the video mode and it is active. The principal is to wait for the current video frame to be transferred on the link. Any data received after the VSYNC are ignored."]
    #[inline(always)]
    pub fn synchronized(self) -> &'a mut crate::W<REG> {
        self.variant(TriggerResetMode::Synchronized)
    }
    #[doc = "Immediate: all pending requests in TX FIFO are taken into account for transfer scheduling, the RX FIFO is ignored, and the data from video port are ignored as soon as possible. Only the current transfer on CSI2 link and already scheduled ones are transmitted. All the other transfers are discarded."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(TriggerResetMode::Immediate)
    }
}
#[doc = "15:15\\]
VSYNC start pulse.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpVsyncStart {
    #[doc = "0: Disabled. No VSYNC START short packet is generated."]
    Disable = 0,
    #[doc = "1: Enabled. While the VSYNC START pulse is detected, the associated short packet VSYNC START is generated."]
    Enable = 1,
}
impl From<VpVsyncStart> for bool {
    #[inline(always)]
    fn from(variant: VpVsyncStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_VSYNC_START` reader - 15:15\\]
VSYNC start pulse."]
pub type VpVsyncStartR = crate::BitReader<VpVsyncStart>;
impl VpVsyncStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpVsyncStart {
        match self.bits {
            false => VpVsyncStart::Disable,
            true => VpVsyncStart::Enable,
        }
    }
    #[doc = "Disabled. No VSYNC START short packet is generated."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VpVsyncStart::Disable
    }
    #[doc = "Enabled. While the VSYNC START pulse is detected, the associated short packet VSYNC START is generated."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VpVsyncStart::Enable
    }
}
#[doc = "Field `VP_VSYNC_START` writer - 15:15\\]
VSYNC start pulse."]
pub type VpVsyncStartW<'a, REG> = crate::BitWriter<'a, REG, VpVsyncStart>;
impl<'a, REG> VpVsyncStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. No VSYNC START short packet is generated."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(VpVsyncStart::Disable)
    }
    #[doc = "Enabled. While the VSYNC START pulse is detected, the associated short packet VSYNC START is generated."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(VpVsyncStart::Enable)
    }
}
#[doc = "16:16\\]
VSYNC end pulse.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpVsyncEnd {
    #[doc = "0: Disabled. No VSYNC END short packet is generated."]
    Disable = 0,
    #[doc = "1: Enabled. While the VSYNC END pulse is detected, the associated short packet VSYNC END is generated."]
    Enable = 1,
}
impl From<VpVsyncEnd> for bool {
    #[inline(always)]
    fn from(variant: VpVsyncEnd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_VSYNC_END` reader - 16:16\\]
VSYNC end pulse."]
pub type VpVsyncEndR = crate::BitReader<VpVsyncEnd>;
impl VpVsyncEndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpVsyncEnd {
        match self.bits {
            false => VpVsyncEnd::Disable,
            true => VpVsyncEnd::Enable,
        }
    }
    #[doc = "Disabled. No VSYNC END short packet is generated."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VpVsyncEnd::Disable
    }
    #[doc = "Enabled. While the VSYNC END pulse is detected, the associated short packet VSYNC END is generated."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VpVsyncEnd::Enable
    }
}
#[doc = "Field `VP_VSYNC_END` writer - 16:16\\]
VSYNC end pulse."]
pub type VpVsyncEndW<'a, REG> = crate::BitWriter<'a, REG, VpVsyncEnd>;
impl<'a, REG> VpVsyncEndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. No VSYNC END short packet is generated."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(VpVsyncEnd::Disable)
    }
    #[doc = "Enabled. While the VSYNC END pulse is detected, the associated short packet VSYNC END is generated."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(VpVsyncEnd::Enable)
    }
}
#[doc = "17:17\\]
HSYNC start pulse.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpHsyncStart {
    #[doc = "0: Disabled. No HSYNC START short packet is generated."]
    Disable = 0,
    #[doc = "1: Enabled. While the HSYNC start pulse is detected, the associated short packet HSYNC START is generated."]
    Enable = 1,
}
impl From<VpHsyncStart> for bool {
    #[inline(always)]
    fn from(variant: VpHsyncStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_HSYNC_START` reader - 17:17\\]
HSYNC start pulse."]
pub type VpHsyncStartR = crate::BitReader<VpHsyncStart>;
impl VpHsyncStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpHsyncStart {
        match self.bits {
            false => VpHsyncStart::Disable,
            true => VpHsyncStart::Enable,
        }
    }
    #[doc = "Disabled. No HSYNC START short packet is generated."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VpHsyncStart::Disable
    }
    #[doc = "Enabled. While the HSYNC start pulse is detected, the associated short packet HSYNC START is generated."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VpHsyncStart::Enable
    }
}
#[doc = "Field `VP_HSYNC_START` writer - 17:17\\]
HSYNC start pulse."]
pub type VpHsyncStartW<'a, REG> = crate::BitWriter<'a, REG, VpHsyncStart>;
impl<'a, REG> VpHsyncStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. No HSYNC START short packet is generated."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(VpHsyncStart::Disable)
    }
    #[doc = "Enabled. While the HSYNC start pulse is detected, the associated short packet HSYNC START is generated."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(VpHsyncStart::Enable)
    }
}
#[doc = "18:18\\]
HSYNC end pulse.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpHsyncEnd {
    #[doc = "0: Disabled. No HSYNC END short packet is generated."]
    Disable = 0,
    #[doc = "1: Enabled. While the HSYNC END pulse is detected, the associated short packet HSYNC END is generated."]
    Enable = 1,
}
impl From<VpHsyncEnd> for bool {
    #[inline(always)]
    fn from(variant: VpHsyncEnd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_HSYNC_END` reader - 18:18\\]
HSYNC end pulse."]
pub type VpHsyncEndR = crate::BitReader<VpHsyncEnd>;
impl VpHsyncEndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpHsyncEnd {
        match self.bits {
            false => VpHsyncEnd::Disable,
            true => VpHsyncEnd::Enable,
        }
    }
    #[doc = "Disabled. No HSYNC END short packet is generated."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VpHsyncEnd::Disable
    }
    #[doc = "Enabled. While the HSYNC END pulse is detected, the associated short packet HSYNC END is generated."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VpHsyncEnd::Enable
    }
}
#[doc = "Field `VP_HSYNC_END` writer - 18:18\\]
HSYNC end pulse."]
pub type VpHsyncEndW<'a, REG> = crate::BitWriter<'a, REG, VpHsyncEnd>;
impl<'a, REG> VpHsyncEndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. No HSYNC END short packet is generated."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(VpHsyncEnd::Disable)
    }
    #[doc = "Enabled. While the HSYNC END pulse is detected, the associated short packet HSYNC END is generated."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(VpHsyncEnd::Enable)
    }
}
#[doc = "19:19\\]
Enable EOT packets at the end of HS transmission.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EotEnable {
    #[doc = "0: No EOT packets"]
    EotOff = 0,
    #[doc = "1: EOT packet is sent at all HS to LP transitions."]
    EotOn = 1,
}
impl From<EotEnable> for bool {
    #[inline(always)]
    fn from(variant: EotEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOT_ENABLE` reader - 19:19\\]
Enable EOT packets at the end of HS transmission."]
pub type EotEnableR = crate::BitReader<EotEnable>;
impl EotEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EotEnable {
        match self.bits {
            false => EotEnable::EotOff,
            true => EotEnable::EotOn,
        }
    }
    #[doc = "No EOT packets"]
    #[inline(always)]
    pub fn is_eot_off(&self) -> bool {
        *self == EotEnable::EotOff
    }
    #[doc = "EOT packet is sent at all HS to LP transitions."]
    #[inline(always)]
    pub fn is_eot_on(&self) -> bool {
        *self == EotEnable::EotOn
    }
}
#[doc = "Field `EOT_ENABLE` writer - 19:19\\]
Enable EOT packets at the end of HS transmission."]
pub type EotEnableW<'a, REG> = crate::BitWriter<'a, REG, EotEnable>;
impl<'a, REG> EotEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No EOT packets"]
    #[inline(always)]
    pub fn eot_off(self) -> &'a mut crate::W<REG> {
        self.variant(EotEnable::EotOff)
    }
    #[doc = "EOT packet is sent at all HS to LP transitions."]
    #[inline(always)]
    pub fn eot_on(self) -> &'a mut crate::W<REG> {
        self.variant(EotEnable::EotOn)
    }
}
#[doc = "20:20\\]
Blanking mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlankingMode {
    #[doc = "0: LPS is used during blanking periods of video mode (except HSA, HBP, HFP defined in HSA_BLANKING_MODE, HBP_BLANKING_MODE AND HFP_BLANKING_MODE respectively) when there is no command mode data in TX FIFO ready to be sent. So blanking periods can be different during the frame depending on the TX FIFO."]
    Lps = 0,
    #[doc = "1: LONG BLANKING PACKETS are used during blanking periods of video mode (except HSA, HBP, HFP defined in HSA_BLANKING_MODE, HBP_BLANKING_MODE AND HFP_BLANKING_MODE respectively) regardless of the packets present in the TX FIFO ready to be sent"]
    Hs = 1,
}
impl From<BlankingMode> for bool {
    #[inline(always)]
    fn from(variant: BlankingMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLANKING_MODE` reader - 20:20\\]
Blanking mode"]
pub type BlankingModeR = crate::BitReader<BlankingMode>;
impl BlankingModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BlankingMode {
        match self.bits {
            false => BlankingMode::Lps,
            true => BlankingMode::Hs,
        }
    }
    #[doc = "LPS is used during blanking periods of video mode (except HSA, HBP, HFP defined in HSA_BLANKING_MODE, HBP_BLANKING_MODE AND HFP_BLANKING_MODE respectively) when there is no command mode data in TX FIFO ready to be sent. So blanking periods can be different during the frame depending on the TX FIFO."]
    #[inline(always)]
    pub fn is_lps(&self) -> bool {
        *self == BlankingMode::Lps
    }
    #[doc = "LONG BLANKING PACKETS are used during blanking periods of video mode (except HSA, HBP, HFP defined in HSA_BLANKING_MODE, HBP_BLANKING_MODE AND HFP_BLANKING_MODE respectively) regardless of the packets present in the TX FIFO ready to be sent"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == BlankingMode::Hs
    }
}
#[doc = "Field `BLANKING_MODE` writer - 20:20\\]
Blanking mode"]
pub type BlankingModeW<'a, REG> = crate::BitWriter<'a, REG, BlankingMode>;
impl<'a, REG> BlankingModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPS is used during blanking periods of video mode (except HSA, HBP, HFP defined in HSA_BLANKING_MODE, HBP_BLANKING_MODE AND HFP_BLANKING_MODE respectively) when there is no command mode data in TX FIFO ready to be sent. So blanking periods can be different during the frame depending on the TX FIFO."]
    #[inline(always)]
    pub fn lps(self) -> &'a mut crate::W<REG> {
        self.variant(BlankingMode::Lps)
    }
    #[doc = "LONG BLANKING PACKETS are used during blanking periods of video mode (except HSA, HBP, HFP defined in HSA_BLANKING_MODE, HBP_BLANKING_MODE AND HFP_BLANKING_MODE respectively) regardless of the packets present in the TX FIFO ready to be sent"]
    #[inline(always)]
    pub fn hs(self) -> &'a mut crate::W<REG> {
        self.variant(BlankingMode::Hs)
    }
}
#[doc = "21:21\\]
Blanking mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HfpBlankingMode {
    #[doc = "0: Packets in TX FIFO are sent during HFP blanking period of video mode or LPS is used."]
    CommandPacketTxFifo = 0,
    #[doc = "1: LONG BLANKING PACKETS only are used during HFP blanking period of video mode."]
    HsBlankingPacketOnly = 1,
}
impl From<HfpBlankingMode> for bool {
    #[inline(always)]
    fn from(variant: HfpBlankingMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFP_BLANKING_MODE` reader - 21:21\\]
Blanking mode"]
pub type HfpBlankingModeR = crate::BitReader<HfpBlankingMode>;
impl HfpBlankingModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HfpBlankingMode {
        match self.bits {
            false => HfpBlankingMode::CommandPacketTxFifo,
            true => HfpBlankingMode::HsBlankingPacketOnly,
        }
    }
    #[doc = "Packets in TX FIFO are sent during HFP blanking period of video mode or LPS is used."]
    #[inline(always)]
    pub fn is_command_packet_tx_fifo(&self) -> bool {
        *self == HfpBlankingMode::CommandPacketTxFifo
    }
    #[doc = "LONG BLANKING PACKETS only are used during HFP blanking period of video mode."]
    #[inline(always)]
    pub fn is_hs_blanking_packet_only(&self) -> bool {
        *self == HfpBlankingMode::HsBlankingPacketOnly
    }
}
#[doc = "Field `HFP_BLANKING_MODE` writer - 21:21\\]
Blanking mode"]
pub type HfpBlankingModeW<'a, REG> = crate::BitWriter<'a, REG, HfpBlankingMode>;
impl<'a, REG> HfpBlankingModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Packets in TX FIFO are sent during HFP blanking period of video mode or LPS is used."]
    #[inline(always)]
    pub fn command_packet_tx_fifo(self) -> &'a mut crate::W<REG> {
        self.variant(HfpBlankingMode::CommandPacketTxFifo)
    }
    #[doc = "LONG BLANKING PACKETS only are used during HFP blanking period of video mode."]
    #[inline(always)]
    pub fn hs_blanking_packet_only(self) -> &'a mut crate::W<REG> {
        self.variant(HfpBlankingMode::HsBlankingPacketOnly)
    }
}
#[doc = "22:22\\]
Blanking mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HbpBlankingMode {
    #[doc = "0: Packets in TX FIFO are sent during HBP blanking period of video mode or LPS is used."]
    CommandPacketTxFifo = 0,
    #[doc = "1: LONG BLANKING PACKETS only are used during HBP blanking period of video mode."]
    HsBlankingPacketOnly = 1,
}
impl From<HbpBlankingMode> for bool {
    #[inline(always)]
    fn from(variant: HbpBlankingMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HBP_BLANKING_MODE` reader - 22:22\\]
Blanking mode"]
pub type HbpBlankingModeR = crate::BitReader<HbpBlankingMode>;
impl HbpBlankingModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HbpBlankingMode {
        match self.bits {
            false => HbpBlankingMode::CommandPacketTxFifo,
            true => HbpBlankingMode::HsBlankingPacketOnly,
        }
    }
    #[doc = "Packets in TX FIFO are sent during HBP blanking period of video mode or LPS is used."]
    #[inline(always)]
    pub fn is_command_packet_tx_fifo(&self) -> bool {
        *self == HbpBlankingMode::CommandPacketTxFifo
    }
    #[doc = "LONG BLANKING PACKETS only are used during HBP blanking period of video mode."]
    #[inline(always)]
    pub fn is_hs_blanking_packet_only(&self) -> bool {
        *self == HbpBlankingMode::HsBlankingPacketOnly
    }
}
#[doc = "Field `HBP_BLANKING_MODE` writer - 22:22\\]
Blanking mode"]
pub type HbpBlankingModeW<'a, REG> = crate::BitWriter<'a, REG, HbpBlankingMode>;
impl<'a, REG> HbpBlankingModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Packets in TX FIFO are sent during HBP blanking period of video mode or LPS is used."]
    #[inline(always)]
    pub fn command_packet_tx_fifo(self) -> &'a mut crate::W<REG> {
        self.variant(HbpBlankingMode::CommandPacketTxFifo)
    }
    #[doc = "LONG BLANKING PACKETS only are used during HBP blanking period of video mode."]
    #[inline(always)]
    pub fn hs_blanking_packet_only(self) -> &'a mut crate::W<REG> {
        self.variant(HbpBlankingMode::HsBlankingPacketOnly)
    }
}
#[doc = "23:23\\]
Blanking mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsaBlankingMode {
    #[doc = "0: Packets in TX FIFO are sent during HSA blanking period of video mode or LPS is used."]
    CommandPacketTxFifo = 0,
    #[doc = "1: LONG BLANKING PACKETS only are used during HSA blanking period of video mode."]
    HsBlankingPacketOnly = 1,
}
impl From<HsaBlankingMode> for bool {
    #[inline(always)]
    fn from(variant: HsaBlankingMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSA_BLANKING_MODE` reader - 23:23\\]
Blanking mode"]
pub type HsaBlankingModeR = crate::BitReader<HsaBlankingMode>;
impl HsaBlankingModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsaBlankingMode {
        match self.bits {
            false => HsaBlankingMode::CommandPacketTxFifo,
            true => HsaBlankingMode::HsBlankingPacketOnly,
        }
    }
    #[doc = "Packets in TX FIFO are sent during HSA blanking period of video mode or LPS is used."]
    #[inline(always)]
    pub fn is_command_packet_tx_fifo(&self) -> bool {
        *self == HsaBlankingMode::CommandPacketTxFifo
    }
    #[doc = "LONG BLANKING PACKETS only are used during HSA blanking period of video mode."]
    #[inline(always)]
    pub fn is_hs_blanking_packet_only(&self) -> bool {
        *self == HsaBlankingMode::HsBlankingPacketOnly
    }
}
#[doc = "Field `HSA_BLANKING_MODE` writer - 23:23\\]
Blanking mode"]
pub type HsaBlankingModeW<'a, REG> = crate::BitWriter<'a, REG, HsaBlankingMode>;
impl<'a, REG> HsaBlankingModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Packets in TX FIFO are sent during HSA blanking period of video mode or LPS is used."]
    #[inline(always)]
    pub fn command_packet_tx_fifo(self) -> &'a mut crate::W<REG> {
        self.variant(HsaBlankingMode::CommandPacketTxFifo)
    }
    #[doc = "LONG BLANKING PACKETS only are used during HSA blanking period of video mode."]
    #[inline(always)]
    pub fn hs_blanking_packet_only(self) -> &'a mut crate::W<REG> {
        self.variant(HsaBlankingMode::HsBlankingPacketOnly)
    }
}
#[doc = "24:24\\]
Determines if the Dispc_Update_Sync signal from the display controller is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DispcUpdateSync {
    #[doc = "0: Dispc_Update_Sync signal is not used."]
    Disable = 0,
    #[doc = "1: Dispc_Update_Sync signal is used."]
    Enable = 1,
}
impl From<DispcUpdateSync> for bool {
    #[inline(always)]
    fn from(variant: DispcUpdateSync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISPC_UPDATE_SYNC` reader - 24:24\\]
Determines if the Dispc_Update_Sync signal from the display controller is used."]
pub type DispcUpdateSyncR = crate::BitReader<DispcUpdateSync>;
impl DispcUpdateSyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DispcUpdateSync {
        match self.bits {
            false => DispcUpdateSync::Disable,
            true => DispcUpdateSync::Enable,
        }
    }
    #[doc = "Dispc_Update_Sync signal is not used."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DispcUpdateSync::Disable
    }
    #[doc = "Dispc_Update_Sync signal is used."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DispcUpdateSync::Enable
    }
}
#[doc = "Field `DISPC_UPDATE_SYNC` writer - 24:24\\]
Determines if the Dispc_Update_Sync signal from the display controller is used."]
pub type DispcUpdateSyncW<'a, REG> = crate::BitWriter<'a, REG, DispcUpdateSync>;
impl<'a, REG> DispcUpdateSyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dispc_Update_Sync signal is not used."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DispcUpdateSync::Disable)
    }
    #[doc = "Dispc_Update_Sync signal is used."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DispcUpdateSync::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the module. When the module is disabled the signals from the complex IO are gated (no updates of the interrupt status register). It is not possible to change the bit-fields in the register CSI2_CTRL except IF_EN when it is enabled. All the other registers can be changed except the ones that require CSI2_VC_CTRL.VC_EN to be equal to 0 to be modified."]
    #[inline(always)]
    pub fn if_en(&self) -> IfEnR {
        IfEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables the checksum check for the received payload (long packet only for all virtual channel ids)."]
    #[inline(always)]
    pub fn cs_rx_en(&self) -> CsRxEnR {
        CsRxEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables the Error Correction Code check for the received header (short and long packets for all virtual channel ids)."]
    #[inline(always)]
    pub fn ecc_rx_en(&self) -> EccRxEnR {
        EccRxEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Defines the arbitration scheme for granting the virtual channel pending ready requests in the TX FIFO"]
    #[inline(always)]
    pub fn tx_fifo_arbitration(&self) -> TxFifoArbitrationR {
        TxFifoArbitrationR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
The field indicates the clock ratio between VP.CLK and VP.PCLK. The clock VP.PCLK is generated from VP.CLK. It is divided down. The information is only used when the video port is used to provide data in command mode. In the case of video mode, it is not used."]
    #[inline(always)]
    pub fn vp_clk_ratio(&self) -> VpClkRatioR {
        VpClkRatioR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Send the reset trigger to the peripheral."]
    #[inline(always)]
    pub fn trigger_reset(&self) -> TriggerResetR {
        TriggerResetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Defines the size of the video port data bus"]
    #[inline(always)]
    pub fn vp_data_bus_width(&self) -> VpDataBusWidthR {
        VpDataBusWidthR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
VP clock polarity"]
    #[inline(always)]
    pub fn vp_clk_pol(&self) -> VpClkPolR {
        VpClkPolR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
VP data enable signal polarity"]
    #[inline(always)]
    pub fn vp_de_pol(&self) -> VpDePolR {
        VpDePolR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
VP horizontal synchronization signal polarity"]
    #[inline(always)]
    pub fn vp_hsync_pol(&self) -> VpHsyncPolR {
        VpHsyncPolR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
VP vertical synchronization signal polarity"]
    #[inline(always)]
    pub fn vp_vsync_pol(&self) -> VpVsyncPolR {
        VpVsyncPolR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Number of line buffers to be used while receiving data on the video port. The valid values are from 0 to CSI2_GNQ.VP1_NB_LINE_BUFFER."]
    #[inline(always)]
    pub fn line_buffer(&self) -> LineBufferR {
        LineBufferR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Selection of the trigger reset mode"]
    #[inline(always)]
    pub fn trigger_reset_mode(&self) -> TriggerResetModeR {
        TriggerResetModeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
VSYNC start pulse."]
    #[inline(always)]
    pub fn vp_vsync_start(&self) -> VpVsyncStartR {
        VpVsyncStartR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
VSYNC end pulse."]
    #[inline(always)]
    pub fn vp_vsync_end(&self) -> VpVsyncEndR {
        VpVsyncEndR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
HSYNC start pulse."]
    #[inline(always)]
    pub fn vp_hsync_start(&self) -> VpHsyncStartR {
        VpHsyncStartR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
HSYNC end pulse."]
    #[inline(always)]
    pub fn vp_hsync_end(&self) -> VpHsyncEndR {
        VpHsyncEndR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Enable EOT packets at the end of HS transmission."]
    #[inline(always)]
    pub fn eot_enable(&self) -> EotEnableR {
        EotEnableR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Blanking mode"]
    #[inline(always)]
    pub fn blanking_mode(&self) -> BlankingModeR {
        BlankingModeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Blanking mode"]
    #[inline(always)]
    pub fn hfp_blanking_mode(&self) -> HfpBlankingModeR {
        HfpBlankingModeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Blanking mode"]
    #[inline(always)]
    pub fn hbp_blanking_mode(&self) -> HbpBlankingModeR {
        HbpBlankingModeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Blanking mode"]
    #[inline(always)]
    pub fn hsa_blanking_mode(&self) -> HsaBlankingModeR {
        HsaBlankingModeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Determines if the Dispc_Update_Sync signal from the display controller is used."]
    #[inline(always)]
    pub fn dispc_update_sync(&self) -> DispcUpdateSyncR {
        DispcUpdateSyncR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the module. When the module is disabled the signals from the complex IO are gated (no updates of the interrupt status register). It is not possible to change the bit-fields in the register CSI2_CTRL except IF_EN when it is enabled. All the other registers can be changed except the ones that require CSI2_VC_CTRL.VC_EN to be equal to 0 to be modified."]
    #[inline(always)]
    #[must_use]
    pub fn if_en(&mut self) -> IfEnW<Csi2CtrlSpec> {
        IfEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables the checksum check for the received payload (long packet only for all virtual channel ids)."]
    #[inline(always)]
    #[must_use]
    pub fn cs_rx_en(&mut self) -> CsRxEnW<Csi2CtrlSpec> {
        CsRxEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables the Error Correction Code check for the received header (short and long packets for all virtual channel ids)."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_rx_en(&mut self) -> EccRxEnW<Csi2CtrlSpec> {
        EccRxEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Defines the arbitration scheme for granting the virtual channel pending ready requests in the TX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_arbitration(&mut self) -> TxFifoArbitrationW<Csi2CtrlSpec> {
        TxFifoArbitrationW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
The field indicates the clock ratio between VP.CLK and VP.PCLK. The clock VP.PCLK is generated from VP.CLK. It is divided down. The information is only used when the video port is used to provide data in command mode. In the case of video mode, it is not used."]
    #[inline(always)]
    #[must_use]
    pub fn vp_clk_ratio(&mut self) -> VpClkRatioW<Csi2CtrlSpec> {
        VpClkRatioW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Send the reset trigger to the peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn trigger_reset(&mut self) -> TriggerResetW<Csi2CtrlSpec> {
        TriggerResetW::new(self, 5)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Defines the size of the video port data bus"]
    #[inline(always)]
    #[must_use]
    pub fn vp_data_bus_width(&mut self) -> VpDataBusWidthW<Csi2CtrlSpec> {
        VpDataBusWidthW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
VP clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vp_clk_pol(&mut self) -> VpClkPolW<Csi2CtrlSpec> {
        VpClkPolW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
VP data enable signal polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vp_de_pol(&mut self) -> VpDePolW<Csi2CtrlSpec> {
        VpDePolW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
VP horizontal synchronization signal polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vp_hsync_pol(&mut self) -> VpHsyncPolW<Csi2CtrlSpec> {
        VpHsyncPolW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
VP vertical synchronization signal polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vp_vsync_pol(&mut self) -> VpVsyncPolW<Csi2CtrlSpec> {
        VpVsyncPolW::new(self, 11)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Number of line buffers to be used while receiving data on the video port. The valid values are from 0 to CSI2_GNQ.VP1_NB_LINE_BUFFER."]
    #[inline(always)]
    #[must_use]
    pub fn line_buffer(&mut self) -> LineBufferW<Csi2CtrlSpec> {
        LineBufferW::new(self, 12)
    }
    #[doc = "Bit 14 - 14:14\\]
Selection of the trigger reset mode"]
    #[inline(always)]
    #[must_use]
    pub fn trigger_reset_mode(&mut self) -> TriggerResetModeW<Csi2CtrlSpec> {
        TriggerResetModeW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
VSYNC start pulse."]
    #[inline(always)]
    #[must_use]
    pub fn vp_vsync_start(&mut self) -> VpVsyncStartW<Csi2CtrlSpec> {
        VpVsyncStartW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
VSYNC end pulse."]
    #[inline(always)]
    #[must_use]
    pub fn vp_vsync_end(&mut self) -> VpVsyncEndW<Csi2CtrlSpec> {
        VpVsyncEndW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
HSYNC start pulse."]
    #[inline(always)]
    #[must_use]
    pub fn vp_hsync_start(&mut self) -> VpHsyncStartW<Csi2CtrlSpec> {
        VpHsyncStartW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
HSYNC end pulse."]
    #[inline(always)]
    #[must_use]
    pub fn vp_hsync_end(&mut self) -> VpHsyncEndW<Csi2CtrlSpec> {
        VpHsyncEndW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Enable EOT packets at the end of HS transmission."]
    #[inline(always)]
    #[must_use]
    pub fn eot_enable(&mut self) -> EotEnableW<Csi2CtrlSpec> {
        EotEnableW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Blanking mode"]
    #[inline(always)]
    #[must_use]
    pub fn blanking_mode(&mut self) -> BlankingModeW<Csi2CtrlSpec> {
        BlankingModeW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Blanking mode"]
    #[inline(always)]
    #[must_use]
    pub fn hfp_blanking_mode(&mut self) -> HfpBlankingModeW<Csi2CtrlSpec> {
        HfpBlankingModeW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Blanking mode"]
    #[inline(always)]
    #[must_use]
    pub fn hbp_blanking_mode(&mut self) -> HbpBlankingModeW<Csi2CtrlSpec> {
        HbpBlankingModeW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Blanking mode"]
    #[inline(always)]
    #[must_use]
    pub fn hsa_blanking_mode(&mut self) -> HsaBlankingModeW<Csi2CtrlSpec> {
        HsaBlankingModeW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Determines if the Dispc_Update_Sync signal from the display controller is used."]
    #[inline(always)]
    #[must_use]
    pub fn dispc_update_sync(&mut self) -> DispcUpdateSyncW<Csi2CtrlSpec> {
        DispcUpdateSyncW::new(self, 24)
    }
}
#[doc = "GLOBAL CONTROL REGISTER This register controls the CSI2 Protocol Engine module. This register shall not be modified dynamically (except IF_EN bit fields).\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2CtrlSpec;
impl crate::RegisterSpec for Csi2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_ctrl::R`](R) reader structure"]
impl crate::Readable for Csi2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_ctrl::W`](W) writer structure"]
impl crate::Writable for Csi2CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_CTRL to value 0"]
impl crate::Resettable for Csi2CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
