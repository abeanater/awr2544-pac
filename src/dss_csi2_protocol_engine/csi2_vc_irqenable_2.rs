#[doc = "Register `CSI2_VC_IRQENABLE_2` reader"]
pub type R = crate::R<Csi2VcIrqenable2Spec>;
#[doc = "Register `CSI2_VC_IRQENABLE_2` writer"]
pub type W = crate::W<Csi2VcIrqenable2Spec>;
#[doc = "0:0\\]
Virtual channel - Check-Sum of the payload mismatch detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<CsIrqEn> for bool {
    #[inline(always)]
    fn from(variant: CsIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS_IRQ_EN` reader - 0:0\\]
Virtual channel - Check-Sum of the payload mismatch detection"]
pub type CsIrqEnR = crate::BitReader<CsIrqEn>;
impl CsIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CsIrqEn {
        match self.bits {
            false => CsIrqEn::Disable,
            true => CsIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CsIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CsIrqEn::Enable
    }
}
#[doc = "Field `CS_IRQ_EN` writer - 0:0\\]
Virtual channel - Check-Sum of the payload mismatch detection"]
pub type CsIrqEnW<'a, REG> = crate::BitWriter<'a, REG, CsIrqEn>;
impl<'a, REG> CsIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CsIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CsIrqEn::Enable)
    }
}
#[doc = "1:1\\]
Virtual channel - ECC has been used to correct the only 1-bit error (short and long packet).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EccCorrectionIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<EccCorrectionIrqEn> for bool {
    #[inline(always)]
    fn from(variant: EccCorrectionIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECC_CORRECTION_IRQ_EN` reader - 1:1\\]
Virtual channel - ECC has been used to correct the only 1-bit error (short and long packet)."]
pub type EccCorrectionIrqEnR = crate::BitReader<EccCorrectionIrqEn>;
impl EccCorrectionIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EccCorrectionIrqEn {
        match self.bits {
            false => EccCorrectionIrqEn::Disable,
            true => EccCorrectionIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EccCorrectionIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EccCorrectionIrqEn::Enable
    }
}
#[doc = "Field `ECC_CORRECTION_IRQ_EN` writer - 1:1\\]
Virtual channel - ECC has been used to correct the only 1-bit error (short and long packet)."]
pub type EccCorrectionIrqEnW<'a, REG> = crate::BitWriter<'a, REG, EccCorrectionIrqEn>;
impl<'a, REG> EccCorrectionIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EccCorrectionIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EccCorrectionIrqEn::Enable)
    }
}
#[doc = "2:2\\]
Indicates that a packet has been sent. It is used when BTA manual mode is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PacketSentIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<PacketSentIrqEn> for bool {
    #[inline(always)]
    fn from(variant: PacketSentIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PACKET_SENT_IRQ_EN` reader - 2:2\\]
Indicates that a packet has been sent. It is used when BTA manual mode is used."]
pub type PacketSentIrqEnR = crate::BitReader<PacketSentIrqEn>;
impl PacketSentIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PacketSentIrqEn {
        match self.bits {
            false => PacketSentIrqEn::Disable,
            true => PacketSentIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PacketSentIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PacketSentIrqEn::Enable
    }
}
#[doc = "Field `PACKET_SENT_IRQ_EN` writer - 2:2\\]
Indicates that a packet has been sent. It is used when BTA manual mode is used."]
pub type PacketSentIrqEnW<'a, REG> = crate::BitWriter<'a, REG, PacketSentIrqEn>;
impl<'a, REG> PacketSentIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PacketSentIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PacketSentIrqEn::Enable)
    }
}
#[doc = "3:3\\]
FIFO overflow enable. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has overflowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifoTxOvfIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<FifoTxOvfIrqEn> for bool {
    #[inline(always)]
    fn from(variant: FifoTxOvfIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_TX_OVF_IRQ_EN` reader - 3:3\\]
FIFO overflow enable. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has overflowed."]
pub type FifoTxOvfIrqEnR = crate::BitReader<FifoTxOvfIrqEn>;
impl FifoTxOvfIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FifoTxOvfIrqEn {
        match self.bits {
            false => FifoTxOvfIrqEn::Disable,
            true => FifoTxOvfIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FifoTxOvfIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FifoTxOvfIrqEn::Enable
    }
}
#[doc = "Field `FIFO_TX_OVF_IRQ_EN` writer - 3:3\\]
FIFO overflow enable. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has overflowed."]
pub type FifoTxOvfIrqEnW<'a, REG> = crate::BitWriter<'a, REG, FifoTxOvfIrqEn>;
impl<'a, REG> FifoTxOvfIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FifoTxOvfIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FifoTxOvfIrqEn::Enable)
    }
}
#[doc = "4:4\\]
FIFO overflow enable. The FIFO used on the slave port for buffering the data received on the CSI2 link for the virtual channel has overflowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifoRxOvfIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<FifoRxOvfIrqEn> for bool {
    #[inline(always)]
    fn from(variant: FifoRxOvfIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_RX_OVF_IRQ_EN` reader - 4:4\\]
FIFO overflow enable. The FIFO used on the slave port for buffering the data received on the CSI2 link for the virtual channel has overflowed."]
pub type FifoRxOvfIrqEnR = crate::BitReader<FifoRxOvfIrqEn>;
impl FifoRxOvfIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FifoRxOvfIrqEn {
        match self.bits {
            false => FifoRxOvfIrqEn::Disable,
            true => FifoRxOvfIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FifoRxOvfIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FifoRxOvfIrqEn::Enable
    }
}
#[doc = "Field `FIFO_RX_OVF_IRQ_EN` writer - 4:4\\]
FIFO overflow enable. The FIFO used on the slave port for buffering the data received on the CSI2 link for the virtual channel has overflowed."]
pub type FifoRxOvfIrqEnW<'a, REG> = crate::BitWriter<'a, REG, FifoRxOvfIrqEn>;
impl<'a, REG> FifoRxOvfIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FifoRxOvfIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FifoRxOvfIrqEn::Enable)
    }
}
#[doc = "5:5\\]
Virtual channel -Bus turn around reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BtaIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<BtaIrqEn> for bool {
    #[inline(always)]
    fn from(variant: BtaIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTA_IRQ_EN` reader - 5:5\\]
Virtual channel -Bus turn around reception"]
pub type BtaIrqEnR = crate::BitReader<BtaIrqEn>;
impl BtaIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BtaIrqEn {
        match self.bits {
            false => BtaIrqEn::Disable,
            true => BtaIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BtaIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BtaIrqEn::Enable
    }
}
#[doc = "Field `BTA_IRQ_EN` writer - 5:5\\]
Virtual channel -Bus turn around reception"]
pub type BtaIrqEnW<'a, REG> = crate::BitWriter<'a, REG, BtaIrqEn>;
impl<'a, REG> BtaIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BtaIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BtaIrqEn::Enable)
    }
}
#[doc = "6:6\\]
ECC error (short and long packets). No correction of the header because of more than 1-bit error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EccNoCorrectionIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<EccNoCorrectionIrqEn> for bool {
    #[inline(always)]
    fn from(variant: EccNoCorrectionIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECC_NO_CORRECTION_IRQ_EN` reader - 6:6\\]
ECC error (short and long packets). No correction of the header because of more than 1-bit error."]
pub type EccNoCorrectionIrqEnR = crate::BitReader<EccNoCorrectionIrqEn>;
impl EccNoCorrectionIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EccNoCorrectionIrqEn {
        match self.bits {
            false => EccNoCorrectionIrqEn::Disable,
            true => EccNoCorrectionIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EccNoCorrectionIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EccNoCorrectionIrqEn::Enable
    }
}
#[doc = "Field `ECC_NO_CORRECTION_IRQ_EN` writer - 6:6\\]
ECC error (short and long packets). No correction of the header because of more than 1-bit error."]
pub type EccNoCorrectionIrqEnW<'a, REG> = crate::BitWriter<'a, REG, EccNoCorrectionIrqEn>;
impl<'a, REG> EccNoCorrectionIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EccNoCorrectionIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EccNoCorrectionIrqEn::Enable)
    }
}
#[doc = "7:7\\]
FIFO underflow enable. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has underflowed which means that the data for the current packet have not been received in time since the transfer of the packet are already started (transfer started since the packet size is bigger than space allocated in the FIFO).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifoTxUdfIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<FifoTxUdfIrqEn> for bool {
    #[inline(always)]
    fn from(variant: FifoTxUdfIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_TX_UDF_IRQ_EN` reader - 7:7\\]
FIFO underflow enable. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has underflowed which means that the data for the current packet have not been received in time since the transfer of the packet are already started (transfer started since the packet size is bigger than space allocated in the FIFO)."]
pub type FifoTxUdfIrqEnR = crate::BitReader<FifoTxUdfIrqEn>;
impl FifoTxUdfIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FifoTxUdfIrqEn {
        match self.bits {
            false => FifoTxUdfIrqEn::Disable,
            true => FifoTxUdfIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FifoTxUdfIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FifoTxUdfIrqEn::Enable
    }
}
#[doc = "Field `FIFO_TX_UDF_IRQ_EN` writer - 7:7\\]
FIFO underflow enable. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has underflowed which means that the data for the current packet have not been received in time since the transfer of the packet are already started (transfer started since the packet size is bigger than space allocated in the FIFO)."]
pub type FifoTxUdfIrqEnW<'a, REG> = crate::BitWriter<'a, REG, FifoTxUdfIrqEn>;
impl<'a, REG> FifoTxUdfIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FifoTxUdfIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FifoTxUdfIrqEn::Enable)
    }
}
#[doc = "8:8\\]
Video Port ping-pong buffer busy status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PpBusyChangeIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<PpBusyChangeIrqEn> for bool {
    #[inline(always)]
    fn from(variant: PpBusyChangeIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PP_BUSY_CHANGE_IRQ_EN` reader - 8:8\\]
Video Port ping-pong buffer busy status."]
pub type PpBusyChangeIrqEnR = crate::BitReader<PpBusyChangeIrqEn>;
impl PpBusyChangeIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PpBusyChangeIrqEn {
        match self.bits {
            false => PpBusyChangeIrqEn::Disable,
            true => PpBusyChangeIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PpBusyChangeIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PpBusyChangeIrqEn::Enable
    }
}
#[doc = "Field `PP_BUSY_CHANGE_IRQ_EN` writer - 8:8\\]
Video Port ping-pong buffer busy status."]
pub type PpBusyChangeIrqEnW<'a, REG> = crate::BitWriter<'a, REG, PpBusyChangeIrqEn>;
impl<'a, REG> PpBusyChangeIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PpBusyChangeIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PpBusyChangeIrqEn::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Virtual channel - Check-Sum of the payload mismatch detection"]
    #[inline(always)]
    pub fn cs_irq_en(&self) -> CsIrqEnR {
        CsIrqEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Virtual channel - ECC has been used to correct the only 1-bit error (short and long packet)."]
    #[inline(always)]
    pub fn ecc_correction_irq_en(&self) -> EccCorrectionIrqEnR {
        EccCorrectionIrqEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates that a packet has been sent. It is used when BTA manual mode is used."]
    #[inline(always)]
    pub fn packet_sent_irq_en(&self) -> PacketSentIrqEnR {
        PacketSentIrqEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
FIFO overflow enable. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has overflowed."]
    #[inline(always)]
    pub fn fifo_tx_ovf_irq_en(&self) -> FifoTxOvfIrqEnR {
        FifoTxOvfIrqEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
FIFO overflow enable. The FIFO used on the slave port for buffering the data received on the CSI2 link for the virtual channel has overflowed."]
    #[inline(always)]
    pub fn fifo_rx_ovf_irq_en(&self) -> FifoRxOvfIrqEnR {
        FifoRxOvfIrqEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Virtual channel -Bus turn around reception"]
    #[inline(always)]
    pub fn bta_irq_en(&self) -> BtaIrqEnR {
        BtaIrqEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
ECC error (short and long packets). No correction of the header because of more than 1-bit error."]
    #[inline(always)]
    pub fn ecc_no_correction_irq_en(&self) -> EccNoCorrectionIrqEnR {
        EccNoCorrectionIrqEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
FIFO underflow enable. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has underflowed which means that the data for the current packet have not been received in time since the transfer of the packet are already started (transfer started since the packet size is bigger than space allocated in the FIFO)."]
    #[inline(always)]
    pub fn fifo_tx_udf_irq_en(&self) -> FifoTxUdfIrqEnR {
        FifoTxUdfIrqEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Video Port ping-pong buffer busy status."]
    #[inline(always)]
    pub fn pp_busy_change_irq_en(&self) -> PpBusyChangeIrqEnR {
        PpBusyChangeIrqEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Virtual channel - Check-Sum of the payload mismatch detection"]
    #[inline(always)]
    #[must_use]
    pub fn cs_irq_en(&mut self) -> CsIrqEnW<Csi2VcIrqenable2Spec> {
        CsIrqEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Virtual channel - ECC has been used to correct the only 1-bit error (short and long packet)."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_correction_irq_en(&mut self) -> EccCorrectionIrqEnW<Csi2VcIrqenable2Spec> {
        EccCorrectionIrqEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates that a packet has been sent. It is used when BTA manual mode is used."]
    #[inline(always)]
    #[must_use]
    pub fn packet_sent_irq_en(&mut self) -> PacketSentIrqEnW<Csi2VcIrqenable2Spec> {
        PacketSentIrqEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
FIFO overflow enable. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has overflowed."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_tx_ovf_irq_en(&mut self) -> FifoTxOvfIrqEnW<Csi2VcIrqenable2Spec> {
        FifoTxOvfIrqEnW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
FIFO overflow enable. The FIFO used on the slave port for buffering the data received on the CSI2 link for the virtual channel has overflowed."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_rx_ovf_irq_en(&mut self) -> FifoRxOvfIrqEnW<Csi2VcIrqenable2Spec> {
        FifoRxOvfIrqEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Virtual channel -Bus turn around reception"]
    #[inline(always)]
    #[must_use]
    pub fn bta_irq_en(&mut self) -> BtaIrqEnW<Csi2VcIrqenable2Spec> {
        BtaIrqEnW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
ECC error (short and long packets). No correction of the header because of more than 1-bit error."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_no_correction_irq_en(&mut self) -> EccNoCorrectionIrqEnW<Csi2VcIrqenable2Spec> {
        EccNoCorrectionIrqEnW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
FIFO underflow enable. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has underflowed which means that the data for the current packet have not been received in time since the transfer of the packet are already started (transfer started since the packet size is bigger than space allocated in the FIFO)."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_tx_udf_irq_en(&mut self) -> FifoTxUdfIrqEnW<Csi2VcIrqenable2Spec> {
        FifoTxUdfIrqEnW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Video Port ping-pong buffer busy status."]
    #[inline(always)]
    #[must_use]
    pub fn pp_busy_change_irq_en(&mut self) -> PpBusyChangeIrqEnW<Csi2VcIrqenable2Spec> {
        PpBusyChangeIrqEnW::new(self, 8)
    }
}
#[doc = "INTERRUPT ENABLE REGISTER - Virtual channel This register regroups all the events related to virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_irqenable_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_irqenable_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2VcIrqenable2Spec;
impl crate::RegisterSpec for Csi2VcIrqenable2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_vc_irqenable_2::R`](R) reader structure"]
impl crate::Readable for Csi2VcIrqenable2Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_vc_irqenable_2::W`](W) writer structure"]
impl crate::Writable for Csi2VcIrqenable2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_VC_IRQENABLE_2 to value 0"]
impl crate::Resettable for Csi2VcIrqenable2Spec {
    const RESET_VALUE: u32 = 0;
}
