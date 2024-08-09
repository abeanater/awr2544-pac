#[doc = "Register `CSI2_VC_IRQSTATUS_1` reader"]
pub type R = crate::R<Csi2VcIrqstatus1Spec>;
#[doc = "Register `CSI2_VC_IRQSTATUS_1` writer"]
pub type W = crate::W<Csi2VcIrqstatus1Spec>;
#[doc = "0:0\\]
Virtual channel - Check-Sum mismatch status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<CsIrq> for bool {
    #[inline(always)]
    fn from(variant: CsIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS_IRQ` reader - 0:0\\]
Virtual channel - Check-Sum mismatch status."]
pub type CsIrqR = crate::BitReader<CsIrq>;
impl CsIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CsIrq {
        match self.bits {
            false => CsIrq::False,
            true => CsIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == CsIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == CsIrq::True
    }
}
#[doc = "Field `CS_IRQ` writer - 0:0\\]
Virtual channel - Check-Sum mismatch status."]
pub type CsIrqW<'a, REG> = crate::BitWriter<'a, REG, CsIrq>;
impl<'a, REG> CsIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(CsIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(CsIrq::True)
    }
}
#[doc = "1:1\\]
Virtual channel - ECC has been used to do the correction of the only 1-bit error status (short and long packet).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EccCorrectionIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<EccCorrectionIrq> for bool {
    #[inline(always)]
    fn from(variant: EccCorrectionIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECC_CORRECTION_IRQ` reader - 1:1\\]
Virtual channel - ECC has been used to do the correction of the only 1-bit error status (short and long packet)."]
pub type EccCorrectionIrqR = crate::BitReader<EccCorrectionIrq>;
impl EccCorrectionIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EccCorrectionIrq {
        match self.bits {
            false => EccCorrectionIrq::False,
            true => EccCorrectionIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == EccCorrectionIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == EccCorrectionIrq::True
    }
}
#[doc = "Field `ECC_CORRECTION_IRQ` writer - 1:1\\]
Virtual channel - ECC has been used to do the correction of the only 1-bit error status (short and long packet)."]
pub type EccCorrectionIrqW<'a, REG> = crate::BitWriter<'a, REG, EccCorrectionIrq>;
impl<'a, REG> EccCorrectionIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(EccCorrectionIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(EccCorrectionIrq::True)
    }
}
#[doc = "2:2\\]
Indicates that a packet has been sent. It is used when BTA manual mode is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PacketSentIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<PacketSentIrq> for bool {
    #[inline(always)]
    fn from(variant: PacketSentIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PACKET_SENT_IRQ` reader - 2:2\\]
Indicates that a packet has been sent. It is used when BTA manual mode is used."]
pub type PacketSentIrqR = crate::BitReader<PacketSentIrq>;
impl PacketSentIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PacketSentIrq {
        match self.bits {
            false => PacketSentIrq::False,
            true => PacketSentIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == PacketSentIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == PacketSentIrq::True
    }
}
#[doc = "Field `PACKET_SENT_IRQ` writer - 2:2\\]
Indicates that a packet has been sent. It is used when BTA manual mode is used."]
pub type PacketSentIrqW<'a, REG> = crate::BitWriter<'a, REG, PacketSentIrq>;
impl<'a, REG> PacketSentIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(PacketSentIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(PacketSentIrq::True)
    }
}
#[doc = "3:3\\]
FIFO overflow error status. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has overflowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifoTxOvfIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<FifoTxOvfIrq> for bool {
    #[inline(always)]
    fn from(variant: FifoTxOvfIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_TX_OVF_IRQ` reader - 3:3\\]
FIFO overflow error status. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has overflowed."]
pub type FifoTxOvfIrqR = crate::BitReader<FifoTxOvfIrq>;
impl FifoTxOvfIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FifoTxOvfIrq {
        match self.bits {
            false => FifoTxOvfIrq::False,
            true => FifoTxOvfIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == FifoTxOvfIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == FifoTxOvfIrq::True
    }
}
#[doc = "Field `FIFO_TX_OVF_IRQ` writer - 3:3\\]
FIFO overflow error status. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has overflowed."]
pub type FifoTxOvfIrqW<'a, REG> = crate::BitWriter<'a, REG, FifoTxOvfIrq>;
impl<'a, REG> FifoTxOvfIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(FifoTxOvfIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(FifoTxOvfIrq::True)
    }
}
#[doc = "4:4\\]
FIFO overflow error status. The FIFO used on the slave port for buffering the data received on the CSI2 link for the virtual channel has overflowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifoRxOvfIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<FifoRxOvfIrq> for bool {
    #[inline(always)]
    fn from(variant: FifoRxOvfIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_RX_OVF_IRQ` reader - 4:4\\]
FIFO overflow error status. The FIFO used on the slave port for buffering the data received on the CSI2 link for the virtual channel has overflowed."]
pub type FifoRxOvfIrqR = crate::BitReader<FifoRxOvfIrq>;
impl FifoRxOvfIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FifoRxOvfIrq {
        match self.bits {
            false => FifoRxOvfIrq::False,
            true => FifoRxOvfIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == FifoRxOvfIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == FifoRxOvfIrq::True
    }
}
#[doc = "Field `FIFO_RX_OVF_IRQ` writer - 4:4\\]
FIFO overflow error status. The FIFO used on the slave port for buffering the data received on the CSI2 link for the virtual channel has overflowed."]
pub type FifoRxOvfIrqW<'a, REG> = crate::BitWriter<'a, REG, FifoRxOvfIrq>;
impl<'a, REG> FifoRxOvfIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(FifoRxOvfIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(FifoRxOvfIrq::True)
    }
}
#[doc = "5:5\\]
Virtual channel - BTA status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BtaIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<BtaIrq> for bool {
    #[inline(always)]
    fn from(variant: BtaIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTA_IRQ` reader - 5:5\\]
Virtual channel - BTA status."]
pub type BtaIrqR = crate::BitReader<BtaIrq>;
impl BtaIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BtaIrq {
        match self.bits {
            false => BtaIrq::False,
            true => BtaIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == BtaIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == BtaIrq::True
    }
}
#[doc = "Field `BTA_IRQ` writer - 5:5\\]
Virtual channel - BTA status."]
pub type BtaIrqW<'a, REG> = crate::BitWriter<'a, REG, BtaIrq>;
impl<'a, REG> BtaIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(BtaIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(BtaIrq::True)
    }
}
#[doc = "6:6\\]
ECC error status (short and long packets). No correction of the header because of more than 1-bit error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EccNoCorrectionIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<EccNoCorrectionIrq> for bool {
    #[inline(always)]
    fn from(variant: EccNoCorrectionIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECC_NO_CORRECTION_IRQ` reader - 6:6\\]
ECC error status (short and long packets). No correction of the header because of more than 1-bit error."]
pub type EccNoCorrectionIrqR = crate::BitReader<EccNoCorrectionIrq>;
impl EccNoCorrectionIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EccNoCorrectionIrq {
        match self.bits {
            false => EccNoCorrectionIrq::False,
            true => EccNoCorrectionIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == EccNoCorrectionIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == EccNoCorrectionIrq::True
    }
}
#[doc = "Field `ECC_NO_CORRECTION_IRQ` writer - 6:6\\]
ECC error status (short and long packets). No correction of the header because of more than 1-bit error."]
pub type EccNoCorrectionIrqW<'a, REG> = crate::BitWriter<'a, REG, EccNoCorrectionIrq>;
impl<'a, REG> EccNoCorrectionIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(EccNoCorrectionIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(EccNoCorrectionIrq::True)
    }
}
#[doc = "7:7\\]
FIFO underflow status. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has underflowed which means that the data for the current packet have not been received in time since the transfer of the packet are already started (transfer started since the packet size is bigger than space allocated in the FIFO).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifoTxUdfIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<FifoTxUdfIrq> for bool {
    #[inline(always)]
    fn from(variant: FifoTxUdfIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_TX_UDF_IRQ` reader - 7:7\\]
FIFO underflow status. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has underflowed which means that the data for the current packet have not been received in time since the transfer of the packet are already started (transfer started since the packet size is bigger than space allocated in the FIFO)."]
pub type FifoTxUdfIrqR = crate::BitReader<FifoTxUdfIrq>;
impl FifoTxUdfIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FifoTxUdfIrq {
        match self.bits {
            false => FifoTxUdfIrq::False,
            true => FifoTxUdfIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == FifoTxUdfIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == FifoTxUdfIrq::True
    }
}
#[doc = "Field `FIFO_TX_UDF_IRQ` writer - 7:7\\]
FIFO underflow status. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has underflowed which means that the data for the current packet have not been received in time since the transfer of the packet are already started (transfer started since the packet size is bigger than space allocated in the FIFO)."]
pub type FifoTxUdfIrqW<'a, REG> = crate::BitWriter<'a, REG, FifoTxUdfIrq>;
impl<'a, REG> FifoTxUdfIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(FifoTxUdfIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(FifoTxUdfIrq::True)
    }
}
#[doc = "8:8\\]
Video Port ping-pong buffer busy status. PP_BUSY has changed from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PpBusyChangeIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<PpBusyChangeIrq> for bool {
    #[inline(always)]
    fn from(variant: PpBusyChangeIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PP_BUSY_CHANGE_IRQ` reader - 8:8\\]
Video Port ping-pong buffer busy status. PP_BUSY has changed from 1 to 0."]
pub type PpBusyChangeIrqR = crate::BitReader<PpBusyChangeIrq>;
impl PpBusyChangeIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PpBusyChangeIrq {
        match self.bits {
            false => PpBusyChangeIrq::False,
            true => PpBusyChangeIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == PpBusyChangeIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == PpBusyChangeIrq::True
    }
}
#[doc = "Field `PP_BUSY_CHANGE_IRQ` writer - 8:8\\]
Video Port ping-pong buffer busy status. PP_BUSY has changed from 1 to 0."]
pub type PpBusyChangeIrqW<'a, REG> = crate::BitWriter<'a, REG, PpBusyChangeIrq>;
impl<'a, REG> PpBusyChangeIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(PpBusyChangeIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(PpBusyChangeIrq::True)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Virtual channel - Check-Sum mismatch status."]
    #[inline(always)]
    pub fn cs_irq(&self) -> CsIrqR {
        CsIrqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Virtual channel - ECC has been used to do the correction of the only 1-bit error status (short and long packet)."]
    #[inline(always)]
    pub fn ecc_correction_irq(&self) -> EccCorrectionIrqR {
        EccCorrectionIrqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates that a packet has been sent. It is used when BTA manual mode is used."]
    #[inline(always)]
    pub fn packet_sent_irq(&self) -> PacketSentIrqR {
        PacketSentIrqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
FIFO overflow error status. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has overflowed."]
    #[inline(always)]
    pub fn fifo_tx_ovf_irq(&self) -> FifoTxOvfIrqR {
        FifoTxOvfIrqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
FIFO overflow error status. The FIFO used on the slave port for buffering the data received on the CSI2 link for the virtual channel has overflowed."]
    #[inline(always)]
    pub fn fifo_rx_ovf_irq(&self) -> FifoRxOvfIrqR {
        FifoRxOvfIrqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Virtual channel - BTA status."]
    #[inline(always)]
    pub fn bta_irq(&self) -> BtaIrqR {
        BtaIrqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
ECC error status (short and long packets). No correction of the header because of more than 1-bit error."]
    #[inline(always)]
    pub fn ecc_no_correction_irq(&self) -> EccNoCorrectionIrqR {
        EccNoCorrectionIrqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
FIFO underflow status. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has underflowed which means that the data for the current packet have not been received in time since the transfer of the packet are already started (transfer started since the packet size is bigger than space allocated in the FIFO)."]
    #[inline(always)]
    pub fn fifo_tx_udf_irq(&self) -> FifoTxUdfIrqR {
        FifoTxUdfIrqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Video Port ping-pong buffer busy status. PP_BUSY has changed from 1 to 0."]
    #[inline(always)]
    pub fn pp_busy_change_irq(&self) -> PpBusyChangeIrqR {
        PpBusyChangeIrqR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Virtual channel - Check-Sum mismatch status."]
    #[inline(always)]
    #[must_use]
    pub fn cs_irq(&mut self) -> CsIrqW<Csi2VcIrqstatus1Spec> {
        CsIrqW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Virtual channel - ECC has been used to do the correction of the only 1-bit error status (short and long packet)."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_correction_irq(&mut self) -> EccCorrectionIrqW<Csi2VcIrqstatus1Spec> {
        EccCorrectionIrqW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates that a packet has been sent. It is used when BTA manual mode is used."]
    #[inline(always)]
    #[must_use]
    pub fn packet_sent_irq(&mut self) -> PacketSentIrqW<Csi2VcIrqstatus1Spec> {
        PacketSentIrqW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
FIFO overflow error status. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has overflowed."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_tx_ovf_irq(&mut self) -> FifoTxOvfIrqW<Csi2VcIrqstatus1Spec> {
        FifoTxOvfIrqW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
FIFO overflow error status. The FIFO used on the slave port for buffering the data received on the CSI2 link for the virtual channel has overflowed."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_rx_ovf_irq(&mut self) -> FifoRxOvfIrqW<Csi2VcIrqstatus1Spec> {
        FifoRxOvfIrqW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Virtual channel - BTA status."]
    #[inline(always)]
    #[must_use]
    pub fn bta_irq(&mut self) -> BtaIrqW<Csi2VcIrqstatus1Spec> {
        BtaIrqW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
ECC error status (short and long packets). No correction of the header because of more than 1-bit error."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_no_correction_irq(&mut self) -> EccNoCorrectionIrqW<Csi2VcIrqstatus1Spec> {
        EccNoCorrectionIrqW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
FIFO underflow status. The FIFO used on the slave port for buffering the data received on the OCP slave port for the virtual channel has underflowed which means that the data for the current packet have not been received in time since the transfer of the packet are already started (transfer started since the packet size is bigger than space allocated in the FIFO)."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_tx_udf_irq(&mut self) -> FifoTxUdfIrqW<Csi2VcIrqstatus1Spec> {
        FifoTxUdfIrqW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Video Port ping-pong buffer busy status. PP_BUSY has changed from 1 to 0."]
    #[inline(always)]
    #[must_use]
    pub fn pp_busy_change_irq(&mut self) -> PpBusyChangeIrqW<Csi2VcIrqstatus1Spec> {
        PpBusyChangeIrqW::new(self, 8)
    }
}
#[doc = "INTERRUPT STATUS REGISTER - Virtual channel This register regroups all the events related to the virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_irqstatus_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_irqstatus_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2VcIrqstatus1Spec;
impl crate::RegisterSpec for Csi2VcIrqstatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_vc_irqstatus_1::R`](R) reader structure"]
impl crate::Readable for Csi2VcIrqstatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_vc_irqstatus_1::W`](W) writer structure"]
impl crate::Writable for Csi2VcIrqstatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_VC_IRQSTATUS_1 to value 0"]
impl crate::Resettable for Csi2VcIrqstatus1Spec {
    const RESET_VALUE: u32 = 0;
}
