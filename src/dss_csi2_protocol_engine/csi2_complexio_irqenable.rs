#[doc = "Register `CSI2_COMPLEXIO_IRQENABLE` reader"]
pub type R = crate::R<Csi2ComplexioIrqenableSpec>;
#[doc = "Register `CSI2_COMPLEXIO_IRQENABLE` writer"]
pub type W = crate::W<Csi2ComplexioIrqenableSpec>;
#[doc = "0:0\\]
Low power Data transmission synchronization error for lane #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errsyncsesc1IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errsyncsesc1IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errsyncsesc1IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSYNCSESC1_IRQ_EN` reader - 0:0\\]
Low power Data transmission synchronization error for lane #1"]
pub type Errsyncsesc1IrqEnR = crate::BitReader<Errsyncsesc1IrqEn>;
impl Errsyncsesc1IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errsyncsesc1IrqEn {
        match self.bits {
            false => Errsyncsesc1IrqEn::Disable,
            true => Errsyncsesc1IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errsyncsesc1IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errsyncsesc1IrqEn::Enable
    }
}
#[doc = "Field `ERRSYNCSESC1_IRQ_EN` writer - 0:0\\]
Low power Data transmission synchronization error for lane #1"]
pub type Errsyncsesc1IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errsyncsesc1IrqEn>;
impl<'a, REG> Errsyncsesc1IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncsesc1IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncsesc1IrqEn::Enable)
    }
}
#[doc = "1:1\\]
Low power Data transmission synchronization error for lane #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errsyncsesc2IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errsyncsesc2IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errsyncsesc2IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSYNCSESC2_IRQ_EN` reader - 1:1\\]
Low power Data transmission synchronization error for lane #2"]
pub type Errsyncsesc2IrqEnR = crate::BitReader<Errsyncsesc2IrqEn>;
impl Errsyncsesc2IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errsyncsesc2IrqEn {
        match self.bits {
            false => Errsyncsesc2IrqEn::Disable,
            true => Errsyncsesc2IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errsyncsesc2IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errsyncsesc2IrqEn::Enable
    }
}
#[doc = "Field `ERRSYNCSESC2_IRQ_EN` writer - 1:1\\]
Low power Data transmission synchronization error for lane #2"]
pub type Errsyncsesc2IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errsyncsesc2IrqEn>;
impl<'a, REG> Errsyncsesc2IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncsesc2IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncsesc2IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED1` reader - "]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - "]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "2:2\\]
Low power Data transmission synchronization error for lane #3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errsyncsesc3IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errsyncsesc3IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errsyncsesc3IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSYNCSESC3_IRQ_EN` reader - 2:2\\]
Low power Data transmission synchronization error for lane #3"]
pub type Errsyncsesc3IrqEnR = crate::BitReader<Errsyncsesc3IrqEn>;
impl Errsyncsesc3IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errsyncsesc3IrqEn {
        match self.bits {
            false => Errsyncsesc3IrqEn::Disable,
            true => Errsyncsesc3IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errsyncsesc3IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errsyncsesc3IrqEn::Enable
    }
}
#[doc = "Field `ERRSYNCSESC3_IRQ_EN` writer - 2:2\\]
Low power Data transmission synchronization error for lane #3"]
pub type Errsyncsesc3IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errsyncsesc3IrqEn>;
impl<'a, REG> Errsyncsesc3IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncsesc3IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncsesc3IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED2` reader - "]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `RESERVED2` writer - "]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "3:3\\]
Low power Data transmission synchronization error for lane #4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errsyncsesc4IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errsyncsesc4IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errsyncsesc4IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSYNCSESC4_IRQ_EN` reader - 3:3\\]
Low power Data transmission synchronization error for lane #4"]
pub type Errsyncsesc4IrqEnR = crate::BitReader<Errsyncsesc4IrqEn>;
impl Errsyncsesc4IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errsyncsesc4IrqEn {
        match self.bits {
            false => Errsyncsesc4IrqEn::Disable,
            true => Errsyncsesc4IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errsyncsesc4IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errsyncsesc4IrqEn::Enable
    }
}
#[doc = "Field `ERRSYNCSESC4_IRQ_EN` writer - 3:3\\]
Low power Data transmission synchronization error for lane #4"]
pub type Errsyncsesc4IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errsyncsesc4IrqEn>;
impl<'a, REG> Errsyncsesc4IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncsesc4IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncsesc4IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED3` reader - "]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RESERVED3` writer - "]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "4:4\\]
Low power Data transmission synchronization error for lane #5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errsyncsesc5IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errsyncsesc5IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errsyncsesc5IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSYNCSESC5_IRQ_EN` reader - 4:4\\]
Low power Data transmission synchronization error for lane #5"]
pub type Errsyncsesc5IrqEnR = crate::BitReader<Errsyncsesc5IrqEn>;
impl Errsyncsesc5IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errsyncsesc5IrqEn {
        match self.bits {
            false => Errsyncsesc5IrqEn::Disable,
            true => Errsyncsesc5IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errsyncsesc5IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errsyncsesc5IrqEn::Enable
    }
}
#[doc = "Field `ERRSYNCSESC5_IRQ_EN` writer - 4:4\\]
Low power Data transmission synchronization error for lane #5"]
pub type Errsyncsesc5IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errsyncsesc5IrqEn>;
impl<'a, REG> Errsyncsesc5IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncsesc5IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errsyncsesc5IrqEn::Enable)
    }
}
#[doc = "5:5\\]
Escape entry error for lane #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erresc1IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Erresc1IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Erresc1IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRESC1_IRQ_EN` reader - 5:5\\]
Escape entry error for lane #1"]
pub type Erresc1IrqEnR = crate::BitReader<Erresc1IrqEn>;
impl Erresc1IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erresc1IrqEn {
        match self.bits {
            false => Erresc1IrqEn::Disable,
            true => Erresc1IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Erresc1IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Erresc1IrqEn::Enable
    }
}
#[doc = "Field `ERRESC1_IRQ_EN` writer - 5:5\\]
Escape entry error for lane #1"]
pub type Erresc1IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Erresc1IrqEn>;
impl<'a, REG> Erresc1IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc1IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc1IrqEn::Enable)
    }
}
#[doc = "6:6\\]
Escape entry error for lane #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erresc2IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Erresc2IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Erresc2IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRESC2_IRQ_EN` reader - 6:6\\]
Escape entry error for lane #2"]
pub type Erresc2IrqEnR = crate::BitReader<Erresc2IrqEn>;
impl Erresc2IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erresc2IrqEn {
        match self.bits {
            false => Erresc2IrqEn::Disable,
            true => Erresc2IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Erresc2IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Erresc2IrqEn::Enable
    }
}
#[doc = "Field `ERRESC2_IRQ_EN` writer - 6:6\\]
Escape entry error for lane #2"]
pub type Erresc2IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Erresc2IrqEn>;
impl<'a, REG> Erresc2IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc2IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc2IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED4` reader - "]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `RESERVED4` writer - "]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "7:7\\]
Escape entry error for lane #3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erresc3IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Erresc3IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Erresc3IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRESC3_IRQ_EN` reader - 7:7\\]
Escape entry error for lane #3"]
pub type Erresc3IrqEnR = crate::BitReader<Erresc3IrqEn>;
impl Erresc3IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erresc3IrqEn {
        match self.bits {
            false => Erresc3IrqEn::Disable,
            true => Erresc3IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Erresc3IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Erresc3IrqEn::Enable
    }
}
#[doc = "Field `ERRESC3_IRQ_EN` writer - 7:7\\]
Escape entry error for lane #3"]
pub type Erresc3IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Erresc3IrqEn>;
impl<'a, REG> Erresc3IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc3IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc3IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED5` reader - "]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `RESERVED5` writer - "]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "8:8\\]
Escape entry error for lane #4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erresc4IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Erresc4IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Erresc4IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRESC4_IRQ_EN` reader - 8:8\\]
Escape entry error for lane #4"]
pub type Erresc4IrqEnR = crate::BitReader<Erresc4IrqEn>;
impl Erresc4IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erresc4IrqEn {
        match self.bits {
            false => Erresc4IrqEn::Disable,
            true => Erresc4IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Erresc4IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Erresc4IrqEn::Enable
    }
}
#[doc = "Field `ERRESC4_IRQ_EN` writer - 8:8\\]
Escape entry error for lane #4"]
pub type Erresc4IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Erresc4IrqEn>;
impl<'a, REG> Erresc4IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc4IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc4IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED6` reader - "]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `RESERVED6` writer - "]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "9:9\\]
Escape entry error for lane #5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erresc5IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Erresc5IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Erresc5IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRESC5_IRQ_EN` reader - 9:9\\]
Escape entry error for lane #5"]
pub type Erresc5IrqEnR = crate::BitReader<Erresc5IrqEn>;
impl Erresc5IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erresc5IrqEn {
        match self.bits {
            false => Erresc5IrqEn::Disable,
            true => Erresc5IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Erresc5IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Erresc5IrqEn::Enable
    }
}
#[doc = "Field `ERRESC5_IRQ_EN` writer - 9:9\\]
Escape entry error for lane #5"]
pub type Erresc5IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Erresc5IrqEn>;
impl<'a, REG> Erresc5IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc5IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Erresc5IrqEn::Enable)
    }
}
#[doc = "10:10\\]
Control error for lane #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontrol1IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errcontrol1IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errcontrol1IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTROL1_IRQ_EN` reader - 10:10\\]
Control error for lane #1"]
pub type Errcontrol1IrqEnR = crate::BitReader<Errcontrol1IrqEn>;
impl Errcontrol1IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontrol1IrqEn {
        match self.bits {
            false => Errcontrol1IrqEn::Disable,
            true => Errcontrol1IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errcontrol1IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errcontrol1IrqEn::Enable
    }
}
#[doc = "Field `ERRCONTROL1_IRQ_EN` writer - 10:10\\]
Control error for lane #1"]
pub type Errcontrol1IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errcontrol1IrqEn>;
impl<'a, REG> Errcontrol1IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol1IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol1IrqEn::Enable)
    }
}
#[doc = "11:11\\]
Control error for lane #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontrol2IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errcontrol2IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errcontrol2IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTROL2_IRQ_EN` reader - 11:11\\]
Control error for lane #2"]
pub type Errcontrol2IrqEnR = crate::BitReader<Errcontrol2IrqEn>;
impl Errcontrol2IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontrol2IrqEn {
        match self.bits {
            false => Errcontrol2IrqEn::Disable,
            true => Errcontrol2IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errcontrol2IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errcontrol2IrqEn::Enable
    }
}
#[doc = "Field `ERRCONTROL2_IRQ_EN` writer - 11:11\\]
Control error for lane #2"]
pub type Errcontrol2IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errcontrol2IrqEn>;
impl<'a, REG> Errcontrol2IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol2IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol2IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED7` reader - "]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESERVED7` writer - "]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "12:12\\]
Control error for lane #3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontrol3IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errcontrol3IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errcontrol3IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTROL3_IRQ_EN` reader - 12:12\\]
Control error for lane #3"]
pub type Errcontrol3IrqEnR = crate::BitReader<Errcontrol3IrqEn>;
impl Errcontrol3IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontrol3IrqEn {
        match self.bits {
            false => Errcontrol3IrqEn::Disable,
            true => Errcontrol3IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errcontrol3IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errcontrol3IrqEn::Enable
    }
}
#[doc = "Field `ERRCONTROL3_IRQ_EN` writer - 12:12\\]
Control error for lane #3"]
pub type Errcontrol3IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errcontrol3IrqEn>;
impl<'a, REG> Errcontrol3IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol3IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol3IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED8` reader - "]
pub type Reserved8R = crate::BitReader;
#[doc = "Field `RESERVED8` writer - "]
pub type Reserved8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "13:13\\]
Control error for lane #4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontrol4IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errcontrol4IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errcontrol4IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTROL4_IRQ_EN` reader - 13:13\\]
Control error for lane #4"]
pub type Errcontrol4IrqEnR = crate::BitReader<Errcontrol4IrqEn>;
impl Errcontrol4IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontrol4IrqEn {
        match self.bits {
            false => Errcontrol4IrqEn::Disable,
            true => Errcontrol4IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errcontrol4IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errcontrol4IrqEn::Enable
    }
}
#[doc = "Field `ERRCONTROL4_IRQ_EN` writer - 13:13\\]
Control error for lane #4"]
pub type Errcontrol4IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errcontrol4IrqEn>;
impl<'a, REG> Errcontrol4IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol4IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol4IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED9` reader - "]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `RESERVED9` writer - "]
pub type Reserved9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "14:14\\]
Control error for lane #5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontrol5IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errcontrol5IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errcontrol5IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTROL5_IRQ_EN` reader - 14:14\\]
Control error for lane #5"]
pub type Errcontrol5IrqEnR = crate::BitReader<Errcontrol5IrqEn>;
impl Errcontrol5IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontrol5IrqEn {
        match self.bits {
            false => Errcontrol5IrqEn::Disable,
            true => Errcontrol5IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errcontrol5IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errcontrol5IrqEn::Enable
    }
}
#[doc = "Field `ERRCONTROL5_IRQ_EN` writer - 14:14\\]
Control error for lane #5"]
pub type Errcontrol5IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errcontrol5IrqEn>;
impl<'a, REG> Errcontrol5IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol5IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontrol5IrqEn::Enable)
    }
}
#[doc = "15:15\\]
Lane #1 in Ultra Low Power State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stateulps1IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Stateulps1IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Stateulps1IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATEULPS1_IRQ_EN` reader - 15:15\\]
Lane #1 in Ultra Low Power State"]
pub type Stateulps1IrqEnR = crate::BitReader<Stateulps1IrqEn>;
impl Stateulps1IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stateulps1IrqEn {
        match self.bits {
            false => Stateulps1IrqEn::Disable,
            true => Stateulps1IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Stateulps1IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Stateulps1IrqEn::Enable
    }
}
#[doc = "Field `STATEULPS1_IRQ_EN` writer - 15:15\\]
Lane #1 in Ultra Low Power State"]
pub type Stateulps1IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Stateulps1IrqEn>;
impl<'a, REG> Stateulps1IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps1IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps1IrqEn::Enable)
    }
}
#[doc = "16:16\\]
Lane #2 in Ultra Low Power State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stateulps2IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Stateulps2IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Stateulps2IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATEULPS2_IRQ_EN` reader - 16:16\\]
Lane #2 in Ultra Low Power State"]
pub type Stateulps2IrqEnR = crate::BitReader<Stateulps2IrqEn>;
impl Stateulps2IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stateulps2IrqEn {
        match self.bits {
            false => Stateulps2IrqEn::Disable,
            true => Stateulps2IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Stateulps2IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Stateulps2IrqEn::Enable
    }
}
#[doc = "Field `STATEULPS2_IRQ_EN` writer - 16:16\\]
Lane #2 in Ultra Low Power State"]
pub type Stateulps2IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Stateulps2IrqEn>;
impl<'a, REG> Stateulps2IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps2IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps2IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED10` reader - "]
pub type Reserved10R = crate::BitReader;
#[doc = "Field `RESERVED10` writer - "]
pub type Reserved10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "17:17\\]
Lane #3 in Ultra Low Power State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stateulps3IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Stateulps3IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Stateulps3IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATEULPS3_IRQ_EN` reader - 17:17\\]
Lane #3 in Ultra Low Power State"]
pub type Stateulps3IrqEnR = crate::BitReader<Stateulps3IrqEn>;
impl Stateulps3IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stateulps3IrqEn {
        match self.bits {
            false => Stateulps3IrqEn::Disable,
            true => Stateulps3IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Stateulps3IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Stateulps3IrqEn::Enable
    }
}
#[doc = "Field `STATEULPS3_IRQ_EN` writer - 17:17\\]
Lane #3 in Ultra Low Power State"]
pub type Stateulps3IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Stateulps3IrqEn>;
impl<'a, REG> Stateulps3IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps3IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps3IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED11` reader - "]
pub type Reserved11R = crate::BitReader;
#[doc = "Field `RESERVED11` writer - "]
pub type Reserved11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "18:18\\]
Lane #4 in Ultra Low Power State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stateulps4IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Stateulps4IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Stateulps4IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATEULPS4_IRQ_EN` reader - 18:18\\]
Lane #4 in Ultra Low Power State"]
pub type Stateulps4IrqEnR = crate::BitReader<Stateulps4IrqEn>;
impl Stateulps4IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stateulps4IrqEn {
        match self.bits {
            false => Stateulps4IrqEn::Disable,
            true => Stateulps4IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Stateulps4IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Stateulps4IrqEn::Enable
    }
}
#[doc = "Field `STATEULPS4_IRQ_EN` writer - 18:18\\]
Lane #4 in Ultra Low Power State"]
pub type Stateulps4IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Stateulps4IrqEn>;
impl<'a, REG> Stateulps4IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps4IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps4IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED12` reader - "]
pub type Reserved12R = crate::BitReader;
#[doc = "Field `RESERVED12` writer - "]
pub type Reserved12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "19:19\\]
Lane #5 in Ultra Low Power State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stateulps5IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Stateulps5IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Stateulps5IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATEULPS5_IRQ_EN` reader - 19:19\\]
Lane #5 in Ultra Low Power State"]
pub type Stateulps5IrqEnR = crate::BitReader<Stateulps5IrqEn>;
impl Stateulps5IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stateulps5IrqEn {
        match self.bits {
            false => Stateulps5IrqEn::Disable,
            true => Stateulps5IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Stateulps5IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Stateulps5IrqEn::Enable
    }
}
#[doc = "Field `STATEULPS5_IRQ_EN` writer - 19:19\\]
Lane #5 in Ultra Low Power State"]
pub type Stateulps5IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Stateulps5IrqEn>;
impl<'a, REG> Stateulps5IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps5IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Stateulps5IrqEn::Enable)
    }
}
#[doc = "20:20\\]
Contention LP0 error for lane #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp0_1IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errcontentionlp0_1IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp0_1IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP0_1_IRQ_EN` reader - 20:20\\]
Contention LP0 error for lane #1"]
pub type Errcontentionlp0_1IrqEnR = crate::BitReader<Errcontentionlp0_1IrqEn>;
impl Errcontentionlp0_1IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp0_1IrqEn {
        match self.bits {
            false => Errcontentionlp0_1IrqEn::Disable,
            true => Errcontentionlp0_1IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errcontentionlp0_1IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errcontentionlp0_1IrqEn::Enable
    }
}
#[doc = "Field `ERRCONTENTIONLP0_1_IRQ_EN` writer - 20:20\\]
Contention LP0 error for lane #1"]
pub type Errcontentionlp0_1IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp0_1IrqEn>;
impl<'a, REG> Errcontentionlp0_1IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_1IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_1IrqEn::Enable)
    }
}
#[doc = "21:21\\]
Contention LP1 error for lane #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp1_1IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errcontentionlp1_1IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp1_1IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP1_1_IRQ_EN` reader - 21:21\\]
Contention LP1 error for lane #1"]
pub type Errcontentionlp1_1IrqEnR = crate::BitReader<Errcontentionlp1_1IrqEn>;
impl Errcontentionlp1_1IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp1_1IrqEn {
        match self.bits {
            false => Errcontentionlp1_1IrqEn::Disable,
            true => Errcontentionlp1_1IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errcontentionlp1_1IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errcontentionlp1_1IrqEn::Enable
    }
}
#[doc = "Field `ERRCONTENTIONLP1_1_IRQ_EN` writer - 21:21\\]
Contention LP1 error for lane #1"]
pub type Errcontentionlp1_1IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp1_1IrqEn>;
impl<'a, REG> Errcontentionlp1_1IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_1IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_1IrqEn::Enable)
    }
}
#[doc = "22:22\\]
Contention LP0 error for lane #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp0_2IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errcontentionlp0_2IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp0_2IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP0_2_IRQ_EN` reader - 22:22\\]
Contention LP0 error for lane #2"]
pub type Errcontentionlp0_2IrqEnR = crate::BitReader<Errcontentionlp0_2IrqEn>;
impl Errcontentionlp0_2IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp0_2IrqEn {
        match self.bits {
            false => Errcontentionlp0_2IrqEn::Disable,
            true => Errcontentionlp0_2IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errcontentionlp0_2IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errcontentionlp0_2IrqEn::Enable
    }
}
#[doc = "Field `ERRCONTENTIONLP0_2_IRQ_EN` writer - 22:22\\]
Contention LP0 error for lane #2"]
pub type Errcontentionlp0_2IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp0_2IrqEn>;
impl<'a, REG> Errcontentionlp0_2IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_2IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_2IrqEn::Enable)
    }
}
#[doc = "23:23\\]
Contention LP1 error for lane #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp1_2IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errcontentionlp1_2IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp1_2IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP1_2_IRQ_EN` reader - 23:23\\]
Contention LP1 error for lane #2"]
pub type Errcontentionlp1_2IrqEnR = crate::BitReader<Errcontentionlp1_2IrqEn>;
impl Errcontentionlp1_2IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp1_2IrqEn {
        match self.bits {
            false => Errcontentionlp1_2IrqEn::Disable,
            true => Errcontentionlp1_2IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errcontentionlp1_2IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errcontentionlp1_2IrqEn::Enable
    }
}
#[doc = "Field `ERRCONTENTIONLP1_2_IRQ_EN` writer - 23:23\\]
Contention LP1 error for lane #2"]
pub type Errcontentionlp1_2IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp1_2IrqEn>;
impl<'a, REG> Errcontentionlp1_2IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_2IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_2IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED13` reader - "]
pub type Reserved13R = crate::BitReader;
#[doc = "Field `RESERVED13` writer - "]
pub type Reserved13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "24:24\\]
Contention LP0 error for lane #3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp0_3IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errcontentionlp0_3IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp0_3IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP0_3_IRQ_EN` reader - 24:24\\]
Contention LP0 error for lane #3"]
pub type Errcontentionlp0_3IrqEnR = crate::BitReader<Errcontentionlp0_3IrqEn>;
impl Errcontentionlp0_3IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp0_3IrqEn {
        match self.bits {
            false => Errcontentionlp0_3IrqEn::Disable,
            true => Errcontentionlp0_3IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errcontentionlp0_3IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errcontentionlp0_3IrqEn::Enable
    }
}
#[doc = "Field `ERRCONTENTIONLP0_3_IRQ_EN` writer - 24:24\\]
Contention LP0 error for lane #3"]
pub type Errcontentionlp0_3IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp0_3IrqEn>;
impl<'a, REG> Errcontentionlp0_3IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_3IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_3IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED14` reader - 25:25\\]
Write 0's for future compatibility. Reads returns 0."]
pub type Reserved14R = crate::BitReader;
#[doc = "Field `RESERVED14` writer - 25:25\\]
Write 0's for future compatibility. Reads returns 0."]
pub type Reserved14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "25:25\\]
Contention LP1 error for lane #3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp1_3IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errcontentionlp1_3IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp1_3IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP1_3_IRQ_EN` reader - 25:25\\]
Contention LP1 error for lane #3"]
pub type Errcontentionlp1_3IrqEnR = crate::BitReader<Errcontentionlp1_3IrqEn>;
impl Errcontentionlp1_3IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp1_3IrqEn {
        match self.bits {
            false => Errcontentionlp1_3IrqEn::Disable,
            true => Errcontentionlp1_3IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errcontentionlp1_3IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errcontentionlp1_3IrqEn::Enable
    }
}
#[doc = "Field `ERRCONTENTIONLP1_3_IRQ_EN` writer - 25:25\\]
Contention LP1 error for lane #3"]
pub type Errcontentionlp1_3IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp1_3IrqEn>;
impl<'a, REG> Errcontentionlp1_3IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_3IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_3IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED15` reader - 26:26\\]
Write 0's for future compatibility. Reads returns 0."]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `RESERVED15` writer - 26:26\\]
Write 0's for future compatibility. Reads returns 0."]
pub type Reserved15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "26:26\\]
Contention LP0 error for lane #4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp0_4IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errcontentionlp0_4IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp0_4IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP0_4_IRQ_EN` reader - 26:26\\]
Contention LP0 error for lane #4"]
pub type Errcontentionlp0_4IrqEnR = crate::BitReader<Errcontentionlp0_4IrqEn>;
impl Errcontentionlp0_4IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp0_4IrqEn {
        match self.bits {
            false => Errcontentionlp0_4IrqEn::Disable,
            true => Errcontentionlp0_4IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errcontentionlp0_4IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errcontentionlp0_4IrqEn::Enable
    }
}
#[doc = "Field `ERRCONTENTIONLP0_4_IRQ_EN` writer - 26:26\\]
Contention LP0 error for lane #4"]
pub type Errcontentionlp0_4IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp0_4IrqEn>;
impl<'a, REG> Errcontentionlp0_4IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_4IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_4IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED16` reader - 27:27\\]
Write 0's for future compatibility. Reads returns 0."]
pub type Reserved16R = crate::BitReader;
#[doc = "Field `RESERVED16` writer - 27:27\\]
Write 0's for future compatibility. Reads returns 0."]
pub type Reserved16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "27:27\\]
Contention LP1 error for lane #4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp1_4IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errcontentionlp1_4IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp1_4IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP1_4_IRQ_EN` reader - 27:27\\]
Contention LP1 error for lane #4"]
pub type Errcontentionlp1_4IrqEnR = crate::BitReader<Errcontentionlp1_4IrqEn>;
impl Errcontentionlp1_4IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp1_4IrqEn {
        match self.bits {
            false => Errcontentionlp1_4IrqEn::Disable,
            true => Errcontentionlp1_4IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errcontentionlp1_4IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errcontentionlp1_4IrqEn::Enable
    }
}
#[doc = "Field `ERRCONTENTIONLP1_4_IRQ_EN` writer - 27:27\\]
Contention LP1 error for lane #4"]
pub type Errcontentionlp1_4IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp1_4IrqEn>;
impl<'a, REG> Errcontentionlp1_4IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_4IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_4IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED17` reader - 28:28\\]
Write 0's for future compatibility. Reads returns 0."]
pub type Reserved17R = crate::BitReader;
#[doc = "Field `RESERVED17` writer - 28:28\\]
Write 0's for future compatibility. Reads returns 0."]
pub type Reserved17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "28:28\\]
Contention LP0 error for lane #5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp0_5IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errcontentionlp0_5IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp0_5IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP0_5_IRQ_EN` reader - 28:28\\]
Contention LP0 error for lane #5"]
pub type Errcontentionlp0_5IrqEnR = crate::BitReader<Errcontentionlp0_5IrqEn>;
impl Errcontentionlp0_5IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp0_5IrqEn {
        match self.bits {
            false => Errcontentionlp0_5IrqEn::Disable,
            true => Errcontentionlp0_5IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errcontentionlp0_5IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errcontentionlp0_5IrqEn::Enable
    }
}
#[doc = "Field `ERRCONTENTIONLP0_5_IRQ_EN` writer - 28:28\\]
Contention LP0 error for lane #5"]
pub type Errcontentionlp0_5IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp0_5IrqEn>;
impl<'a, REG> Errcontentionlp0_5IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_5IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp0_5IrqEn::Enable)
    }
}
#[doc = "29:29\\]
Contention LP1 error for lane #5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errcontentionlp1_5IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<Errcontentionlp1_5IrqEn> for bool {
    #[inline(always)]
    fn from(variant: Errcontentionlp1_5IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCONTENTIONLP1_5_IRQ_EN` reader - 29:29\\]
Contention LP1 error for lane #5"]
pub type Errcontentionlp1_5IrqEnR = crate::BitReader<Errcontentionlp1_5IrqEn>;
impl Errcontentionlp1_5IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errcontentionlp1_5IrqEn {
        match self.bits {
            false => Errcontentionlp1_5IrqEn::Disable,
            true => Errcontentionlp1_5IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errcontentionlp1_5IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errcontentionlp1_5IrqEn::Enable
    }
}
#[doc = "Field `ERRCONTENTIONLP1_5_IRQ_EN` writer - 29:29\\]
Contention LP1 error for lane #5"]
pub type Errcontentionlp1_5IrqEnW<'a, REG> = crate::BitWriter<'a, REG, Errcontentionlp1_5IrqEn>;
impl<'a, REG> Errcontentionlp1_5IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_5IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errcontentionlp1_5IrqEn::Enable)
    }
}
#[doc = "Field `RESERVED18` reader - 29:29\\]
Write 0's for future compatibility. Reads returns 0."]
pub type Reserved18R = crate::BitReader;
#[doc = "Field `RESERVED18` writer - 29:29\\]
Write 0's for future compatibility. Reads returns 0."]
pub type Reserved18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "30:30\\]
All signals ULPSActiveNOT are 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UlpsactivenotAll0IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<UlpsactivenotAll0IrqEn> for bool {
    #[inline(always)]
    fn from(variant: UlpsactivenotAll0IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPSACTIVENOT_ALL0_IRQ_EN` reader - 30:30\\]
