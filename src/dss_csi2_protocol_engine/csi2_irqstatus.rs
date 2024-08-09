#[doc = "Register `CSI2_IRQSTATUS` reader"]
pub type R = crate::R<Csi2IrqstatusSpec>;
#[doc = "Register `CSI2_IRQSTATUS` writer"]
pub type W = crate::W<Csi2IrqstatusSpec>;
#[doc = "0:0\\]
Virtual channel #0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VirtualChannel0Irq {
    #[doc = "0: READS: Event is false."]
    False = 0,
    #[doc = "1: READS: Event is true (pending)."]
    True = 1,
}
impl From<VirtualChannel0Irq> for bool {
    #[inline(always)]
    fn from(variant: VirtualChannel0Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIRTUAL_CHANNEL0_IRQ` reader - 0:0\\]
Virtual channel #0"]
pub type VirtualChannel0IrqR = crate::BitReader<VirtualChannel0Irq>;
impl VirtualChannel0IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VirtualChannel0Irq {
        match self.bits {
            false => VirtualChannel0Irq::False,
            true => VirtualChannel0Irq::True,
        }
    }
    #[doc = "READS: Event is false."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == VirtualChannel0Irq::False
    }
    #[doc = "READS: Event is true (pending)."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == VirtualChannel0Irq::True
    }
}
#[doc = "Field `VIRTUAL_CHANNEL0_IRQ` writer - 0:0\\]
Virtual channel #0"]
pub type VirtualChannel0IrqW<'a, REG> = crate::BitWriter<'a, REG, VirtualChannel0Irq>;
impl<'a, REG> VirtualChannel0IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(VirtualChannel0Irq::False)
    }
    #[doc = "READS: Event is true (pending)."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(VirtualChannel0Irq::True)
    }
}
#[doc = "1:1\\]
Virtual channel #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VirtualChannel1Irq {
    #[doc = "0: READS: Event is false."]
    False = 0,
    #[doc = "1: READS: Event is true (pending)."]
    True = 1,
}
impl From<VirtualChannel1Irq> for bool {
    #[inline(always)]
    fn from(variant: VirtualChannel1Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIRTUAL_CHANNEL1_IRQ` reader - 1:1\\]
Virtual channel #1"]
pub type VirtualChannel1IrqR = crate::BitReader<VirtualChannel1Irq>;
impl VirtualChannel1IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VirtualChannel1Irq {
        match self.bits {
            false => VirtualChannel1Irq::False,
            true => VirtualChannel1Irq::True,
        }
    }
    #[doc = "READS: Event is false."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == VirtualChannel1Irq::False
    }
    #[doc = "READS: Event is true (pending)."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == VirtualChannel1Irq::True
    }
}
#[doc = "Field `VIRTUAL_CHANNEL1_IRQ` writer - 1:1\\]
Virtual channel #1"]
pub type VirtualChannel1IrqW<'a, REG> = crate::BitWriter<'a, REG, VirtualChannel1Irq>;
impl<'a, REG> VirtualChannel1IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(VirtualChannel1Irq::False)
    }
    #[doc = "READS: Event is true (pending)."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(VirtualChannel1Irq::True)
    }
}
#[doc = "2:2\\]
Virtual channel #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VirtualChannel2Irq {
    #[doc = "0: READS: Event is false."]
    False = 0,
    #[doc = "1: READS: Event is true (pending)."]
    True = 1,
}
impl From<VirtualChannel2Irq> for bool {
    #[inline(always)]
    fn from(variant: VirtualChannel2Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIRTUAL_CHANNEL2_IRQ` reader - 2:2\\]
Virtual channel #2"]
pub type VirtualChannel2IrqR = crate::BitReader<VirtualChannel2Irq>;
impl VirtualChannel2IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VirtualChannel2Irq {
        match self.bits {
            false => VirtualChannel2Irq::False,
            true => VirtualChannel2Irq::True,
        }
    }
    #[doc = "READS: Event is false."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == VirtualChannel2Irq::False
    }
    #[doc = "READS: Event is true (pending)."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == VirtualChannel2Irq::True
    }
}
#[doc = "Field `VIRTUAL_CHANNEL2_IRQ` writer - 2:2\\]
Virtual channel #2"]
pub type VirtualChannel2IrqW<'a, REG> = crate::BitWriter<'a, REG, VirtualChannel2Irq>;
impl<'a, REG> VirtualChannel2IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(VirtualChannel2Irq::False)
    }
    #[doc = "READS: Event is true (pending)."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(VirtualChannel2Irq::True)
    }
}
#[doc = "3:3\\]
Virtual channel #3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VirtualChannel3Irq {
    #[doc = "0: READS: Event is false."]
    False = 0,
    #[doc = "1: READS: Event is true (pending)."]
    True = 1,
}
impl From<VirtualChannel3Irq> for bool {
    #[inline(always)]
    fn from(variant: VirtualChannel3Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIRTUAL_CHANNEL3_IRQ` reader - 3:3\\]
Virtual channel #3"]
pub type VirtualChannel3IrqR = crate::BitReader<VirtualChannel3Irq>;
impl VirtualChannel3IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VirtualChannel3Irq {
        match self.bits {
            false => VirtualChannel3Irq::False,
            true => VirtualChannel3Irq::True,
        }
    }
    #[doc = "READS: Event is false."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == VirtualChannel3Irq::False
    }
    #[doc = "READS: Event is true (pending)."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == VirtualChannel3Irq::True
    }
}
#[doc = "Field `VIRTUAL_CHANNEL3_IRQ` writer - 3:3\\]
Virtual channel #3"]
pub type VirtualChannel3IrqW<'a, REG> = crate::BitWriter<'a, REG, VirtualChannel3Irq>;
impl<'a, REG> VirtualChannel3IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(VirtualChannel3Irq::False)
    }
    #[doc = "READS: Event is true (pending)."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(VirtualChannel3Irq::True)
    }
}
#[doc = "4:4\\]
Wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<WakeupIrq> for bool {
    #[inline(always)]
    fn from(variant: WakeupIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_IRQ` reader - 4:4\\]
Wakeup"]
pub type WakeupIrqR = crate::BitReader<WakeupIrq>;
impl WakeupIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupIrq {
        match self.bits {
            false => WakeupIrq::False,
            true => WakeupIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == WakeupIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == WakeupIrq::True
    }
}
#[doc = "Field `WAKEUP_IRQ` writer - 4:4\\]
Wakeup"]
pub type WakeupIrqW<'a, REG> = crate::BitWriter<'a, REG, WakeupIrq>;
impl<'a, REG> WakeupIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupIrq::True)
    }
}
#[doc = "5:5\\]
Video mode resynchronization indicates to the software users that the video port works but the configuration of the timings for the display controller (DISPC) and for CSI2 Protocol engine may need to be modified to avoid the resynchronization to occur.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResynchronizationIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<ResynchronizationIrq> for bool {
    #[inline(always)]
    fn from(variant: ResynchronizationIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESYNCHRONIZATION_IRQ` reader - 5:5\\]
Video mode resynchronization indicates to the software users that the video port works but the configuration of the timings for the display controller (DISPC) and for CSI2 Protocol engine may need to be modified to avoid the resynchronization to occur."]
pub type ResynchronizationIrqR = crate::BitReader<ResynchronizationIrq>;
impl ResynchronizationIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResynchronizationIrq {
        match self.bits {
            false => ResynchronizationIrq::False,
            true => ResynchronizationIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == ResynchronizationIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == ResynchronizationIrq::True
    }
}
#[doc = "Field `RESYNCHRONIZATION_IRQ` writer - 5:5\\]
Video mode resynchronization indicates to the software users that the video port works but the configuration of the timings for the display controller (DISPC) and for CSI2 Protocol engine may need to be modified to avoid the resynchronization to occur."]
pub type ResynchronizationIrqW<'a, REG> = crate::BitWriter<'a, REG, ResynchronizationIrq>;
impl<'a, REG> ResynchronizationIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(ResynchronizationIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(ResynchronizationIrq::True)
    }
}
#[doc = "7:7\\]
PLL clock event (assertion of CSI2Lock signal from the CSI2 PLL Control module)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllLockIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<PllLockIrq> for bool {
    #[inline(always)]
    fn from(variant: PllLockIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_LOCK_IRQ` reader - 7:7\\]
PLL clock event (assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
pub type PllLockIrqR = crate::BitReader<PllLockIrq>;
impl PllLockIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllLockIrq {
        match self.bits {
            false => PllLockIrq::False,
            true => PllLockIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == PllLockIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == PllLockIrq::True
    }
}
#[doc = "Field `PLL_LOCK_IRQ` writer - 7:7\\]
PLL clock event (assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
pub type PllLockIrqW<'a, REG> = crate::BitWriter<'a, REG, PllLockIrq>;
impl<'a, REG> PllLockIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(PllLockIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(PllLockIrq::True)
    }
}
#[doc = "8:8\\]
PLL un-clock event (de-assertion of CSI2Lock signal from the CSI2 PLL Control module)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllUnlockIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<PllUnlockIrq> for bool {
    #[inline(always)]
    fn from(variant: PllUnlockIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_UNLOCK_IRQ` reader - 8:8\\]
PLL un-clock event (de-assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
pub type PllUnlockIrqR = crate::BitReader<PllUnlockIrq>;
impl PllUnlockIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllUnlockIrq {
        match self.bits {
            false => PllUnlockIrq::False,
            true => PllUnlockIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == PllUnlockIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == PllUnlockIrq::True
    }
}
#[doc = "Field `PLL_UNLOCK_IRQ` writer - 8:8\\]
PLL un-clock event (de-assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
pub type PllUnlockIrqW<'a, REG> = crate::BitWriter<'a, REG, PllUnlockIrq>;
impl<'a, REG> PllUnlockIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(PllUnlockIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(PllUnlockIrq::True)
    }
}
#[doc = "9:9\\]
PLL recal event (assertion of CSI2Recal signal from the CSI2 PLL Control module)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllRecalIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<PllRecalIrq> for bool {
    #[inline(always)]
    fn from(variant: PllRecalIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_RECAL_IRQ` reader - 9:9\\]
PLL recal event (assertion of CSI2Recal signal from the CSI2 PLL Control module)"]
pub type PllRecalIrqR = crate::BitReader<PllRecalIrq>;
impl PllRecalIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllRecalIrq {
        match self.bits {
            false => PllRecalIrq::False,
            true => PllRecalIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == PllRecalIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == PllRecalIrq::True
    }
}
#[doc = "Field `PLL_RECAL_IRQ` writer - 9:9\\]
PLL recal event (assertion of CSI2Recal signal from the CSI2 PLL Control module)"]
pub type PllRecalIrqW<'a, REG> = crate::BitWriter<'a, REG, PllRecalIrq>;
impl<'a, REG> PllRecalIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(PllRecalIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(PllRecalIrq::True)
    }
}
#[doc = "10:10\\]
Error signaling from Complex IO: status of the complex IO errors received from the complex IO(events are defined in CSI2_COMPLEXIO_IRQSTATUS).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComplexioErrIrq {
    #[doc = "0: READS: Event is false."]
    False = 0,
    #[doc = "1: READS: Event is true (pending)."]
    True = 1,
}
impl From<ComplexioErrIrq> for bool {
    #[inline(always)]
    fn from(variant: ComplexioErrIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPLEXIO_ERR_IRQ` reader - 10:10\\]
Error signaling from Complex IO: status of the complex IO errors received from the complex IO(events are defined in CSI2_COMPLEXIO_IRQSTATUS)."]
pub type ComplexioErrIrqR = crate::BitReader<ComplexioErrIrq>;
impl ComplexioErrIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ComplexioErrIrq {
        match self.bits {
            false => ComplexioErrIrq::False,
            true => ComplexioErrIrq::True,
        }
    }
    #[doc = "READS: Event is false."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == ComplexioErrIrq::False
    }
    #[doc = "READS: Event is true (pending)."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == ComplexioErrIrq::True
    }
}
#[doc = "Field `COMPLEXIO_ERR_IRQ` writer - 10:10\\]
Error signaling from Complex IO: status of the complex IO errors received from the complex IO(events are defined in CSI2_COMPLEXIO_IRQSTATUS)."]
pub type ComplexioErrIrqW<'a, REG> = crate::BitWriter<'a, REG, ComplexioErrIrq>;
impl<'a, REG> ComplexioErrIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(ComplexioErrIrq::False)
    }
    #[doc = "READS: Event is true (pending)."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(ComplexioErrIrq::True)
    }
}
#[doc = "Field `RESERVED1` reader - 13:11\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 13:11\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "14:14\\]
Interrupt for High Speed Tx Time out.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsTxToIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<HsTxToIrq> for bool {
    #[inline(always)]
    fn from(variant: HsTxToIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_TX_TO_IRQ` reader - 14:14\\]
Interrupt for High Speed Tx Time out."]
pub type HsTxToIrqR = crate::BitReader<HsTxToIrq>;
impl HsTxToIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsTxToIrq {
        match self.bits {
            false => HsTxToIrq::False,
            true => HsTxToIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == HsTxToIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == HsTxToIrq::True
    }
}
#[doc = "Field `HS_TX_TO_IRQ` writer - 14:14\\]
Interrupt for High Speed Tx Time out."]
pub type HsTxToIrqW<'a, REG> = crate::BitWriter<'a, REG, HsTxToIrq>;
impl<'a, REG> HsTxToIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(HsTxToIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(HsTxToIrq::True)
    }
}
#[doc = "15:15\\]
Interrupt for Low Power Rx Time out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpRxToIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<LpRxToIrq> for bool {
    #[inline(always)]
    fn from(variant: LpRxToIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP_RX_TO_IRQ` reader - 15:15\\]
Interrupt for Low Power Rx Time out"]
pub type LpRxToIrqR = crate::BitReader<LpRxToIrq>;
impl LpRxToIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpRxToIrq {
        match self.bits {
            false => LpRxToIrq::False,
            true => LpRxToIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == LpRxToIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == LpRxToIrq::True
    }
}
#[doc = "Field `LP_RX_TO_IRQ` writer - 15:15\\]
Interrupt for Low Power Rx Time out"]
pub type LpRxToIrqW<'a, REG> = crate::BitWriter<'a, REG, LpRxToIrq>;
impl<'a, REG> LpRxToIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(LpRxToIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(LpRxToIrq::True)
    }
}
#[doc = "16:16\\]
Tearing Effect Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeTriggerIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<TeTriggerIrq> for bool {
    #[inline(always)]
    fn from(variant: TeTriggerIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_TRIGGER_IRQ` reader - 16:16\\]
Tearing Effect Trigger"]
pub type TeTriggerIrqR = crate::BitReader<TeTriggerIrq>;
impl TeTriggerIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TeTriggerIrq {
        match self.bits {
            false => TeTriggerIrq::False,
            true => TeTriggerIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == TeTriggerIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == TeTriggerIrq::True
    }
}
#[doc = "Field `TE_TRIGGER_IRQ` writer - 16:16\\]
Tearing Effect Trigger"]
pub type TeTriggerIrqW<'a, REG> = crate::BitWriter<'a, REG, TeTriggerIrq>;
impl<'a, REG> TeTriggerIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(TeTriggerIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(TeTriggerIrq::True)
    }
}
#[doc = "17:17\\]
Acknowledge Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AckTriggerIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<AckTriggerIrq> for bool {
    #[inline(always)]
    fn from(variant: AckTriggerIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACK_TRIGGER_IRQ` reader - 17:17\\]
Acknowledge Trigger"]
pub type AckTriggerIrqR = crate::BitReader<AckTriggerIrq>;
impl AckTriggerIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AckTriggerIrq {
        match self.bits {
            false => AckTriggerIrq::False,
            true => AckTriggerIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == AckTriggerIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == AckTriggerIrq::True
    }
}
#[doc = "Field `ACK_TRIGGER_IRQ` writer - 17:17\\]
Acknowledge Trigger"]
pub type AckTriggerIrqW<'a, REG> = crate::BitWriter<'a, REG, AckTriggerIrq>;
impl<'a, REG> AckTriggerIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(AckTriggerIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(AckTriggerIrq::True)
    }
}
#[doc = "18:18\\]
Synchronization with Video port is lost (Video mode only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyncLostIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<SyncLostIrq> for bool {
    #[inline(always)]
    fn from(variant: SyncLostIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC_LOST_IRQ` reader - 18:18\\]
Synchronization with Video port is lost (Video mode only)"]
pub type SyncLostIrqR = crate::BitReader<SyncLostIrq>;
impl SyncLostIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SyncLostIrq {
        match self.bits {
            false => SyncLostIrq::False,
            true => SyncLostIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == SyncLostIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == SyncLostIrq::True
    }
}
#[doc = "Field `SYNC_LOST_IRQ` writer - 18:18\\]
Synchronization with Video port is lost (Video mode only)"]
pub type SyncLostIrqW<'a, REG> = crate::BitWriter<'a, REG, SyncLostIrq>;
impl<'a, REG> SyncLostIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(SyncLostIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(SyncLostIrq::True)
    }
}
#[doc = "19:19\\]
Transition of the status signal LDOPWRGOOD from the CSI2PHY indicating a state change for the supply VDDALDOCSI2PLL from up to down or down to up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LdoPowerGoodIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<LdoPowerGoodIrq> for bool {
    #[inline(always)]
    fn from(variant: LdoPowerGoodIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDO_POWER_GOOD_IRQ` reader - 19:19\\]
Transition of the status signal LDOPWRGOOD from the CSI2PHY indicating a state change for the supply VDDALDOCSI2PLL from up to down or down to up."]
pub type LdoPowerGoodIrqR = crate::BitReader<LdoPowerGoodIrq>;
impl LdoPowerGoodIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LdoPowerGoodIrq {
        match self.bits {
            false => LdoPowerGoodIrq::False,
            true => LdoPowerGoodIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == LdoPowerGoodIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == LdoPowerGoodIrq::True
    }
}
#[doc = "Field `LDO_POWER_GOOD_IRQ` writer - 19:19\\]
Transition of the status signal LDOPWRGOOD from the CSI2PHY indicating a state change for the supply VDDALDOCSI2PLL from up to down or down to up."]
pub type LdoPowerGoodIrqW<'a, REG> = crate::BitWriter<'a, REG, LdoPowerGoodIrq>;
impl<'a, REG> LdoPowerGoodIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(LdoPowerGoodIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(LdoPowerGoodIrq::True)
    }
}
#[doc = "20:20\\]
Turn-around Time out.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaToIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<TaToIrq> for bool {
    #[inline(always)]
    fn from(variant: TaToIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TA_TO_IRQ` reader - 20:20\\]
Turn-around Time out."]
pub type TaToIrqR = crate::BitReader<TaToIrq>;
impl TaToIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TaToIrq {
        match self.bits {
            false => TaToIrq::False,
            true => TaToIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == TaToIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == TaToIrq::True
    }
}
#[doc = "Field `TA_TO_IRQ` writer - 20:20\\]
Turn-around Time out."]
pub type TaToIrqW<'a, REG> = crate::BitWriter<'a, REG, TaToIrq>;
impl<'a, REG> TaToIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(TaToIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(TaToIrq::True)
    }
}
#[doc = "21:21\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE0 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te0LineIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Te0LineIrq> for bool {
    #[inline(always)]
    fn from(variant: Te0LineIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE0_LINE_IRQ` reader - 21:21\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE0 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
pub type Te0LineIrqR = crate::BitReader<Te0LineIrq>;
impl Te0LineIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te0LineIrq {
        match self.bits {
            false => Te0LineIrq::False,
            true => Te0LineIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Te0LineIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Te0LineIrq::True
    }
}
#[doc = "Field `TE0_LINE_IRQ` writer - 21:21\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE0 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
pub type Te0LineIrqW<'a, REG> = crate::BitWriter<'a, REG, Te0LineIrq>;
impl<'a, REG> Te0LineIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Te0LineIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Te0LineIrq::True)
    }
}
#[doc = "22:22\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE1 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te1LineIrq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Te1LineIrq> for bool {
    #[inline(always)]
    fn from(variant: Te1LineIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE1_LINE_IRQ` reader - 22:22\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE1 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
pub type Te1LineIrqR = crate::BitReader<Te1LineIrq>;
impl Te1LineIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te1LineIrq {
        match self.bits {
            false => Te1LineIrq::False,
            true => Te1LineIrq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Te1LineIrq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Te1LineIrq::True
    }
}
#[doc = "Field `TE1_LINE_IRQ` writer - 22:22\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE1 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
pub type Te1LineIrqW<'a, REG> = crate::BitWriter<'a, REG, Te1LineIrq>;
impl<'a, REG> Te1LineIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Te1LineIrq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Te1LineIrq::True)
    }
}
#[doc = "Field `RESERVED2` reader - 31:23\\]
Reserved"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED2` writer - 31:23\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Virtual channel #0"]
    #[inline(always)]
    pub fn virtual_channel0_irq(&self) -> VirtualChannel0IrqR {
        VirtualChannel0IrqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Virtual channel #1"]
    #[inline(always)]
    pub fn virtual_channel1_irq(&self) -> VirtualChannel1IrqR {
        VirtualChannel1IrqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Virtual channel #2"]
    #[inline(always)]
    pub fn virtual_channel2_irq(&self) -> VirtualChannel2IrqR {
        VirtualChannel2IrqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Virtual channel #3"]
    #[inline(always)]
    pub fn virtual_channel3_irq(&self) -> VirtualChannel3IrqR {
        VirtualChannel3IrqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Wakeup"]
    #[inline(always)]
    pub fn wakeup_irq(&self) -> WakeupIrqR {
        WakeupIrqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Video mode resynchronization indicates to the software users that the video port works but the configuration of the timings for the display controller (DISPC) and for CSI2 Protocol engine may need to be modified to avoid the resynchronization to occur."]
    #[inline(always)]
    pub fn resynchronization_irq(&self) -> ResynchronizationIrqR {
        ResynchronizationIrqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
PLL clock event (assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
    #[inline(always)]
    pub fn pll_lock_irq(&self) -> PllLockIrqR {
        PllLockIrqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
PLL un-clock event (de-assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
    #[inline(always)]
    pub fn pll_unlock_irq(&self) -> PllUnlockIrqR {
        PllUnlockIrqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
PLL recal event (assertion of CSI2Recal signal from the CSI2 PLL Control module)"]
    #[inline(always)]
    pub fn pll_recal_irq(&self) -> PllRecalIrqR {
        PllRecalIrqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Error signaling from Complex IO: status of the complex IO errors received from the complex IO(events are defined in CSI2_COMPLEXIO_IRQSTATUS)."]
    #[inline(always)]
    pub fn complexio_err_irq(&self) -> ComplexioErrIrqR {
        ComplexioErrIrqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt for High Speed Tx Time out."]
    #[inline(always)]
    pub fn hs_tx_to_irq(&self) -> HsTxToIrqR {
        HsTxToIrqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt for Low Power Rx Time out"]
    #[inline(always)]
    pub fn lp_rx_to_irq(&self) -> LpRxToIrqR {
        LpRxToIrqR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Tearing Effect Trigger"]
    #[inline(always)]
    pub fn te_trigger_irq(&self) -> TeTriggerIrqR {
        TeTriggerIrqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Acknowledge Trigger"]
    #[inline(always)]
    pub fn ack_trigger_irq(&self) -> AckTriggerIrqR {
        AckTriggerIrqR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Synchronization with Video port is lost (Video mode only)"]
    #[inline(always)]
    pub fn sync_lost_irq(&self) -> SyncLostIrqR {
        SyncLostIrqR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Transition of the status signal LDOPWRGOOD from the CSI2PHY indicating a state change for the supply VDDALDOCSI2PLL from up to down or down to up."]
    #[inline(always)]
    pub fn ldo_power_good_irq(&self) -> LdoPowerGoodIrqR {
        LdoPowerGoodIrqR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Turn-around Time out."]
    #[inline(always)]
    pub fn ta_to_irq(&self) -> TaToIrqR {
        TaToIrqR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE0 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
    #[inline(always)]
    pub fn te0_line_irq(&self) -> Te0LineIrqR {
        Te0LineIrqR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE1 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
    #[inline(always)]
    pub fn te1_line_irq(&self) -> Te1LineIrqR {
        Te1LineIrqR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Virtual channel #0"]
    #[inline(always)]
    #[must_use]
    pub fn virtual_channel0_irq(&mut self) -> VirtualChannel0IrqW<Csi2IrqstatusSpec> {
        VirtualChannel0IrqW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Virtual channel #1"]
    #[inline(always)]
    #[must_use]
    pub fn virtual_channel1_irq(&mut self) -> VirtualChannel1IrqW<Csi2IrqstatusSpec> {
        VirtualChannel1IrqW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Virtual channel #2"]
    #[inline(always)]
    #[must_use]
    pub fn virtual_channel2_irq(&mut self) -> VirtualChannel2IrqW<Csi2IrqstatusSpec> {
        VirtualChannel2IrqW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Virtual channel #3"]
    #[inline(always)]
    #[must_use]
    pub fn virtual_channel3_irq(&mut self) -> VirtualChannel3IrqW<Csi2IrqstatusSpec> {
        VirtualChannel3IrqW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_irq(&mut self) -> WakeupIrqW<Csi2IrqstatusSpec> {
        WakeupIrqW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Video mode resynchronization indicates to the software users that the video port works but the configuration of the timings for the display controller (DISPC) and for CSI2 Protocol engine may need to be modified to avoid the resynchronization to occur."]
    #[inline(always)]
    #[must_use]
    pub fn resynchronization_irq(&mut self) -> ResynchronizationIrqW<Csi2IrqstatusSpec> {
        ResynchronizationIrqW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
PLL clock event (assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lock_irq(&mut self) -> PllLockIrqW<Csi2IrqstatusSpec> {
        PllLockIrqW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
PLL un-clock event (de-assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
    #[inline(always)]
    #[must_use]
    pub fn pll_unlock_irq(&mut self) -> PllUnlockIrqW<Csi2IrqstatusSpec> {
        PllUnlockIrqW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
PLL recal event (assertion of CSI2Recal signal from the CSI2 PLL Control module)"]
    #[inline(always)]
    #[must_use]
    pub fn pll_recal_irq(&mut self) -> PllRecalIrqW<Csi2IrqstatusSpec> {
        PllRecalIrqW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Error signaling from Complex IO: status of the complex IO errors received from the complex IO(events are defined in CSI2_COMPLEXIO_IRQSTATUS)."]
    #[inline(always)]
    #[must_use]
    pub fn complexio_err_irq(&mut self) -> ComplexioErrIrqW<Csi2IrqstatusSpec> {
        ComplexioErrIrqW::new(self, 10)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Csi2IrqstatusSpec> {
        Reserved1W::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt for High Speed Tx Time out."]
    #[inline(always)]
    #[must_use]
    pub fn hs_tx_to_irq(&mut self) -> HsTxToIrqW<Csi2IrqstatusSpec> {
        HsTxToIrqW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt for Low Power Rx Time out"]
    #[inline(always)]
    #[must_use]
    pub fn lp_rx_to_irq(&mut self) -> LpRxToIrqW<Csi2IrqstatusSpec> {
        LpRxToIrqW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Tearing Effect Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn te_trigger_irq(&mut self) -> TeTriggerIrqW<Csi2IrqstatusSpec> {
        TeTriggerIrqW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Acknowledge Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn ack_trigger_irq(&mut self) -> AckTriggerIrqW<Csi2IrqstatusSpec> {
        AckTriggerIrqW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Synchronization with Video port is lost (Video mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn sync_lost_irq(&mut self) -> SyncLostIrqW<Csi2IrqstatusSpec> {
        SyncLostIrqW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Transition of the status signal LDOPWRGOOD from the CSI2PHY indicating a state change for the supply VDDALDOCSI2PLL from up to down or down to up."]
    #[inline(always)]
    #[must_use]
    pub fn ldo_power_good_irq(&mut self) -> LdoPowerGoodIrqW<Csi2IrqstatusSpec> {
        LdoPowerGoodIrqW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Turn-around Time out."]
    #[inline(always)]
    #[must_use]
    pub fn ta_to_irq(&mut self) -> TaToIrqW<Csi2IrqstatusSpec> {
        TaToIrqW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE0 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn te0_line_irq(&mut self) -> Te0LineIrqW<Csi2IrqstatusSpec> {
        Te0LineIrqW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE1 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn te1_line_irq(&mut self) -> Te1LineIrqW<Csi2IrqstatusSpec> {
        Te1LineIrqW::new(self, 22)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Csi2IrqstatusSpec> {
        Reserved2W::new(self, 23)
    }
}
#[doc = "INTERRUPT STATUS REGISTER - All virtual channels + Complex IO + PLL This register associates one bit for each virtual channel in order to determine which virtual channel has generated the interrupt. The virtual channel shall be enabled for events to be generated on that virtual channel. If the virtual channel is disabled, the interrupt is not generated.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_irqstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_irqstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2IrqstatusSpec;
impl crate::RegisterSpec for Csi2IrqstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_irqstatus::R`](R) reader structure"]
impl crate::Readable for Csi2IrqstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_irqstatus::W`](W) writer structure"]
impl crate::Writable for Csi2IrqstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_IRQSTATUS to value 0"]
impl crate::Resettable for Csi2IrqstatusSpec {
    const RESET_VALUE: u32 = 0;
}
