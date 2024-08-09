#[doc = "Register `CSI2_COMPLEXIO_IRQSTATUS` reader"]
pub type R = crate::R<Csi2ComplexioIrqstatusSpec>;
#[doc = "Register `CSI2_COMPLEXIO_IRQSTATUS` writer"]
pub type W = crate::W<Csi2ComplexioIrqstatusSpec>;
#[doc = "0:0\\]
Low power Data transmission synchronization error for lane #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errsyncesc1Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errsyncesc1Irq> for bool {
    #[inline(always)]
    fn from(variant: Errsyncesc1Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSYNCESC1_IRQ` reader - 0:0\\]
Low power Data transmission synchronization error for lane #1"]
pub type Errsyncesc1IrqR = crate::BitReader<Errsyncesc1Irq>;
impl Errsyncesc1IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errsyncesc1Irq {
        match self.bits {
            false => Errsyncesc1Irq::False,
            true => Errsyncesc1Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errsyncesc1Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errsyncesc1Irq::True
    }
}
#[doc = "Field `ERRSYNCESC1_IRQ` writer - 0:0\\]
Low power Data transmission synchronization error for lane #1"]
pub type Errsyncesc1IrqW<'a, REG> = crate::BitWriter<'a, REG, Errsyncesc1Irq>;
impl<'a, REG> Errsyncesc1IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncesc1Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncesc1Irq::True)
    }
}
#[doc = "1:1\\]
Low power Data transmission synchronization error for lane #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errsyncesc2Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errsyncesc2Irq> for bool {
    #[inline(always)]
    fn from(variant: Errsyncesc2Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSYNCESC2_IRQ` reader - 1:1\\]
Low power Data transmission synchronization error for lane #2"]
pub type Errsyncesc2IrqR = crate::BitReader<Errsyncesc2Irq>;
impl Errsyncesc2IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errsyncesc2Irq {
        match self.bits {
            false => Errsyncesc2Irq::False,
            true => Errsyncesc2Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errsyncesc2Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errsyncesc2Irq::True
    }
}
#[doc = "Field `ERRSYNCESC2_IRQ` writer - 1:1\\]
Low power Data transmission synchronization error for lane #2"]
pub type Errsyncesc2IrqW<'a, REG> = crate::BitWriter<'a, REG, Errsyncesc2Irq>;
impl<'a, REG> Errsyncesc2IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncesc2Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncesc2Irq::True)
    }
}
#[doc = "2:2\\]
Low power Data transmission synchronization error for lane #3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errsyncesc3Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errsyncesc3Irq> for bool {
    #[inline(always)]
    fn from(variant: Errsyncesc3Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSYNCESC3_IRQ` reader - 2:2\\]
Low power Data transmission synchronization error for lane #3"]
pub type Errsyncesc3IrqR = crate::BitReader<Errsyncesc3Irq>;
impl Errsyncesc3IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errsyncesc3Irq {
        match self.bits {
            false => Errsyncesc3Irq::False,
            true => Errsyncesc3Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errsyncesc3Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errsyncesc3Irq::True
    }
}
#[doc = "Field `ERRSYNCESC3_IRQ` writer - 2:2\\]
Low power Data transmission synchronization error for lane #3"]
pub type Errsyncesc3IrqW<'a, REG> = crate::BitWriter<'a, REG, Errsyncesc3Irq>;
impl<'a, REG> Errsyncesc3IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncesc3Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncesc3Irq::True)
    }
}
#[doc = "Field `RESERVED1` reader - 3:3\\]
Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - 3:3\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "3:3\\]
Low power Data transmission synchronization error for lane #4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errsyncesc4Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errsyncesc4Irq> for bool {
    #[inline(always)]
    fn from(variant: Errsyncesc4Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSYNCESC4_IRQ` reader - 3:3\\]
Low power Data transmission synchronization error for lane #4"]
pub type Errsyncesc4IrqR = crate::BitReader<Errsyncesc4Irq>;
impl Errsyncesc4IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errsyncesc4Irq {
        match self.bits {
            false => Errsyncesc4Irq::False,
            true => Errsyncesc4Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errsyncesc4Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errsyncesc4Irq::True
    }
}
#[doc = "Field `ERRSYNCESC4_IRQ` writer - 3:3\\]
Low power Data transmission synchronization error for lane #4"]
pub type Errsyncesc4IrqW<'a, REG> = crate::BitWriter<'a, REG, Errsyncesc4Irq>;
impl<'a, REG> Errsyncesc4IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncesc4Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncesc4Irq::True)
    }
}
#[doc = "Field `RESERVED2` reader - 4:4\\]
Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `RESERVED2` writer - 4:4\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "4:4\\]
Low power Data transmission synchronization error for lane #5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errsyncesc5Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errsyncesc5Irq> for bool {
    #[inline(always)]
    fn from(variant: Errsyncesc5Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSYNCESC5_IRQ` reader - 4:4\\]
Low power Data transmission synchronization error for lane #5"]
pub type Errsyncesc5IrqR = crate::BitReader<Errsyncesc5Irq>;
impl Errsyncesc5IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errsyncesc5Irq {
        match self.bits {
            false => Errsyncesc5Irq::False,
            true => Errsyncesc5Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errsyncesc5Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errsyncesc5Irq::True
    }
}
#[doc = "Field `ERRSYNCESC5_IRQ` writer - 4:4\\]
Low power Data transmission synchronization error for lane #5"]
pub type Errsyncesc5IrqW<'a, REG> = crate::BitWriter<'a, REG, Errsyncesc5Irq>;
impl<'a, REG> Errsyncesc5IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncesc5Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncesc5Irq::True)
    }
}
#[doc = "5:5\\]
Escape entry error for lane #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erresc1Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Erresc1Irq> for bool {
    #[inline(always)]
    fn from(variant: Erresc1Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRESC1_IRQ` reader - 5:5\\]
