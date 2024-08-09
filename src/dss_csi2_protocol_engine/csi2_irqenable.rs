#[doc = "Register `CSI2_IRQENABLE` reader"]
pub type R = crate::R<Csi2IrqenableSpec>;
#[doc = "Register `CSI2_IRQENABLE` writer"]
pub type W = crate::W<Csi2IrqenableSpec>;
#[doc = "4:4\\]
Wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<WakeupIrqEn> for bool {
    #[inline(always)]
    fn from(variant: WakeupIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_IRQ_EN` reader - 4:4\\]
Wakeup"]
pub type WakeupIrqEnR = crate::BitReader<WakeupIrqEn>;
impl WakeupIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupIrqEn {
        match self.bits {
            false => WakeupIrqEn::Disable,
            true => WakeupIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WakeupIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WakeupIrqEn::Enable
    }
}
#[doc = "Field `WAKEUP_IRQ_EN` writer - 4:4\\]
Wakeup"]
pub type WakeupIrqEnW<'a, REG> = crate::BitWriter<'a, REG, WakeupIrqEn>;
impl<'a, REG> WakeupIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupIrqEn::Enable)
    }
}
#[doc = "5:5\\]
Video mode resynchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResynchronizationIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<ResynchronizationIrqEn> for bool {
    #[inline(always)]
    fn from(variant: ResynchronizationIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESYNCHRONIZATION_IRQ_EN` reader - 5:5\\]
Video mode resynchronization"]
pub type ResynchronizationIrqEnR = crate::BitReader<ResynchronizationIrqEn>;
impl ResynchronizationIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResynchronizationIrqEn {
        match self.bits {
            false => ResynchronizationIrqEn::Disable,
            true => ResynchronizationIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ResynchronizationIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ResynchronizationIrqEn::Enable
    }
}
#[doc = "Field `RESYNCHRONIZATION_IRQ_EN` writer - 5:5\\]
Video mode resynchronization"]
pub type ResynchronizationIrqEnW<'a, REG> = crate::BitWriter<'a, REG, ResynchronizationIrqEn>;
impl<'a, REG> ResynchronizationIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ResynchronizationIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ResynchronizationIrqEn::Enable)
    }
}
#[doc = "Field `RESERVED1` reader - 6:6\\]
Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - 6:6\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "7:7\\]
PLL clock event (assertion of CSI2Lock signal from the CSI2 PLL Control module)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllLockIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<PllLockIrqEn> for bool {
    #[inline(always)]
    fn from(variant: PllLockIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_LOCK_IRQ_EN` reader - 7:7\\]
PLL clock event (assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
pub type PllLockIrqEnR = crate::BitReader<PllLockIrqEn>;
impl PllLockIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllLockIrqEn {
        match self.bits {
            false => PllLockIrqEn::Disable,
            true => PllLockIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PllLockIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PllLockIrqEn::Enable
    }
}
#[doc = "Field `PLL_LOCK_IRQ_EN` writer - 7:7\\]
PLL clock event (assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
pub type PllLockIrqEnW<'a, REG> = crate::BitWriter<'a, REG, PllLockIrqEn>;
impl<'a, REG> PllLockIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PllLockIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PllLockIrqEn::Enable)
    }
}
#[doc = "8:8\\]
PLL un-clock event (de-assertion of CSI2Lock signal from the CSI2 PLL Control module)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllUnlockIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<PllUnlockIrqEn> for bool {
    #[inline(always)]
    fn from(variant: PllUnlockIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_UNLOCK_IRQ_EN` reader - 8:8\\]
PLL un-clock event (de-assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
pub type PllUnlockIrqEnR = crate::BitReader<PllUnlockIrqEn>;
impl PllUnlockIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllUnlockIrqEn {
        match self.bits {
            false => PllUnlockIrqEn::Disable,
            true => PllUnlockIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PllUnlockIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PllUnlockIrqEn::Enable
    }
}
#[doc = "Field `PLL_UNLOCK_IRQ_EN` writer - 8:8\\]
PLL un-clock event (de-assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
pub type PllUnlockIrqEnW<'a, REG> = crate::BitWriter<'a, REG, PllUnlockIrqEn>;
impl<'a, REG> PllUnlockIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PllUnlockIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PllUnlockIrqEn::Enable)
    }
}
#[doc = "9:9\\]
PLL recal event (assertion of CSI2Recal signal from the CSI2 PLL Control module)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllRecalIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<PllRecalIrqEn> for bool {
    #[inline(always)]
    fn from(variant: PllRecalIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_RECAL_IRQ_EN` reader - 9:9\\]
PLL recal event (assertion of CSI2Recal signal from the CSI2 PLL Control module)"]
pub type PllRecalIrqEnR = crate::BitReader<PllRecalIrqEn>;
impl PllRecalIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllRecalIrqEn {
        match self.bits {
            false => PllRecalIrqEn::Disable,
            true => PllRecalIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PllRecalIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PllRecalIrqEn::Enable
    }
}
#[doc = "Field `PLL_RECAL_IRQ_EN` writer - 9:9\\]
PLL recal event (assertion of CSI2Recal signal from the CSI2 PLL Control module)"]
pub type PllRecalIrqEnW<'a, REG> = crate::BitWriter<'a, REG, PllRecalIrqEn>;
impl<'a, REG> PllRecalIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PllRecalIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PllRecalIrqEn::Enable)
    }
}
#[doc = "Field `RESERVED2` reader - 13:10\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 13:10\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "14:14\\]
Interrupt for High Speed Tx Time out.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsTxToIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<HsTxToIrqEn> for bool {
    #[inline(always)]
    fn from(variant: HsTxToIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_TX_TO_IRQ_EN` reader - 14:14\\]
Interrupt for High Speed Tx Time out."]
pub type HsTxToIrqEnR = crate::BitReader<HsTxToIrqEn>;
impl HsTxToIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsTxToIrqEn {
        match self.bits {
            false => HsTxToIrqEn::Disable,
            true => HsTxToIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HsTxToIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HsTxToIrqEn::Enable
    }
}
#[doc = "Field `HS_TX_TO_IRQ_EN` writer - 14:14\\]
Interrupt for High Speed Tx Time out."]
pub type HsTxToIrqEnW<'a, REG> = crate::BitWriter<'a, REG, HsTxToIrqEn>;
impl<'a, REG> HsTxToIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HsTxToIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HsTxToIrqEn::Enable)
    }
}
#[doc = "15:15\\]
Interrupt for Low Power Rx Time out.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpRxToIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<LpRxToIrqEn> for bool {
    #[inline(always)]
    fn from(variant: LpRxToIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP_RX_TO_IRQ_EN` reader - 15:15\\]
Interrupt for Low Power Rx Time out."]
pub type LpRxToIrqEnR = crate::BitReader<LpRxToIrqEn>;
impl LpRxToIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpRxToIrqEn {
        match self.bits {
            false => LpRxToIrqEn::Disable,
            true => LpRxToIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LpRxToIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LpRxToIrqEn::Enable
    }
}
#[doc = "Field `LP_RX_TO_IRQ_EN` writer - 15:15\\]
Interrupt for Low Power Rx Time out."]
pub type LpRxToIrqEnW<'a, REG> = crate::BitWriter<'a, REG, LpRxToIrqEn>;
impl<'a, REG> LpRxToIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LpRxToIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LpRxToIrqEn::Enable)
    }
}
#[doc = "16:16\\]
Tearing Effect trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeTriggerIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<TeTriggerIrqEn> for bool {
    #[inline(always)]
    fn from(variant: TeTriggerIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_TRIGGER_IRQ_EN` reader - 16:16\\]
Tearing Effect trigger"]
pub type TeTriggerIrqEnR = crate::BitReader<TeTriggerIrqEn>;
impl TeTriggerIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TeTriggerIrqEn {
        match self.bits {
            false => TeTriggerIrqEn::Disable,
            true => TeTriggerIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TeTriggerIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TeTriggerIrqEn::Enable
    }
}
#[doc = "Field `TE_TRIGGER_IRQ_EN` writer - 16:16\\]
Tearing Effect trigger"]
pub type TeTriggerIrqEnW<'a, REG> = crate::BitWriter<'a, REG, TeTriggerIrqEn>;
impl<'a, REG> TeTriggerIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TeTriggerIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TeTriggerIrqEn::Enable)
    }
}
#[doc = "17:17\\]
Acknowledge trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AckTriggerIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<AckTriggerIrqEn> for bool {
    #[inline(always)]
    fn from(variant: AckTriggerIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACK_TRIGGER_IRQ_EN` reader - 17:17\\]
Acknowledge trigger"]
pub type AckTriggerIrqEnR = crate::BitReader<AckTriggerIrqEn>;
impl AckTriggerIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AckTriggerIrqEn {
        match self.bits {
            false => AckTriggerIrqEn::Disable,
            true => AckTriggerIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AckTriggerIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AckTriggerIrqEn::Enable
    }
}
#[doc = "Field `ACK_TRIGGER_IRQ_EN` writer - 17:17\\]
Acknowledge trigger"]
pub type AckTriggerIrqEnW<'a, REG> = crate::BitWriter<'a, REG, AckTriggerIrqEn>;
impl<'a, REG> AckTriggerIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AckTriggerIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AckTriggerIrqEn::Enable)
    }
}
#[doc = "18:18\\]
Synchronization with Video port is lost (Video mode only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyncLostIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<SyncLostIrqEn> for bool {
    #[inline(always)]
    fn from(variant: SyncLostIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC_LOST_IRQ_EN` reader - 18:18\\]
Synchronization with Video port is lost (Video mode only)"]
pub type SyncLostIrqEnR = crate::BitReader<SyncLostIrqEn>;
impl SyncLostIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SyncLostIrqEn {
        match self.bits {
            false => SyncLostIrqEn::Disable,
            true => SyncLostIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SyncLostIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SyncLostIrqEn::Enable
    }
}
#[doc = "Field `SYNC_LOST_IRQ_EN` writer - 18:18\\]
Synchronization with Video port is lost (Video mode only)"]
pub type SyncLostIrqEnW<'a, REG> = crate::BitWriter<'a, REG, SyncLostIrqEn>;
impl<'a, REG> SyncLostIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SyncLostIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SyncLostIrqEn::Enable)
    }
}
#[doc = "19:19\\]
Transition of the status signal LDOPWRGOOD from the CSI2PHY indicating a state change for the supply VDDALDOCSI2PLL from up to down or down to up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LdoPowerGoodIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<LdoPowerGoodIrqEn> for bool {
    #[inline(always)]
    fn from(variant: LdoPowerGoodIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDO_POWER_GOOD_IRQ_EN` reader - 19:19\\]
Transition of the status signal LDOPWRGOOD from the CSI2PHY indicating a state change for the supply VDDALDOCSI2PLL from up to down or down to up."]
pub type LdoPowerGoodIrqEnR = crate::BitReader<LdoPowerGoodIrqEn>;
impl LdoPowerGoodIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LdoPowerGoodIrqEn {
        match self.bits {
            false => LdoPowerGoodIrqEn::Disable,
            true => LdoPowerGoodIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LdoPowerGoodIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LdoPowerGoodIrqEn::Enable
    }
}
#[doc = "Field `LDO_POWER_GOOD_IRQ_EN` writer - 19:19\\]
Transition of the status signal LDOPWRGOOD from the CSI2PHY indicating a state change for the supply VDDALDOCSI2PLL from up to down or down to up."]
pub type LdoPowerGoodIrqEnW<'a, REG> = crate::BitWriter<'a, REG, LdoPowerGoodIrqEn>;
impl<'a, REG> LdoPowerGoodIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LdoPowerGoodIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LdoPowerGoodIrqEn::Enable)
    }
}
#[doc = "20:20\\]
Turn-around Time out.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaToIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<TaToIrqEn> for bool {
    #[inline(always)]
    fn from(variant: TaToIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TA_TO_IRQ_EN` reader - 20:20\\]
Turn-around Time out."]
pub type TaToIrqEnR = crate::BitReader<TaToIrqEn>;
impl TaToIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TaToIrqEn {
        match self.bits {
            false => TaToIrqEn::Disable,
            true => TaToIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TaToIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TaToIrqEn::Enable
    }
}
#[doc = "Field `TA_TO_IRQ_EN` writer - 20:20\\]
Turn-around Time out."]
pub type TaToIrqEnW<'a, REG> = crate::BitWriter<'a, REG, TaToIrqEn>;
impl<'a, REG> TaToIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TaToIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TaToIrqEn::Enable)
    }
}
#[doc = "21:21\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE0 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te0LineIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Te0LineIrqEn> for bool {
    #[inline(always)]
    fn from(variant: Te0LineIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE0_LINE_IRQ_EN` reader - 21:21\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE0 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
pub type Te0LineIrqEnR = crate::BitReader<Te0LineIrqEn>;
impl Te0LineIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te0LineIrqEn {
        match self.bits {
            false => Te0LineIrqEn::Disable,
            true => Te0LineIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te0LineIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te0LineIrqEn::Enable
    }
}
#[doc = "Field `TE0_LINE_IRQ_EN` writer - 21:21\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE0 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
pub type Te0LineIrqEnW<'a, REG> = crate::BitWriter<'a, REG, Te0LineIrqEn>;
impl<'a, REG> Te0LineIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te0LineIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te0LineIrqEn::Enable)
    }
}
#[doc = "22:22\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE1 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te1LineIrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Te1LineIrqEn> for bool {
    #[inline(always)]
    fn from(variant: Te1LineIrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE1_LINE_IRQ_EN` reader - 22:22\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE1 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
pub type Te1LineIrqEnR = crate::BitReader<Te1LineIrqEn>;
impl Te1LineIrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te1LineIrqEn {
        match self.bits {
            false => Te1LineIrqEn::Disable,
            true => Te1LineIrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te1LineIrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te1LineIrqEn::Enable
    }
}
#[doc = "Field `TE1_LINE_IRQ_EN` writer - 22:22\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE1 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
pub type Te1LineIrqEnW<'a, REG> = crate::BitWriter<'a, REG, Te1LineIrqEn>;
impl<'a, REG> Te1LineIrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te1LineIrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te1LineIrqEn::Enable)
    }
}
#[doc = "Field `RESERVED3` reader - 31:23\\]
Reserved"]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED3` writer - 31:23\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
Wakeup"]
    #[inline(always)]
    pub fn wakeup_irq_en(&self) -> WakeupIrqEnR {
        WakeupIrqEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Video mode resynchronization"]
    #[inline(always)]
    pub fn resynchronization_irq_en(&self) -> ResynchronizationIrqEnR {
        ResynchronizationIrqEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
PLL clock event (assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
    #[inline(always)]
    pub fn pll_lock_irq_en(&self) -> PllLockIrqEnR {
        PllLockIrqEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
PLL un-clock event (de-assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
    #[inline(always)]
    pub fn pll_unlock_irq_en(&self) -> PllUnlockIrqEnR {
        PllUnlockIrqEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
PLL recal event (assertion of CSI2Recal signal from the CSI2 PLL Control module)"]
    #[inline(always)]
    pub fn pll_recal_irq_en(&self) -> PllRecalIrqEnR {
        PllRecalIrqEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - 13:10\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt for High Speed Tx Time out."]
    #[inline(always)]
    pub fn hs_tx_to_irq_en(&self) -> HsTxToIrqEnR {
        HsTxToIrqEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt for Low Power Rx Time out."]
    #[inline(always)]
    pub fn lp_rx_to_irq_en(&self) -> LpRxToIrqEnR {
        LpRxToIrqEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Tearing Effect trigger"]
    #[inline(always)]
    pub fn te_trigger_irq_en(&self) -> TeTriggerIrqEnR {
        TeTriggerIrqEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Acknowledge trigger"]
    #[inline(always)]
    pub fn ack_trigger_irq_en(&self) -> AckTriggerIrqEnR {
        AckTriggerIrqEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Synchronization with Video port is lost (Video mode only)"]
    #[inline(always)]
    pub fn sync_lost_irq_en(&self) -> SyncLostIrqEnR {
        SyncLostIrqEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Transition of the status signal LDOPWRGOOD from the CSI2PHY indicating a state change for the supply VDDALDOCSI2PLL from up to down or down to up."]
    #[inline(always)]
    pub fn ldo_power_good_irq_en(&self) -> LdoPowerGoodIrqEnR {
        LdoPowerGoodIrqEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Turn-around Time out."]
    #[inline(always)]
    pub fn ta_to_irq_en(&self) -> TaToIrqEnR {
        TaToIrqEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE0 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
    #[inline(always)]
    pub fn te0_line_irq_en(&self) -> Te0LineIrqEnR {
        Te0LineIrqEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE1 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
    #[inline(always)]
    pub fn te1_line_irq_en(&self) -> Te1LineIrqEnR {
        Te1LineIrqEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_irq_en(&mut self) -> WakeupIrqEnW<Csi2IrqenableSpec> {
        WakeupIrqEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Video mode resynchronization"]
    #[inline(always)]
    #[must_use]
    pub fn resynchronization_irq_en(&mut self) -> ResynchronizationIrqEnW<Csi2IrqenableSpec> {
        ResynchronizationIrqEnW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Csi2IrqenableSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
PLL clock event (assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lock_irq_en(&mut self) -> PllLockIrqEnW<Csi2IrqenableSpec> {
        PllLockIrqEnW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
PLL un-clock event (de-assertion of CSI2Lock signal from the CSI2 PLL Control module)"]
    #[inline(always)]
    #[must_use]
    pub fn pll_unlock_irq_en(&mut self) -> PllUnlockIrqEnW<Csi2IrqenableSpec> {
        PllUnlockIrqEnW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
PLL recal event (assertion of CSI2Recal signal from the CSI2 PLL Control module)"]
    #[inline(always)]
    #[must_use]
    pub fn pll_recal_irq_en(&mut self) -> PllRecalIrqEnW<Csi2IrqenableSpec> {
        PllRecalIrqEnW::new(self, 9)
    }
    #[doc = "Bits 10:13 - 13:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Csi2IrqenableSpec> {
        Reserved2W::new(self, 10)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt for High Speed Tx Time out."]
    #[inline(always)]
    #[must_use]
    pub fn hs_tx_to_irq_en(&mut self) -> HsTxToIrqEnW<Csi2IrqenableSpec> {
        HsTxToIrqEnW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt for Low Power Rx Time out."]
    #[inline(always)]
    #[must_use]
    pub fn lp_rx_to_irq_en(&mut self) -> LpRxToIrqEnW<Csi2IrqenableSpec> {
        LpRxToIrqEnW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Tearing Effect trigger"]
    #[inline(always)]
    #[must_use]
    pub fn te_trigger_irq_en(&mut self) -> TeTriggerIrqEnW<Csi2IrqenableSpec> {
        TeTriggerIrqEnW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Acknowledge trigger"]
    #[inline(always)]
    #[must_use]
    pub fn ack_trigger_irq_en(&mut self) -> AckTriggerIrqEnW<Csi2IrqenableSpec> {
        AckTriggerIrqEnW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Synchronization with Video port is lost (Video mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn sync_lost_irq_en(&mut self) -> SyncLostIrqEnW<Csi2IrqenableSpec> {
        SyncLostIrqEnW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Transition of the status signal LDOPWRGOOD from the CSI2PHY indicating a state change for the supply VDDALDOCSI2PLL from up to down or down to up."]
    #[inline(always)]
    #[must_use]
    pub fn ldo_power_good_irq_en(&mut self) -> LdoPowerGoodIrqEnW<Csi2IrqenableSpec> {
        LdoPowerGoodIrqEnW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Turn-around Time out."]
    #[inline(always)]
    #[must_use]
    pub fn ta_to_irq_en(&mut self) -> TaToIrqEnW<Csi2IrqenableSpec> {
        TaToIrqEnW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE0 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn te0_line_irq_en(&mut self) -> Te0LineIrqEnW<Csi2IrqenableSpec> {
        Te0LineIrqEnW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
The VSYNC and corresponding HSYNC pulses defined in CSI2_TE_HSYNC_NUMBER for the line TE1 have been received by the CSI2 protocol engine and have trigger the start of the data transfer to the peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn te1_line_irq_en(&mut self) -> Te1LineIrqEnW<Csi2IrqenableSpec> {
        Te1LineIrqEnW::new(self, 22)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Csi2IrqenableSpec> {
        Reserved3W::new(self, 23)
    }
}
#[doc = "INTERRUPT ENABLE REGISTER - This register associates one bit for each virtual channel in order to enable/disable each virtual channel individually.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_irqenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_irqenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2IrqenableSpec;
impl crate::RegisterSpec for Csi2IrqenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_irqenable::R`](R) reader structure"]
impl crate::Readable for Csi2IrqenableSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_irqenable::W`](W) writer structure"]
impl crate::Writable for Csi2IrqenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_IRQENABLE to value 0"]
impl crate::Resettable for Csi2IrqenableSpec {
    const RESET_VALUE: u32 = 0;
}