All signals ULPSActiveNOT are 0"]
pub type UlpsactivenotAll0IrqEnR = crate::BitReader<UlpsactivenotAll0IrqEn>;
impl UlpsactivenotAll0IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UlpsactivenotAll0IrqEn {
        match self.bits {
            false => UlpsactivenotAll0IrqEn::Disable,
            true => UlpsactivenotAll0IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UlpsactivenotAll0IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UlpsactivenotAll0IrqEn::Enable
    }
}
#[doc = "Field `ULPSACTIVENOT_ALL0_IRQ_EN` writer - 30:30\\]
All signals ULPSActiveNOT are 0"]
pub type UlpsactivenotAll0IrqEnW<'a, REG> = crate::BitWriter<'a, REG, UlpsactivenotAll0IrqEn>;
impl<'a, REG> UlpsactivenotAll0IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(UlpsactivenotAll0IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(UlpsactivenotAll0IrqEn::Enable)
    }
}
#[doc = "31:31\\]
All the ULPSActiveNOT signals corresponding to the lanes with TXULPSExit being high are high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UlpsactivenotAll1IrqEn {
    #[doc = "0: Event is masked"]
    Disable = 0,
    #[doc = "1: Event generates an interrupt when it occurs"]
    Enable = 1,
}
impl From<UlpsactivenotAll1IrqEn> for bool {
    #[inline(always)]
    fn from(variant: UlpsactivenotAll1IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPSACTIVENOT_ALL1_IRQ_EN` reader - 31:31\\]
All the ULPSActiveNOT signals corresponding to the lanes with TXULPSExit being high are high."]
pub type UlpsactivenotAll1IrqEnR = crate::BitReader<UlpsactivenotAll1IrqEn>;
impl UlpsactivenotAll1IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UlpsactivenotAll1IrqEn {
        match self.bits {
            false => UlpsactivenotAll1IrqEn::Disable,
            true => UlpsactivenotAll1IrqEn::Enable,
        }
    }
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UlpsactivenotAll1IrqEn::Disable
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UlpsactivenotAll1IrqEn::Enable
    }
}
#[doc = "Field `ULPSACTIVENOT_ALL1_IRQ_EN` writer - 31:31\\]
All the ULPSActiveNOT signals corresponding to the lanes with TXULPSExit being high are high."]
pub type UlpsactivenotAll1IrqEnW<'a, REG> = crate::BitWriter<'a, REG, UlpsactivenotAll1IrqEn>;
impl<'a, REG> UlpsactivenotAll1IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is masked"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(UlpsactivenotAll1IrqEn::Disable)
    }
    #[doc = "Event generates an interrupt when it occurs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(UlpsactivenotAll1IrqEn::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Low power Data transmission synchronization error for lane #1"]
    #[inline(always)]
    pub fn errsyncsesc1_irq_en(&self) -> Errsyncsesc1IrqEnR {
        Errsyncsesc1IrqEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Low power Data transmission synchronization error for lane #2"]
    #[inline(always)]
    pub fn errsyncsesc2_irq_en(&self) -> Errsyncsesc2IrqEnR {
        Errsyncsesc2IrqEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Low power Data transmission synchronization error for lane #3"]
    #[inline(always)]
    pub fn errsyncsesc3_irq_en(&self) -> Errsyncsesc3IrqEnR {
        Errsyncsesc3IrqEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Low power Data transmission synchronization error for lane #4"]
    #[inline(always)]
    pub fn errsyncsesc4_irq_en(&self) -> Errsyncsesc4IrqEnR {
        Errsyncsesc4IrqEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Low power Data transmission synchronization error for lane #5"]
    #[inline(always)]
    pub fn errsyncsesc5_irq_en(&self) -> Errsyncsesc5IrqEnR {
        Errsyncsesc5IrqEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Escape entry error for lane #1"]
    #[inline(always)]
    pub fn erresc1_irq_en(&self) -> Erresc1IrqEnR {
        Erresc1IrqEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Escape entry error for lane #2"]
    #[inline(always)]
    pub fn erresc2_irq_en(&self) -> Erresc2IrqEnR {
        Erresc2IrqEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Escape entry error for lane #3"]
    #[inline(always)]
    pub fn erresc3_irq_en(&self) -> Erresc3IrqEnR {
        Erresc3IrqEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Escape entry error for lane #4"]
    #[inline(always)]
    pub fn erresc4_irq_en(&self) -> Erresc4IrqEnR {
        Erresc4IrqEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Escape entry error for lane #5"]
    #[inline(always)]
    pub fn erresc5_irq_en(&self) -> Erresc5IrqEnR {
        Erresc5IrqEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Control error for lane #1"]
    #[inline(always)]
    pub fn errcontrol1_irq_en(&self) -> Errcontrol1IrqEnR {
        Errcontrol1IrqEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Control error for lane #2"]
    #[inline(always)]
    pub fn errcontrol2_irq_en(&self) -> Errcontrol2IrqEnR {
        Errcontrol2IrqEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Control error for lane #3"]
    #[inline(always)]
    pub fn errcontrol3_irq_en(&self) -> Errcontrol3IrqEnR {
        Errcontrol3IrqEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Control error for lane #4"]
    #[inline(always)]
    pub fn errcontrol4_irq_en(&self) -> Errcontrol4IrqEnR {
        Errcontrol4IrqEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Control error for lane #5"]
    #[inline(always)]
    pub fn errcontrol5_irq_en(&self) -> Errcontrol5IrqEnR {
        Errcontrol5IrqEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Lane #1 in Ultra Low Power State"]
    #[inline(always)]
    pub fn stateulps1_irq_en(&self) -> Stateulps1IrqEnR {
        Stateulps1IrqEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Lane #2 in Ultra Low Power State"]
    #[inline(always)]
    pub fn stateulps2_irq_en(&self) -> Stateulps2IrqEnR {
        Stateulps2IrqEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Lane #3 in Ultra Low Power State"]
    #[inline(always)]
    pub fn stateulps3_irq_en(&self) -> Stateulps3IrqEnR {
        Stateulps3IrqEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Lane #4 in Ultra Low Power State"]
    #[inline(always)]
    pub fn stateulps4_irq_en(&self) -> Stateulps4IrqEnR {
        Stateulps4IrqEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Lane #5 in Ultra Low Power State"]
    #[inline(always)]
    pub fn stateulps5_irq_en(&self) -> Stateulps5IrqEnR {
        Stateulps5IrqEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Contention LP0 error for lane #1"]
    #[inline(always)]
    pub fn errcontentionlp0_1_irq_en(&self) -> Errcontentionlp0_1IrqEnR {
        Errcontentionlp0_1IrqEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Contention LP1 error for lane #1"]
    #[inline(always)]
    pub fn errcontentionlp1_1_irq_en(&self) -> Errcontentionlp1_1IrqEnR {
        Errcontentionlp1_1IrqEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Contention LP0 error for lane #2"]
    #[inline(always)]
    pub fn errcontentionlp0_2_irq_en(&self) -> Errcontentionlp0_2IrqEnR {
        Errcontentionlp0_2IrqEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Contention LP1 error for lane #2"]
    #[inline(always)]
    pub fn errcontentionlp1_2_irq_en(&self) -> Errcontentionlp1_2IrqEnR {
        Errcontentionlp1_2IrqEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Contention LP0 error for lane #3"]
    #[inline(always)]
    pub fn errcontentionlp0_3_irq_en(&self) -> Errcontentionlp0_3IrqEnR {
        Errcontentionlp0_3IrqEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Write 0's for future compatibility. Reads returns 0."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Contention LP1 error for lane #3"]
    #[inline(always)]
    pub fn errcontentionlp1_3_irq_en(&self) -> Errcontentionlp1_3IrqEnR {
        Errcontentionlp1_3IrqEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Write 0's for future compatibility. Reads returns 0."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Contention LP0 error for lane #4"]
    #[inline(always)]
    pub fn errcontentionlp0_4_irq_en(&self) -> Errcontentionlp0_4IrqEnR {
        Errcontentionlp0_4IrqEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Write 0's for future compatibility. Reads returns 0."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Contention LP1 error for lane #4"]
    #[inline(always)]
    pub fn errcontentionlp1_4_irq_en(&self) -> Errcontentionlp1_4IrqEnR {
        Errcontentionlp1_4IrqEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Write 0's for future compatibility. Reads returns 0."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Contention LP0 error for lane #5"]
    #[inline(always)]
    pub fn errcontentionlp0_5_irq_en(&self) -> Errcontentionlp0_5IrqEnR {
        Errcontentionlp0_5IrqEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Contention LP1 error for lane #5"]
    #[inline(always)]
    pub fn errcontentionlp1_5_irq_en(&self) -> Errcontentionlp1_5IrqEnR {
        Errcontentionlp1_5IrqEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Write 0's for future compatibility. Reads returns 0."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
All signals ULPSActiveNOT are 0"]
    #[inline(always)]
    pub fn ulpsactivenot_all0_irq_en(&self) -> UlpsactivenotAll0IrqEnR {
        UlpsactivenotAll0IrqEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
All the ULPSActiveNOT signals corresponding to the lanes with TXULPSExit being high are high."]
    #[inline(always)]
    pub fn ulpsactivenot_all1_irq_en(&self) -> UlpsactivenotAll1IrqEnR {
        UlpsactivenotAll1IrqEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Low power Data transmission synchronization error for lane #1"]
    #[inline(always)]
    #[must_use]
    pub fn errsyncsesc1_irq_en(&mut self) -> Errsyncsesc1IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errsyncsesc1IrqEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Low power Data transmission synchronization error for lane #2"]
    #[inline(always)]
    #[must_use]
    pub fn errsyncsesc2_irq_en(&mut self) -> Errsyncsesc2IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errsyncsesc2IrqEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Csi2ComplexioIrqenableSpec> {
        Reserved1W::new(self, 2)
    }
    #[doc = "Bit 2 - 2:2\\]
Low power Data transmission synchronization error for lane #3"]
    #[inline(always)]
    #[must_use]
    pub fn errsyncsesc3_irq_en(&mut self) -> Errsyncsesc3IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errsyncsesc3IrqEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Csi2ComplexioIrqenableSpec> {
        Reserved2W::new(self, 3)
    }
    #[doc = "Bit 3 - 3:3\\]
Low power Data transmission synchronization error for lane #4"]
    #[inline(always)]
    #[must_use]
    pub fn errsyncsesc4_irq_en(&mut self) -> Errsyncsesc4IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errsyncsesc4IrqEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Csi2ComplexioIrqenableSpec> {
        Reserved3W::new(self, 4)
    }
    #[doc = "Bit 4 - 4:4\\]
Low power Data transmission synchronization error for lane #5"]
    #[inline(always)]
    #[must_use]
    pub fn errsyncsesc5_irq_en(&mut self) -> Errsyncsesc5IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errsyncsesc5IrqEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Escape entry error for lane #1"]
    #[inline(always)]
    #[must_use]
    pub fn erresc1_irq_en(&mut self) -> Erresc1IrqEnW<Csi2ComplexioIrqenableSpec> {
        Erresc1IrqEnW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Escape entry error for lane #2"]
    #[inline(always)]
    #[must_use]
    pub fn erresc2_irq_en(&mut self) -> Erresc2IrqEnW<Csi2ComplexioIrqenableSpec> {
        Erresc2IrqEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<Csi2ComplexioIrqenableSpec> {
        Reserved4W::new(self, 7)
    }
    #[doc = "Bit 7 - 7:7\\]
Escape entry error for lane #3"]
    #[inline(always)]
    #[must_use]
    pub fn erresc3_irq_en(&mut self) -> Erresc3IrqEnW<Csi2ComplexioIrqenableSpec> {
        Erresc3IrqEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<Csi2ComplexioIrqenableSpec> {
        Reserved5W::new(self, 8)
    }
    #[doc = "Bit 8 - 8:8\\]
Escape entry error for lane #4"]
    #[inline(always)]
    #[must_use]
    pub fn erresc4_irq_en(&mut self) -> Erresc4IrqEnW<Csi2ComplexioIrqenableSpec> {
        Erresc4IrqEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<Csi2ComplexioIrqenableSpec> {
        Reserved6W::new(self, 9)
    }
    #[doc = "Bit 9 - 9:9\\]
Escape entry error for lane #5"]
    #[inline(always)]
    #[must_use]
    pub fn erresc5_irq_en(&mut self) -> Erresc5IrqEnW<Csi2ComplexioIrqenableSpec> {
        Erresc5IrqEnW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Control error for lane #1"]
    #[inline(always)]
    #[must_use]
    pub fn errcontrol1_irq_en(&mut self) -> Errcontrol1IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errcontrol1IrqEnW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Control error for lane #2"]
    #[inline(always)]
    #[must_use]
    pub fn errcontrol2_irq_en(&mut self) -> Errcontrol2IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errcontrol2IrqEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<Csi2ComplexioIrqenableSpec> {
        Reserved7W::new(self, 12)
    }
    #[doc = "Bit 12 - 12:12\\]
Control error for lane #3"]
    #[inline(always)]
    #[must_use]
    pub fn errcontrol3_irq_en(&mut self) -> Errcontrol3IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errcontrol3IrqEnW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Csi2ComplexioIrqenableSpec> {
        Reserved8W::new(self, 13)
    }
    #[doc = "Bit 13 - 13:13\\]
Control error for lane #4"]
    #[inline(always)]
    #[must_use]
    pub fn errcontrol4_irq_en(&mut self) -> Errcontrol4IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errcontrol4IrqEnW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<Csi2ComplexioIrqenableSpec> {
        Reserved9W::new(self, 14)
    }
    #[doc = "Bit 14 - 14:14\\]
Control error for lane #5"]
    #[inline(always)]
    #[must_use]
    pub fn errcontrol5_irq_en(&mut self) -> Errcontrol5IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errcontrol5IrqEnW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Lane #1 in Ultra Low Power State"]
    #[inline(always)]
    #[must_use]
    pub fn stateulps1_irq_en(&mut self) -> Stateulps1IrqEnW<Csi2ComplexioIrqenableSpec> {
        Stateulps1IrqEnW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Lane #2 in Ultra Low Power State"]
    #[inline(always)]
    #[must_use]
    pub fn stateulps2_irq_en(&mut self) -> Stateulps2IrqEnW<Csi2ComplexioIrqenableSpec> {
        Stateulps2IrqEnW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<Csi2ComplexioIrqenableSpec> {
        Reserved10W::new(self, 17)
    }
    #[doc = "Bit 17 - 17:17\\]
Lane #3 in Ultra Low Power State"]
    #[inline(always)]
    #[must_use]
    pub fn stateulps3_irq_en(&mut self) -> Stateulps3IrqEnW<Csi2ComplexioIrqenableSpec> {
        Stateulps3IrqEnW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<Csi2ComplexioIrqenableSpec> {
        Reserved11W::new(self, 18)
    }
    #[doc = "Bit 18 - 18:18\\]
Lane #4 in Ultra Low Power State"]
    #[inline(always)]
    #[must_use]
    pub fn stateulps4_irq_en(&mut self) -> Stateulps4IrqEnW<Csi2ComplexioIrqenableSpec> {
        Stateulps4IrqEnW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<Csi2ComplexioIrqenableSpec> {
        Reserved12W::new(self, 19)
    }
    #[doc = "Bit 19 - 19:19\\]
Lane #5 in Ultra Low Power State"]
    #[inline(always)]
    #[must_use]
    pub fn stateulps5_irq_en(&mut self) -> Stateulps5IrqEnW<Csi2ComplexioIrqenableSpec> {
        Stateulps5IrqEnW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Contention LP0 error for lane #1"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp0_1_irq_en(
        &mut self,
    ) -> Errcontentionlp0_1IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errcontentionlp0_1IrqEnW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Contention LP1 error for lane #1"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp1_1_irq_en(
        &mut self,
    ) -> Errcontentionlp1_1IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errcontentionlp1_1IrqEnW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Contention LP0 error for lane #2"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp0_2_irq_en(
        &mut self,
    ) -> Errcontentionlp0_2IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errcontentionlp0_2IrqEnW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Contention LP1 error for lane #2"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp1_2_irq_en(
        &mut self,
    ) -> Errcontentionlp1_2IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errcontentionlp1_2IrqEnW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> Reserved13W<Csi2ComplexioIrqenableSpec> {
        Reserved13W::new(self, 24)
    }
    #[doc = "Bit 24 - 24:24\\]
Contention LP0 error for lane #3"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp0_3_irq_en(
        &mut self,
    ) -> Errcontentionlp0_3IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errcontentionlp0_3IrqEnW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Write 0's for future compatibility. Reads returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<Csi2ComplexioIrqenableSpec> {
        Reserved14W::new(self, 25)
    }
    #[doc = "Bit 25 - 25:25\\]
Contention LP1 error for lane #3"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp1_3_irq_en(
        &mut self,
    ) -> Errcontentionlp1_3IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errcontentionlp1_3IrqEnW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Write 0's for future compatibility. Reads returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<Csi2ComplexioIrqenableSpec> {
        Reserved15W::new(self, 26)
    }
    #[doc = "Bit 26 - 26:26\\]
Contention LP0 error for lane #4"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp0_4_irq_en(
        &mut self,
    ) -> Errcontentionlp0_4IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errcontentionlp0_4IrqEnW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Write 0's for future compatibility. Reads returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Csi2ComplexioIrqenableSpec> {
        Reserved16W::new(self, 27)
    }
    #[doc = "Bit 27 - 27:27\\]
Contention LP1 error for lane #4"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp1_4_irq_en(
        &mut self,
    ) -> Errcontentionlp1_4IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errcontentionlp1_4IrqEnW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Write 0's for future compatibility. Reads returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<Csi2ComplexioIrqenableSpec> {
        Reserved17W::new(self, 28)
    }
    #[doc = "Bit 28 - 28:28\\]
Contention LP0 error for lane #5"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp0_5_irq_en(
        &mut self,
    ) -> Errcontentionlp0_5IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errcontentionlp0_5IrqEnW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Contention LP1 error for lane #5"]
    #[inline(always)]
    #[must_use]
    pub fn errcontentionlp1_5_irq_en(
        &mut self,
    ) -> Errcontentionlp1_5IrqEnW<Csi2ComplexioIrqenableSpec> {
        Errcontentionlp1_5IrqEnW::new(self, 29)
    }
    #[doc = "Bit 29 - 29:29\\]
Write 0's for future compatibility. Reads returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> Reserved18W<Csi2ComplexioIrqenableSpec> {
        Reserved18W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
All signals ULPSActiveNOT are 0"]
    #[inline(always)]
    #[must_use]
    pub fn ulpsactivenot_all0_irq_en(
        &mut self,
    ) -> UlpsactivenotAll0IrqEnW<Csi2ComplexioIrqenableSpec> {
        UlpsactivenotAll0IrqEnW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
All the ULPSActiveNOT signals corresponding to the lanes with TXULPSExit being high are high."]
    #[inline(always)]
    #[must_use]
    pub fn ulpsactivenot_all1_irq_en(
        &mut self,
    ) -> UlpsactivenotAll1IrqEnW<Csi2ComplexioIrqenableSpec> {
        UlpsactivenotAll1IrqEnW::new(self, 31)
    }
}
#[doc = "INTERRUPT ENABLE REGISTER - All errors from complex IO\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_complexio_irqenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_complexio_irqenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2ComplexioIrqenableSpec;
impl crate::RegisterSpec for Csi2ComplexioIrqenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_complexio_irqenable::R`](R) reader structure"]
impl crate::Readable for Csi2ComplexioIrqenableSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_complexio_irqenable::W`](W) writer structure"]
impl crate::Writable for Csi2ComplexioIrqenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_COMPLEXIO_IRQENABLE to value 0"]
impl crate::Resettable for Csi2ComplexioIrqenableSpec {
    const RESET_VALUE: u32 = 0;
}