Escape entry error for lane #1"]
pub type Erresc1IrqR = crate::BitReader<Erresc1Irq>;
impl Erresc1IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erresc1Irq {
        match self.bits {
            false => Erresc1Irq::False,
            true => Erresc1Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Erresc1Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Erresc1Irq::True
    }
}
#[doc = "Field `ERRESC1_IRQ` writer - 5:5\\]
Escape entry error for lane #1"]
pub type Erresc1IrqW<'a, REG> = crate::BitWriter<'a, REG, Erresc1Irq>;
impl<'a, REG> Erresc1IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc1Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc1Irq::True)
    }
}
#[doc = "6:6\\]
Escape entry error for lane #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erresc2Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Erresc2Irq> for bool {
    #[inline(always)]
    fn from(variant: Erresc2Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRESC2_IRQ` reader - 6:6\\]
Escape entry error for lane #2"]
pub type Erresc2IrqR = crate::BitReader<Erresc2Irq>;
impl Erresc2IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erresc2Irq {
        match self.bits {
            false => Erresc2Irq::False,
            true => Erresc2Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Erresc2Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Erresc2Irq::True
    }
}
#[doc = "Field `ERRESC2_IRQ` writer - 6:6\\]
Escape entry error for lane #2"]
pub type Erresc2IrqW<'a, REG> = crate::BitWriter<'a, REG, Erresc2Irq>;
impl<'a, REG> Erresc2IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc2Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc2Irq::True)
    }
}
#[doc = "Field `RESERVED3` reader - 7:7\\]
Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RESERVED3` writer - 7:7\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "7:7\\]
Escape entry error for lane #3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erresc3Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Erresc3Irq> for bool {
    #[inline(always)]
    fn from(variant: Erresc3Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRESC3_IRQ` reader - 7:7\\]
Escape entry error for lane #3"]
pub type Erresc3IrqR = crate::BitReader<Erresc3Irq>;
impl Erresc3IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erresc3Irq {
        match self.bits {
            false => Erresc3Irq::False,
            true => Erresc3Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Erresc3Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Erresc3Irq::True
    }
}
#[doc = "Field `ERRESC3_IRQ` writer - 7:7\\]
Escape entry error for lane #3"]
pub type Erresc3IrqW<'a, REG> = crate::BitWriter<'a, REG, Erresc3Irq>;
impl<'a, REG> Erresc3IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc3Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc3Irq::True)
    }
}
#[doc = "Field `RESERVED4` reader - 8:8\\]
Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `RESERVED4` writer - 8:8\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "8:8\\]
Escape entry error for lane #4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erresc4Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Erresc4Irq> for bool {
    #[inline(always)]
    fn from(variant: Erresc4Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRESC4_IRQ` reader - 8:8\\]
Escape entry error for lane #4"]
pub type Erresc4IrqR = crate::BitReader<Erresc4Irq>;
impl Erresc4IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erresc4Irq {
        match self.bits {
            false => Erresc4Irq::False,
            true => Erresc4Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Erresc4Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Erresc4Irq::True
    }
}
#[doc = "Field `ERRESC4_IRQ` writer - 8:8\\]
Escape entry error for lane #4"]
pub type Erresc4IrqW<'a, REG> = crate::BitWriter<'a, REG, Erresc4Irq>;
impl<'a, REG> Erresc4IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc4Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc4Irq::True)
    }
}
#[doc = "Field `RESERVED5` reader - 9:9\\]
Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `RESERVED5` writer - 9:9\\]
Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "9:9\\]
Escape entry error for lane #5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erresc5Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Erresc5Irq> for bool {
    #[inline(always)]
    fn from(variant: Erresc5Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRESC5_IRQ` reader - 9:9\\]
Escape entry error for lane #5"]
pub type Erresc5IrqR = crate::BitReader<Erresc5Irq>;
impl Erresc5IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erresc5Irq {
        match self.bits {
            false => Erresc5Irq::False,
            true => Erresc5Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Erresc5Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Erresc5Irq::True
    }
}
#[doc = "Field `ERRESC5_IRQ` writer - 9:9\\]
Escape entry error for lane #5"]
pub type Erresc5IrqW<'a, REG> = crate::BitWriter<'a, REG, Erresc5Irq>;
impl<'a, REG> Erresc5IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc5Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc5Irq::True)
    }
}
#[doc = "10:10\\]
Control error for lane #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontrol1Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errcontrol1Irq> for bool {
    #[inline(always)]
    fn from(variant: Errcontrol1Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTROL1_IRQ` reader - 10:10\\]
Control error for lane #1"]
pub type Errcontrol1IrqR = crate::BitReader<Errcontrol1Irq>;
impl Errcontrol1IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontrol1Irq {
        match self.bits {
            false => Errcontrol1Irq::False,
            true => Errcontrol1Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errcontrol1Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errcontrol1Irq::True
    }
}
#[doc = "Field `ERRCONTROL1_IRQ` writer - 10:10\\]
Control error for lane #1"]
pub type Errcontrol1IrqW<'a, REG> = crate::BitWriter<'a, REG, Errcontrol1Irq>;
impl<'a, REG> Errcontrol1IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol1Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol1Irq::True)
    }
}
#[doc = "11:11\\]
Control error for lane #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontrol2Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errcontrol2Irq> for bool {
    #[inline(always)]
    fn from(variant: Errcontrol2Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTROL2_IRQ` reader - 11:11\\]
Control error for lane #2"]
pub type Errcontrol2IrqR = crate::BitReader<Errcontrol2Irq>;
impl Errcontrol2IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontrol2Irq {
        match self.bits {
            false => Errcontrol2Irq::False,
            true => Errcontrol2Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errcontrol2Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errcontrol2Irq::True
    }
}
#[doc = "Field `ERRCONTROL2_IRQ` writer - 11:11\\]
Control error for lane #2"]
pub type Errcontrol2IrqW<'a, REG> = crate::BitWriter<'a, REG, Errcontrol2Irq>;
impl<'a, REG> Errcontrol2IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol2Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol2Irq::True)
    }
}
#[doc = "Field `RESERVED6` reader - 12:12\\]
Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `RESERVED6` writer - 12:12\\]
Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "12:12\\]
Control error for lane #3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontrol3Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errcontrol3Irq> for bool {
    #[inline(always)]
    fn from(variant: Errcontrol3Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTROL3_IRQ` reader - 12:12\\]
Control error for lane #3"]
pub type Errcontrol3IrqR = crate::BitReader<Errcontrol3Irq>;
impl Errcontrol3IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontrol3Irq {
        match self.bits {
            false => Errcontrol3Irq::False,
            true => Errcontrol3Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errcontrol3Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errcontrol3Irq::True
    }
}
#[doc = "Field `ERRCONTROL3_IRQ` writer - 12:12\\]
Control error for lane #3"]
pub type Errcontrol3IrqW<'a, REG> = crate::BitWriter<'a, REG, Errcontrol3Irq>;
impl<'a, REG> Errcontrol3IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol3Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol3Irq::True)
    }
}
#[doc = "Field `RESERVED7` reader - 13:13\\]
Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESERVED7` writer - 13:13\\]
Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "13:13\\]
Control error for lane #4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontrol4Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errcontrol4Irq> for bool {
    #[inline(always)]
    fn from(variant: Errcontrol4Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTROL4_IRQ` reader - 13:13\\]
Control error for lane #4"]
pub type Errcontrol4IrqR = crate::BitReader<Errcontrol4Irq>;
impl Errcontrol4IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontrol4Irq {
        match self.bits {
            false => Errcontrol4Irq::False,
            true => Errcontrol4Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errcontrol4Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errcontrol4Irq::True
    }
}
#[doc = "Field `ERRCONTROL4_IRQ` writer - 13:13\\]
Control error for lane #4"]
pub type Errcontrol4IrqW<'a, REG> = crate::BitWriter<'a, REG, Errcontrol4Irq>;
impl<'a, REG> Errcontrol4IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol4Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol4Irq::True)
    }
}
#[doc = "Field `RESERVED8` reader - 14:14\\]
Reserved"]
pub type Reserved8R = crate::BitReader;
#[doc = "Field `RESERVED8` writer - 14:14\\]
Reserved"]
pub type Reserved8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "14:14\\]
Control error for lane #5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontrol5Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errcontrol5Irq> for bool {
    #[inline(always)]
    fn from(variant: Errcontrol5Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTROL5_IRQ` reader - 14:14\\]
Control error for lane #5"]
pub type Errcontrol5IrqR = crate::BitReader<Errcontrol5Irq>;
impl Errcontrol5IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontrol5Irq {
        match self.bits {
            false => Errcontrol5Irq::False,
            true => Errcontrol5Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errcontrol5Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errcontrol5Irq::True
    }
}
#[doc = "Field `ERRCONTROL5_IRQ` writer - 14:14\\]
Control error for lane #5"]
pub type Errcontrol5IrqW<'a, REG> = crate::BitWriter<'a, REG, Errcontrol5Irq>;
impl<'a, REG> Errcontrol5IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol5Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol5Irq::True)
    }
}
#[doc = "15:15\\]
Lane #1 in Ultra Low Power State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stateulps1Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Stateulps1Irq> for bool {
    #[inline(always)]
    fn from(variant: Stateulps1Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATEULPS1_IRQ` reader - 15:15\\]
Lane #1 in Ultra Low Power State"]
pub type Stateulps1IrqR = crate::BitReader<Stateulps1Irq>;
impl Stateulps1IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stateulps1Irq {
        match self.bits {
            false => Stateulps1Irq::False,
            true => Stateulps1Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Stateulps1Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Stateulps1Irq::True
    }
}
#[doc = "Field `STATEULPS1_IRQ` writer - 15:15\\]
Lane #1 in Ultra Low Power State"]
pub type Stateulps1IrqW<'a, REG> = crate::BitWriter<'a, REG, Stateulps1Irq>;
impl<'a, REG> Stateulps1IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps1Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps1Irq::True)
    }
}
#[doc = "16:16\\]
Lane #2 in Ultra Low Power State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stateulps2Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Stateulps2Irq> for bool {
    #[inline(always)]
    fn from(variant: Stateulps2Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATEULPS2_IRQ` reader - 16:16\\]
Lane #2 in Ultra Low Power State"]
pub type Stateulps2IrqR = crate::BitReader<Stateulps2Irq>;
impl Stateulps2IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stateulps2Irq {
        match self.bits {
            false => Stateulps2Irq::False,
            true => Stateulps2Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Stateulps2Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Stateulps2Irq::True
    }
}
#[doc = "Field `STATEULPS2_IRQ` writer - 16:16\\]
Lane #2 in Ultra Low Power State"]
pub type Stateulps2IrqW<'a, REG> = crate::BitWriter<'a, REG, Stateulps2Irq>;
impl<'a, REG> Stateulps2IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps2Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps2Irq::True)
    }
}
#[doc = "Field `RESERVED9` reader - 17:17\\]
Reserved"]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `RESERVED9` writer - 17:17\\]
Reserved"]
pub type Reserved9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "17:17\\]
Lane #3 in Ultra Low Power State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stateulps3Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Stateulps3Irq> for bool {
    #[inline(always)]
    fn from(variant: Stateulps3Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATEULPS3_IRQ` reader - 17:17\\]
Lane #3 in Ultra Low Power State"]
pub type Stateulps3IrqR = crate::BitReader<Stateulps3Irq>;
impl Stateulps3IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stateulps3Irq {
        match self.bits {
            false => Stateulps3Irq::False,
            true => Stateulps3Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Stateulps3Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Stateulps3Irq::True
    }
}
#[doc = "Field `STATEULPS3_IRQ` writer - 17:17\\]
Lane #3 in Ultra Low Power State"]
pub type Stateulps3IrqW<'a, REG> = crate::BitWriter<'a, REG, Stateulps3Irq>;
impl<'a, REG> Stateulps3IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps3Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps3Irq::True)
    }
}
#[doc = "Field `RESERVED10` reader - 18:18\\]
Reserved"]
pub type Reserved10R = crate::BitReader;
#[doc = "Field `RESERVED10` writer - 18:18\\]
Reserved"]
pub type Reserved10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "18:18\\]
Lane #4 in Ultra Low Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stateulps4Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Stateulps4Irq> for bool {
    #[inline(always)]
    fn from(variant: Stateulps4Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATEULPS4_IRQ` reader - 18:18\\]
Lane #4 in Ultra Low Power Mode"]
pub type Stateulps4IrqR = crate::BitReader<Stateulps4Irq>;
impl Stateulps4IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stateulps4Irq {
        match self.bits {
            false => Stateulps4Irq::False,
            true => Stateulps4Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Stateulps4Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Stateulps4Irq::True
    }
}
#[doc = "Field `STATEULPS4_IRQ` writer - 18:18\\]
Lane #4 in Ultra Low Power Mode"]
pub type Stateulps4IrqW<'a, REG> = crate::BitWriter<'a, REG, Stateulps4Irq>;
impl<'a, REG> Stateulps4IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps4Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps4Irq::True)
    }
}
#[doc = "Field `RESERVED11` reader - 19:19\\]
Reserved"]
pub type Reserved11R = crate::BitReader;
#[doc = "Field `RESERVED11` writer - 19:19\\]
Reserved"]
pub type Reserved11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "19:19\\]
Lane #5 in Ultra Low Power State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stateulps5Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Stateulps5Irq> for bool {
    #[inline(always)]
    fn from(variant: Stateulps5Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATEULPS5_IRQ` reader - 19:19\\]
Lane #5 in Ultra Low Power State"]
pub type Stateulps5IrqR = crate::BitReader<Stateulps5Irq>;
impl Stateulps5IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stateulps5Irq {
        match self.bits {
            false => Stateulps5Irq::False,
            true => Stateulps5Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Stateulps5Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Stateulps5Irq::True
    }
}
#[doc = "Field `STATEULPS5_IRQ` writer - 19:19\\]
Lane #5 in Ultra Low Power State"]
pub type Stateulps5IrqW<'a, REG> = crate::BitWriter<'a, REG, Stateulps5Irq>;
impl<'a, REG> Stateulps5IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps5Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps5Irq::True)
    }
}
#[doc = "20:20\\]
Contention LP0 error for lane #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp0_1Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errcontentionlp0_1Irq> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp0_1Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP0_1_IRQ` reader - 20:20\\]
Contention LP0 error for lane #1"]
pub type Errcontentionlp0_1IrqR = crate::BitReader<Errcontentionlp0_1Irq>;
impl Errcontentionlp0_1IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp0_1Irq {
        match self.bits {
            false => Errcontentionlp0_1Irq::False,
            true => Errcontentionlp0_1Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errcontentionlp0_1Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errcontentionlp0_1Irq::True
    }
}
#[doc = "Field `ERRCONTENTIONLP0_1_IRQ` writer - 20:20\\]
Contention LP0 error for lane #1"]
pub type Errcontentionlp0_1IrqW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp0_1Irq>;
impl<'a, REG> Errcontentionlp0_1IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_1Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_1Irq::True)
    }
}
#[doc = "21:21\\]
Contention LP1 error for lane #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp1_1Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errcontentionlp1_1Irq> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp1_1Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP1_1_IRQ` reader - 21:21\\]
Contention LP1 error for lane #1"]
pub type Errcontentionlp1_1IrqR = crate::BitReader<Errcontentionlp1_1Irq>;
impl Errcontentionlp1_1IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp1_1Irq {
        match self.bits {
            false => Errcontentionlp1_1Irq::False,
            true => Errcontentionlp1_1Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errcontentionlp1_1Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errcontentionlp1_1Irq::True
    }
}
#[doc = "Field `ERRCONTENTIONLP1_1_IRQ` writer - 21:21\\]
Contention LP1 error for lane #1"]
pub type Errcontentionlp1_1IrqW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp1_1Irq>;
impl<'a, REG> Errcontentionlp1_1IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_1Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_1Irq::True)
    }
}
#[doc = "22:22\\]
Contention LP0 error for lane #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp0_2Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errcontentionlp0_2Irq> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp0_2Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP0_2_IRQ` reader - 22:22\\]
Contention LP0 error for lane #2"]
pub type Errcontentionlp0_2IrqR = crate::BitReader<Errcontentionlp0_2Irq>;
impl Errcontentionlp0_2IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp0_2Irq {
        match self.bits {
            false => Errcontentionlp0_2Irq::False,
            true => Errcontentionlp0_2Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errcontentionlp0_2Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errcontentionlp0_2Irq::True
    }
}
#[doc = "Field `ERRCONTENTIONLP0_2_IRQ` writer - 22:22\\]
Contention LP0 error for lane #2"]
pub type Errcontentionlp0_2IrqW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp0_2Irq>;
impl<'a, REG> Errcontentionlp0_2IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_2Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_2Irq::True)
    }
}
#[doc = "23:23\\]
Contention LP1 error for lane #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp1_2Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errcontentionlp1_2Irq> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp1_2Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP1_2_IRQ` reader - 23:23\\]
Contention LP1 error for lane #2"]
pub type Errcontentionlp1_2IrqR = crate::BitReader<Errcontentionlp1_2Irq>;
impl Errcontentionlp1_2IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp1_2Irq {
        match self.bits {
            false => Errcontentionlp1_2Irq::False,
            true => Errcontentionlp1_2Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errcontentionlp1_2Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errcontentionlp1_2Irq::True
    }
}
#[doc = "Field `ERRCONTENTIONLP1_2_IRQ` writer - 23:23\\]
Contention LP1 error for lane #2"]
pub type Errcontentionlp1_2IrqW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp1_2Irq>;
impl<'a, REG> Errcontentionlp1_2IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_2Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_2Irq::True)
    }
}
#[doc = "Field `RESERVED13` reader - "]
pub type Reserved13R = crate::BitReader;
#[doc = "Field `RESERVED13` writer - "]
pub type Reserved13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "24:24\\]
Contention LP0 error for lane #3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp0_3Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errcontentionlp0_3Irq> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp0_3Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP0_3_IRQ` reader - 24:24\\]
Contention LP0 error for lane #3"]
pub type Errcontentionlp0_3IrqR = crate::BitReader<Errcontentionlp0_3Irq>;
impl Errcontentionlp0_3IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp0_3Irq {
        match self.bits {
            false => Errcontentionlp0_3Irq::False,
            true => Errcontentionlp0_3Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errcontentionlp0_3Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errcontentionlp0_3Irq::True
    }
}
#[doc = "Field `ERRCONTENTIONLP0_3_IRQ` writer - 24:24\\]
Contention LP0 error for lane #3"]
pub type Errcontentionlp0_3IrqW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp0_3Irq>;
impl<'a, REG> Errcontentionlp0_3IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_3Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_3Irq::True)
    }
}
#[doc = "Field `RESERVED14` reader - "]
pub type Reserved14R = crate::BitReader;
#[doc = "Field `RESERVED14` writer - "]
pub type Reserved14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "25:25\\]
Contention LP1 error for lane #3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp1_3Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errcontentionlp1_3Irq> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp1_3Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP1_3_IRQ` reader - 25:25\\]
Contention LP1 error for lane #3"]
pub type Errcontentionlp1_3IrqR = crate::BitReader<Errcontentionlp1_3Irq>;
impl Errcontentionlp1_3IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp1_3Irq {
        match self.bits {
            false => Errcontentionlp1_3Irq::False,
            true => Errcontentionlp1_3Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errcontentionlp1_3Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errcontentionlp1_3Irq::True
    }
}
#[doc = "Field `ERRCONTENTIONLP1_3_IRQ` writer - 25:25\\]
Contention LP1 error for lane #3"]
pub type Errcontentionlp1_3IrqW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp1_3Irq>;
impl<'a, REG> Errcontentionlp1_3IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_3Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_3Irq::True)
    }
}
#[doc = "Field `RESERVED15` reader - "]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `RESERVED15` writer - "]
pub type Reserved15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "26:26\\]
Contention LP0 error for lane #4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp0_4Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errcontentionlp0_4Irq> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp0_4Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP0_4_IRQ` reader - 26:26\\]
Contention LP0 error for lane #4"]
pub type Errcontentionlp0_4IrqR = crate::BitReader<Errcontentionlp0_4Irq>;
impl Errcontentionlp0_4IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp0_4Irq {
        match self.bits {
            false => Errcontentionlp0_4Irq::False,
            true => Errcontentionlp0_4Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errcontentionlp0_4Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errcontentionlp0_4Irq::True
    }
}
#[doc = "Field `ERRCONTENTIONLP0_4_IRQ` writer - 26:26\\]
Contention LP0 error for lane #4"]
pub type Errcontentionlp0_4IrqW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp0_4Irq>;
impl<'a, REG> Errcontentionlp0_4IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_4Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_4Irq::True)
    }
}
#[doc = "Field `RESERVED16` reader - "]
pub type Reserved16R = crate::BitReader;
#[doc = "Field `RESERVED16` writer - "]
pub type Reserved16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "27:27\\]
Contention LP1 error for lane #4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp1_4Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errcontentionlp1_4Irq> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp1_4Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP1_4_IRQ` reader - 27:27\\]
Contention LP1 error for lane #4"]
pub type Errcontentionlp1_4IrqR = crate::BitReader<Errcontentionlp1_4Irq>;
impl Errcontentionlp1_4IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp1_4Irq {
        match self.bits {
            false => Errcontentionlp1_4Irq::False,
            true => Errcontentionlp1_4Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errcontentionlp1_4Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errcontentionlp1_4Irq::True
    }
}
#[doc = "Field `ERRCONTENTIONLP1_4_IRQ` writer - 27:27\\]
Contention LP1 error for lane #4"]
pub type Errcontentionlp1_4IrqW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp1_4Irq>;
impl<'a, REG> Errcontentionlp1_4IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_4Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_4Irq::True)
    }
}
#[doc = "Field `RESERVED17` reader - "]
pub type Reserved17R = crate::BitReader;
#[doc = "Field `RESERVED17` writer - "]
pub type Reserved17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "28:28\\]
Contention LP0 error for lane #5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp0_5Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errcontentionlp0_5Irq> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp0_5Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP0_5_IRQ` reader - 28:28\\]
Contention LP0 error for lane #5"]
pub type Errcontentionlp0_5IrqR = crate::BitReader<Errcontentionlp0_5Irq>;
impl Errcontentionlp0_5IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp0_5Irq {
        match self.bits {
            false => Errcontentionlp0_5Irq::False,
            true => Errcontentionlp0_5Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errcontentionlp0_5Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errcontentionlp0_5Irq::True
    }
}
#[doc = "Field `ERRCONTENTIONLP0_5_IRQ` writer - 28:28\\]
Contention LP0 error for lane #5"]
pub type Errcontentionlp0_5IrqW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp0_5Irq>;
impl<'a, REG> Errcontentionlp0_5IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_5Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_5Irq::True)
    }
}
#[doc = "Field `RESERVED18` reader - "]
pub type Reserved18R = crate::BitReader;
#[doc = "Field `RESERVED18` writer - "]
pub type Reserved18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "29:29\\]
Contention LP1 error for lane #5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp1_5Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<Errcontentionlp1_5Irq> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp1_5Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP1_5_IRQ` reader - 29:29\\]
Contention LP1 error for lane #5"]
pub type Errcontentionlp1_5IrqR = crate::BitReader<Errcontentionlp1_5Irq>;
impl Errcontentionlp1_5IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp1_5Irq {
        match self.bits {
            false => Errcontentionlp1_5Irq::False,
            true => Errcontentionlp1_5Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Errcontentionlp1_5Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Errcontentionlp1_5Irq::True
    }
}
#[doc = "Field `ERRCONTENTIONLP1_5_IRQ` writer - 29:29\\]
Contention LP1 error for lane #5"]
pub type Errcontentionlp1_5IrqW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp1_5Irq>;
impl<'a, REG> Errcontentionlp1_5IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_5Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_5Irq::True)
    }
}
#[doc = "30:30\\]
All signals ULPSActiveNOT are 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UlpsactivenotAll0Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<UlpsactivenotAll0Irq> for bool {
    #[inline(always)]
    fn from(variant: UlpsactivenotAll0Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPSACTIVENOT_ALL0_IRQ` reader - 30:30\\]
All signals ULPSActiveNOT are 0"]
pub type UlpsactivenotAll0IrqR = crate::BitReader<UlpsactivenotAll0Irq>;
impl UlpsactivenotAll0IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UlpsactivenotAll0Irq {
        match self.bits {
            false => UlpsactivenotAll0Irq::False,
            true => UlpsactivenotAll0Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == UlpsactivenotAll0Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == UlpsactivenotAll0Irq::True
    }
}
#[doc = "Field `ULPSACTIVENOT_ALL0_IRQ` writer - 30:30\\]
All signals ULPSActiveNOT are 0"]
pub type UlpsactivenotAll0IrqW<'a, REG> = crate::BitWriter<'a, REG, UlpsactivenotAll0Irq>;
impl<'a, REG> UlpsactivenotAll0IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(UlpsactivenotAll0Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(UlpsactivenotAll0Irq::True)
    }
}
#[doc = "31:31\\]
All the ULPSActiveNOT signals corresponding to the lanes with TXULPSExit being high are high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UlpsactivenotAll1Irq {
    #[doc = "0: READS: Event is false. WRITES: Status bit unchanged."]
    False = 0,
    #[doc = "1: READS: Event is true (pending). WRITES: Status bit is reset."]
    True = 1,
}
impl From<UlpsactivenotAll1Irq> for bool {
    #[inline(always)]
    fn from(variant: UlpsactivenotAll1Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPSACTIVENOT_ALL1_IRQ` reader - 31:31\\]
All the ULPSActiveNOT signals corresponding to the lanes with TXULPSExit being high are high."]
pub type UlpsactivenotAll1IrqR = crate::BitReader<UlpsactivenotAll1Irq>;
impl UlpsactivenotAll1IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UlpsactivenotAll1Irq {
        match self.bits {
            false => UlpsactivenotAll1Irq::False,
            true => UlpsactivenotAll1Irq::True,
        }
    }
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == UlpsactivenotAll1Irq::False
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == UlpsactivenotAll1Irq::True
    }
}
#[doc = "Field `ULPSACTIVENOT_ALL1_IRQ` writer - 31:31\\]
All the ULPSActiveNOT signals corresponding to the lanes with TXULPSExit being high are high."]
pub type UlpsactivenotAll1IrqW<'a, REG> = crate::BitWriter<'a, REG, UlpsactivenotAll1Irq>;
impl<'a, REG> UlpsactivenotAll1IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "READS: Event is false. WRITES: Status bit unchanged."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(UlpsactivenotAll1Irq::False)
    }
    #[doc = "READS: Event is true (pending). WRITES: Status bit is reset."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(UlpsactivenotAll1Irq::True)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Low power Data transmission synchronization error for lane #1"]
    #[inline(always)]
    pub fn errsyncesc1_irq(&self) -> Errsyncesc1IrqR {
        Errsyncesc1IrqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Low power Data transmission synchronization error for lane #2"]
    #[inline(always)]
    pub fn errsyncesc2_irq(&self) -> Errsyncesc2IrqR {
        Errsyncesc2IrqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Low power Data transmission synchronization error for lane #3"]
    #[inline(always)]
    pub fn errsyncesc3_irq(&self) -> Errsyncesc3IrqR {
        Errsyncesc3IrqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Low power Data transmission synchronization error for lane #4"]
    #[inline(always)]
    pub fn errsyncesc4_irq(&self) -> Errsyncesc4IrqR {
        Errsyncesc4IrqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Low power Data transmission synchronization error for lane #5"]
    #[inline(always)]
    pub fn errsyncesc5_irq(&self) -> Errsyncesc5IrqR {
        Errsyncesc5IrqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Escape entry error for lane #1"]
    #[inline(always)]
    pub fn erresc1_irq(&self) -> Erresc1IrqR {
        Erresc1IrqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Escape entry error for lane #2"]
    #[inline(always)]
    pub fn erresc2_irq(&self) -> Erresc2IrqR {
        Erresc2IrqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Escape entry error for lane #3"]
    #[inline(always)]
    pub fn erresc3_irq(&self) -> Erresc3IrqR {
        Erresc3IrqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Escape entry error for lane #4"]
    #[inline(always)]
    pub fn erresc4_irq(&self) -> Erresc4IrqR {
        Erresc4IrqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Escape entry error for lane #5"]
    #[inline(always)]
    pub fn erresc5_irq(&self) -> Erresc5IrqR {
        Erresc5IrqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Control error for lane #1"]
    #[inline(always)]
    pub fn errcontrol1_irq(&self) -> Errcontrol1IrqR {
        Errcontrol1IrqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Control error for lane #2"]
    #[inline(always)]
    pub fn errcontrol2_irq(&self) -> Errcontrol2IrqR {
        Errcontrol2IrqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Control error for lane #3"]
    #[inline(always)]
    pub fn errcontrol3_irq(&self) -> Errcontrol3IrqR {
        Errcontrol3IrqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Control error for lane #4"]
    #[inline(always)]
    pub fn errcontrol4_irq(&self) -> Errcontrol4IrqR {
        Errcontrol4IrqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Control error for lane #5"]
    #[inline(always)]
    pub fn errcontrol5_irq(&self) -> Errcontrol5IrqR {
        Errcontrol5IrqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Lane #1 in Ultra Low Power State"]
    #[inline(always)]
    pub fn stateulps1_irq(&self) -> Stateulps1IrqR {
        Stateulps1IrqR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Lane #2 in Ultra Low Power State"]
    #[inline(always)]
    pub fn stateulps2_irq(&self) -> Stateulps2IrqR {
        Stateulps2IrqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Lane #3 in Ultra Low Power State"]
    #[inline(always)]
    pub fn stateulps3_irq(&self) -> Stateulps3IrqR {
        Stateulps3IrqR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Reserved"]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Lane #4 in Ultra Low Power Mode"]
    #[inline(always)]
    pub fn stateulps4_irq(&self) -> Stateulps4IrqR {
        Stateulps4IrqR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Reserved"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Lane #5 in Ultra Low Power State"]
    #[inline(always)]
    pub fn stateulps5_irq(&self) -> Stateulps5IrqR {
        Stateulps5IrqR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Contention LP0 error for lane #1"]
    #[inline(always)]
    pub fn errcontentionlp0_1_irq(&self) -> Errcontentionlp0_1IrqR {
        Errcontentionlp0_1IrqR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Contention LP1 error for lane #1"]
    #[inline(always)]
    pub fn errcontentionlp1_1_irq(&self) -> Errcontentionlp1_1IrqR {
        Errcontentionlp1_1IrqR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Contention LP0 error for lane #2"]
    #[inline(always)]
    pub fn errcontentionlp0_2_irq(&self) -> Errcontentionlp0_2IrqR {
        Errcontentionlp0_2IrqR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Contention LP1 error for lane #2"]
    #[inline(always)]
    pub fn errcontentionlp1_2_irq(&self) -> Errcontentionlp1_2IrqR {
        Errcontentionlp1_2IrqR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Contention LP0 error for lane #3"]
    #[inline(always)]
    pub fn errcontentionlp0_3_irq(&self) -> Errcontentionlp0_3IrqR {
        Errcontentionlp0_3IrqR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Contention LP1 error for lane #3"]
    #[inline(always)]
    pub fn errcontentionlp1_3_irq(&self) -> Errcontentionlp1_3IrqR {
        Errcontentionlp1_3IrqR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Contention LP0 error for lane #4"]
    #[inline(always)]
    pub fn errcontentionlp0_4_irq(&self) -> Errcontentionlp0_4IrqR {
        Errcontentionlp0_4IrqR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Contention LP1 error for lane #4"]
    #[inline(always)]
    pub fn errcontentionlp1_4_irq(&self) -> Errcontentionlp1_4IrqR {
        Errcontentionlp1_4IrqR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Contention LP0 error for lane #5"]
    #[inline(always)]
    pub fn errcontentionlp0_5_irq(&self) -> Errcontentionlp0_5IrqR {
        Errcontentionlp0_5IrqR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Contention LP1 error for lane #5"]
    #[inline(always)]
    pub fn errcontentionlp1_5_irq(&self) -> Errcontentionlp1_5IrqR {
        Errcontentionlp1_5IrqR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
All signals ULPSActiveNOT are 0"]
    #[inline(always)]
    pub fn ulpsactivenot_all0_irq(&self) -> UlpsactivenotAll0IrqR {
        UlpsactivenotAll0IrqR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
All the ULPSActiveNOT signals corresponding to the lanes with TXULPSExit being high are high."]
    #[inline(always)]
    pub fn ulpsactivenot_all1_irq(&self) -> UlpsactivenotAll1IrqR {
        UlpsactivenotAll1IrqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Low power Data transmission synchronization error for lane #1"]
    #[inline(always)]
    #[must_use]
    pub fn errsyncesc1_irq(&mut self) -> Errsyncesc1IrqW<Csi2ComplexioIrqstatusSpec> {
        Errsyncesc1IrqW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Low power Data transmission synchronization error for lane #2"]
    #[inline(always)]
    #[must_use]
    pub fn errsyncesc2_irq(&mut self) -> Errsyncesc2IrqW<Csi2ComplexioIrqstatusSpec> {
        Errsyncesc2IrqW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Low power Data transmission synchronization error for lane #3"]
    #[inline(always)]
    #[must_use]
    pub fn errsyncesc3_irq(&mut self) -> Errsyncesc3IrqW<Csi2ComplexioIrqstatusSpec> {
        Errsyncesc3IrqW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Csi2ComplexioIrqstatusSpec> {
        Reserved1W::new(self, 3)
    }
    #[doc = "Bit 3 - 3:3\\]
Low power Data transmission synchronization error for lane #4"]
    #[inline(always)]
    #[must_use]
    pub fn errsyncesc4_irq(&mut self) -> Errsyncesc4IrqW<Csi2ComplexioIrqstatusSpec> {
        Errsyncesc4IrqW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Csi2ComplexioIrqstatusSpec> {
        Reserved2W::new(self, 4)
    }
    #[doc = "Bit 4 - 4:4\\]
Low power Data transmission synchronization error for lane #5"]
    #[inline(always)]
    #[must_use]
    pub fn errsyncesc5_irq(&mut self) -> Errsyncesc5IrqW<Csi2ComplexioIrqstatusSpec> {
        Errsyncesc5IrqW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Escape entry error for lane #1"]
    #[inline(always)]
    #[must_use]
    pub fn erresc1_irq(&mut self) -> Erresc1IrqW<Csi2ComplexioIrqstatusSpec> {
        Erresc1IrqW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Escape entry error for lane #2"]
    #[inline(always)]
    #[must_use]
    pub fn erresc2_irq(&mut self) -> Erresc2IrqW<Csi2ComplexioIrqstatusSpec> {
        Erresc2IrqW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Csi2ComplexioIrqstatusSpec> {
        Reserved3W::new(self, 7)
    }
    #[doc = "Bit 7 - 7:7\\]
Escape entry error for lane #3"]
    #[inline(always)]
    #[must_use]
    pub fn erresc3_irq(&mut self) -> Erresc3IrqW<Csi2ComplexioIrqstatusSpec> {
        Erresc3IrqW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<Csi2ComplexioIrqstatusSpec> {
        Reserved4W::new(self, 8)
    }
    #[doc = "Bit 8 - 8:8\\]
Escape entry error for lane #4"]
    #[inline(always)]
    #[must_use]
    pub fn erresc4_irq(&mut self) -> Erresc4IrqW<Csi2ComplexioIrqstatusSpec> {
        Erresc4IrqW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<Csi2ComplexioIrqstatusSpec> {
        Reserved5W::new(self, 9)
    }
    #[doc = "Bit 9 - 9:9\\]
Escape entry error for lane #5"]
    #[inline(always)]
    #[must_use]
    pub fn erresc5_irq(&mut self) -> Erresc5IrqW<Csi2ComplexioIrqstatusSpec> {
        Erresc5IrqW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Control error for lane #1"]
    #[inline(always)]
    #[must_use]
    pub fn errcontrol1_irq(&mut self) -> Errcontrol1IrqW<Csi2ComplexioIrqstatusSpec> {
        Errcontrol1IrqW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Control error for lane #2"]
    #[inline(always)]
    #[must_use]
    pub fn errcontrol2_irq(&mut self) -> Errcontrol2IrqW<Csi2ComplexioIrqstatusSpec> {
        Errcontrol2IrqW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<Csi2ComplexioIrqstatusSpec> {
        Reserved6W::new(self, 12)
    }
    #[doc = "Bit 12 - 12:12\\]
Control error for lane #3"]
    #[inline(always)]
    #[must_use]
    pub fn errcontrol3_irq(&mut self) -> Errcontrol3IrqW<Csi2ComplexioIrqstatusSpec> {
        Errcontrol3IrqW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<Csi2ComplexioIrqstatusSpec> {
        Reserved7W::new(self, 13)
    }
    #[doc = "Bit 13 - 13:13\\]
Control error for lane #4"]
    #[inline(always)]
    #[must_use]
    pub fn errcontrol4_irq(&mut self) -> Errcontrol4IrqW<Csi2ComplexioIrqstatusSpec> {
        Errcontrol4IrqW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Csi2ComplexioIrqstatusSpec> {
        Reserved8W::new(self, 14)
    }
    #[doc = "Bit 14 - 14:14\\]
Control error for lane #5"]
    #[inline(always)]
    #[must_use]
    pub fn errcontrol5_irq(&mut self) -> Errcontrol5IrqW<Csi2ComplexioIrqstatusSpec> {
        Errcontrol5IrqW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Lane #1 in Ultra Low Power State"]
    #[inline(always)]
    #[must_use]
    pub fn stateulps1_irq(&mut self) -> Stateulps1IrqW<Csi2ComplexioIrqstatusSpec> {
        Stateulps1IrqW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Lane #2 in Ultra Low Power State"]
    #[inline(always)]
    #[must_use]
    pub fn stateulps2_irq(&mut self) -> Stateulps2IrqW<Csi2ComplexioIrqstatusSpec> {
        Stateulps2IrqW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<Csi2ComplexioIrqstatusSpec> {
        Reserved9W::new(self, 17)
    }
    #[doc = "Bit 17 - 17:17\\]
Lane #3 in Ultra Low Power State"]
    #[inline(always)]
    #[must_use]
    pub fn stateulps3_irq(&mut self) -> Stateulps3IrqW<Csi2ComplexioIrqstatusSpec> {
        Stateulps3IrqW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<Csi2ComplexioIrqstatusSpec> {
        Reserved10W::new(self, 18)
    }
    #[doc = "Bit 18 - 18:18\\]
Lane #4 in Ultra Low Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn stateulps4_irq(&mut self) -> Stateulps4IrqW<Csi2ComplexioIrqstatusSpec> {
        Stateulps4IrqW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<Csi2ComplexioIrqstatusSpec> {
        Reserved11W::new(self, 19)
    }
    #[doc = "Bit 19 - 19:19\\]
Lane #5 in Ultra Low Power State"]
    #[inline(always)]
    #[must_use]
    pub fn stateulps5_irq(&mut self) -> Stateulps5IrqW<Csi2ComplexioIrqstatusSpec> {
        Stateulps5IrqW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Contention LP0 error for lane #1"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp0_1_irq(&mut self) -> Errcontentionlp0_1IrqW<Csi2ComplexioIrqstatusSpec> {
        Errcontentionlp0_1IrqW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Contention LP1 error for lane #1"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp1_1_irq(&mut self) -> Errcontentionlp1_1IrqW<Csi2ComplexioIrqstatusSpec> {
        Errcontentionlp1_1IrqW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Contention LP0 error for lane #2"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp0_2_irq(&mut self) -> Errcontentionlp0_2IrqW<Csi2ComplexioIrqstatusSpec> {
        Errcontentionlp0_2IrqW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Contention LP1 error for lane #2"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp1_2_irq(&mut self) -> Errcontentionlp1_2IrqW<Csi2ComplexioIrqstatusSpec> {
        Errcontentionlp1_2IrqW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> Reserved13W<Csi2ComplexioIrqstatusSpec> {
        Reserved13W::new(self, 24)
    }
    #[doc = "Bit 24 - 24:24\\]
Contention LP0 error for lane #3"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp0_3_irq(&mut self) -> Errcontentionlp0_3IrqW<Csi2ComplexioIrqstatusSpec> {
        Errcontentionlp0_3IrqW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<Csi2ComplexioIrqstatusSpec> {
        Reserved14W::new(self, 25)
    }
    #[doc = "Bit 25 - 25:25\\]
Contention LP1 error for lane #3"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp1_3_irq(&mut self) -> Errcontentionlp1_3IrqW<Csi2ComplexioIrqstatusSpec> {
        Errcontentionlp1_3IrqW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<Csi2ComplexioIrqstatusSpec> {
        Reserved15W::new(self, 26)
    }
    #[doc = "Bit 26 - 26:26\\]
Contention LP0 error for lane #4"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp0_4_irq(&mut self) -> Errcontentionlp0_4IrqW<Csi2ComplexioIrqstatusSpec> {
        Errcontentionlp0_4IrqW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Csi2ComplexioIrqstatusSpec> {
        Reserved16W::new(self, 27)
    }
    #[doc = "Bit 27 - 27:27\\]
Contention LP1 error for lane #4"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp1_4_irq(&mut self) -> Errcontentionlp1_4IrqW<Csi2ComplexioIrqstatusSpec> {
        Errcontentionlp1_4IrqW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<Csi2ComplexioIrqstatusSpec> {
        Reserved17W::new(self, 28)
    }
    #[doc = "Bit 28 - 28:28\\]
Contention LP0 error for lane #5"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp0_5_irq(&mut self) -> Errcontentionlp0_5IrqW<Csi2ComplexioIrqstatusSpec> {
        Errcontentionlp0_5IrqW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> Reserved18W<Csi2ComplexioIrqstatusSpec> {
        Reserved18W::new(self, 29)
    }
    #[doc = "Bit 29 - 29:29\\]
Contention LP1 error for lane #5"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp1_5_irq(&mut self) -> Errcontentionlp1_5IrqW<Csi2ComplexioIrqstatusSpec> {
        Errcontentionlp1_5IrqW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
All signals ULPSActiveNOT are 0"]
    #[inline(always)]
    #[must_use]
    pub fn ulpsactivenot_all0_irq(&mut self) -> UlpsactivenotAll0IrqW<Csi2ComplexioIrqstatusSpec> {
        UlpsactivenotAll0IrqW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
All the ULPSActiveNOT signals corresponding to the lanes with TXULPSExit being high are high."]
    #[inline(always)]
    #[must_use]
    pub fn ulpsactivenot_all1_irq(&mut self) -> UlpsactivenotAll1IrqW<Csi2ComplexioIrqstatusSpec> {
        UlpsactivenotAll1IrqW::new(self, 31)
    }
}
#[doc = "INTERRUPT STATUS REGISTER - All errors from complex IO\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_complexio_irqstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_complexio_irqstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2ComplexioIrqstatusSpec;
impl crate::RegisterSpec for Csi2ComplexioIrqstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_complexio_irqstatus::R`](R) reader structure"]
impl crate::Readable for Csi2ComplexioIrqstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_complexio_irqstatus::W`](W) writer structure"]
impl crate::Writable for Csi2ComplexioIrqstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_COMPLEXIO_IRQSTATUS to value 0"]
impl crate::Resettable for Csi2ComplexioIrqstatusSpec {
    const RESET_VALUE: u32 = 0;
}
