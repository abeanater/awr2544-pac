#[doc = "Register `CSI2_SYSCONFIG` reader"]
pub type R = crate::R<Csi2SysconfigSpec>;
#[doc = "Register `CSI2_SYSCONFIG` writer"]
pub type W = crate::W<Csi2SysconfigSpec>;
#[doc = "0:0\\]
Internal OCP gating strategy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoIdle {
    #[doc = "0: OCP clock is free-running."]
    Free = 0,
    #[doc = "1: Automatic OCP clock gating strategy is applied based on the OCP interface activity."]
    Gated = 1,
}
impl From<AutoIdle> for bool {
    #[inline(always)]
    fn from(variant: AutoIdle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTO_IDLE` reader - 0:0\\]
Internal OCP gating strategy"]
pub type AutoIdleR = crate::BitReader<AutoIdle>;
impl AutoIdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutoIdle {
        match self.bits {
            false => AutoIdle::Free,
            true => AutoIdle::Gated,
        }
    }
    #[doc = "OCP clock is free-running."]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == AutoIdle::Free
    }
    #[doc = "Automatic OCP clock gating strategy is applied based on the OCP interface activity."]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == AutoIdle::Gated
    }
}
#[doc = "Field `AUTO_IDLE` writer - 0:0\\]
Internal OCP gating strategy"]
pub type AutoIdleW<'a, REG> = crate::BitWriter<'a, REG, AutoIdle>;
impl<'a, REG> AutoIdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OCP clock is free-running."]
    #[inline(always)]
    pub fn free(self) -> &'a mut crate::W<REG> {
        self.variant(AutoIdle::Free)
    }
    #[doc = "Automatic OCP clock gating strategy is applied based on the OCP interface activity."]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(AutoIdle::Gated)
    }
}
#[doc = "1:1\\]
Software reset. Set the bit to 1 to trigger a module reset. The bit is automatically reset by the hw. During reads return 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoftReset {
    #[doc = "0: Normal mode."]
    Normal = 0,
    #[doc = "1: The module is reset"]
    Reset = 1,
}
impl From<SoftReset> for bool {
    #[inline(always)]
    fn from(variant: SoftReset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFT_RESET` reader - 1:1\\]
Software reset. Set the bit to 1 to trigger a module reset. The bit is automatically reset by the hw. During reads return 0."]
pub type SoftResetR = crate::BitReader<SoftReset>;
impl SoftResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SoftReset {
        match self.bits {
            false => SoftReset::Normal,
            true => SoftReset::Reset,
        }
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SoftReset::Normal
    }
    #[doc = "The module is reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SoftReset::Reset
    }
}
#[doc = "Field `SOFT_RESET` writer - 1:1\\]
Software reset. Set the bit to 1 to trigger a module reset. The bit is automatically reset by the hw. During reads return 0."]
pub type SoftResetW<'a, REG> = crate::BitWriter<'a, REG, SoftReset>;
impl<'a, REG> SoftResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SoftReset::Normal)
    }
    #[doc = "The module is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SoftReset::Reset)
    }
}
#[doc = "2:2\\]
Wake-up mode enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enwakeup {
    #[doc = "0: Wakeup is disabled"]
    Wakeupdis = 0,
    #[doc = "1: Wakeup is enabled"]
    Wakeupenb = 1,
}
impl From<Enwakeup> for bool {
    #[inline(always)]
    fn from(variant: Enwakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENWAKEUP` reader - 2:2\\]
Wake-up mode enable bit"]
pub type EnwakeupR = crate::BitReader<Enwakeup>;
impl EnwakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enwakeup {
        match self.bits {
            false => Enwakeup::Wakeupdis,
            true => Enwakeup::Wakeupenb,
        }
    }
    #[doc = "Wakeup is disabled"]
    #[inline(always)]
    pub fn is_wakeupdis(&self) -> bool {
        *self == Enwakeup::Wakeupdis
    }
    #[doc = "Wakeup is enabled"]
    #[inline(always)]
    pub fn is_wakeupenb(&self) -> bool {
        *self == Enwakeup::Wakeupenb
    }
}
#[doc = "Field `ENWAKEUP` writer - 2:2\\]
Wake-up mode enable bit"]
pub type EnwakeupW<'a, REG> = crate::BitWriter<'a, REG, Enwakeup>;
impl<'a, REG> EnwakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup is disabled"]
    #[inline(always)]
    pub fn wakeupdis(self) -> &'a mut crate::W<REG> {
        self.variant(Enwakeup::Wakeupdis)
    }
    #[doc = "Wakeup is enabled"]
    #[inline(always)]
    pub fn wakeupenb(self) -> &'a mut crate::W<REG> {
        self.variant(Enwakeup::Wakeupenb)
    }
}
#[doc = "4:3\\]
Slave interface power management, Idle req/ack control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sidlemode {
    #[doc = "0: Force-idle. An idle request is acknowledged unconditionally"]
    Fidle = 0,
    #[doc = "1: No-idle. An idle request is never acknowledged"]
    Nidle = 1,
    #[doc = "2: Smart-idle. Acknowledgement to an idle request is given based on the internal activity of the module."]
    Sidle = 2,
    #[doc = "3: Reserved"]
    Res = 3,
}
impl From<Sidlemode> for u8 {
    #[inline(always)]
    fn from(variant: Sidlemode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sidlemode {
    type Ux = u8;
}
impl crate::IsEnum for Sidlemode {}
#[doc = "Field `SIDLEMODE` reader - 4:3\\]
Slave interface power management, Idle req/ack control"]
pub type SidlemodeR = crate::FieldReader<Sidlemode>;
impl SidlemodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sidlemode {
        match self.bits {
            0 => Sidlemode::Fidle,
            1 => Sidlemode::Nidle,
            2 => Sidlemode::Sidle,
            3 => Sidlemode::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Force-idle. An idle request is acknowledged unconditionally"]
    #[inline(always)]
    pub fn is_fidle(&self) -> bool {
        *self == Sidlemode::Fidle
    }
    #[doc = "No-idle. An idle request is never acknowledged"]
    #[inline(always)]
    pub fn is_nidle(&self) -> bool {
        *self == Sidlemode::Nidle
    }
    #[doc = "Smart-idle. Acknowledgement to an idle request is given based on the internal activity of the module."]
    #[inline(always)]
    pub fn is_sidle(&self) -> bool {
        *self == Sidlemode::Sidle
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Sidlemode::Res
    }
}
#[doc = "Field `SIDLEMODE` writer - 4:3\\]
Slave interface power management, Idle req/ack control"]
pub type SidlemodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sidlemode, crate::Safe>;
impl<'a, REG> SidlemodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Force-idle. An idle request is acknowledged unconditionally"]
    #[inline(always)]
    pub fn fidle(self) -> &'a mut crate::W<REG> {
        self.variant(Sidlemode::Fidle)
    }
    #[doc = "No-idle. An idle request is never acknowledged"]
    #[inline(always)]
    pub fn nidle(self) -> &'a mut crate::W<REG> {
        self.variant(Sidlemode::Nidle)
    }
    #[doc = "Smart-idle. Acknowledgement to an idle request is given based on the internal activity of the module."]
    #[inline(always)]
    pub fn sidle(self) -> &'a mut crate::W<REG> {
        self.variant(Sidlemode::Sidle)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn res(self) -> &'a mut crate::W<REG> {
        self.variant(Sidlemode::Res)
    }
}
#[doc = "9:8\\]
Clocks activity during wake up mode period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clockactivity {
    #[doc = "0: OCP and Functional clocks can be switched off"]
    Ocpfuncoff = 0,
    #[doc = "1: Functional clocks can be switched off and OCP clocks are maintained during wake up period"]
    Funcoff = 1,
    #[doc = "2: OCP clocks can be switched off and Functional clocks are maintained during wake up period"]
    Ocpoff = 2,
    #[doc = "3: OCP and Functional clocks are maintained during wake up period"]
    Ocpfuncon = 3,
}
impl From<Clockactivity> for u8 {
    #[inline(always)]
    fn from(variant: Clockactivity) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clockactivity {
    type Ux = u8;
}
impl crate::IsEnum for Clockactivity {}
#[doc = "Field `CLOCKACTIVITY` reader - 9:8\\]
Clocks activity during wake up mode period"]
pub type ClockactivityR = crate::FieldReader<Clockactivity>;
impl ClockactivityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clockactivity {
        match self.bits {
            0 => Clockactivity::Ocpfuncoff,
            1 => Clockactivity::Funcoff,
            2 => Clockactivity::Ocpoff,
            3 => Clockactivity::Ocpfuncon,
            _ => unreachable!(),
        }
    }
    #[doc = "OCP and Functional clocks can be switched off"]
    #[inline(always)]
    pub fn is_ocpfuncoff(&self) -> bool {
        *self == Clockactivity::Ocpfuncoff
    }
    #[doc = "Functional clocks can be switched off and OCP clocks are maintained during wake up period"]
    #[inline(always)]
    pub fn is_funcoff(&self) -> bool {
        *self == Clockactivity::Funcoff
    }
    #[doc = "OCP clocks can be switched off and Functional clocks are maintained during wake up period"]
    #[inline(always)]
    pub fn is_ocpoff(&self) -> bool {
        *self == Clockactivity::Ocpoff
    }
    #[doc = "OCP and Functional clocks are maintained during wake up period"]
    #[inline(always)]
    pub fn is_ocpfuncon(&self) -> bool {
        *self == Clockactivity::Ocpfuncon
    }
}
#[doc = "Field `CLOCKACTIVITY` writer - 9:8\\]
Clocks activity during wake up mode period"]
pub type ClockactivityW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clockactivity, crate::Safe>;
impl<'a, REG> ClockactivityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OCP and Functional clocks can be switched off"]
    #[inline(always)]
    pub fn ocpfuncoff(self) -> &'a mut crate::W<REG> {
        self.variant(Clockactivity::Ocpfuncoff)
    }
    #[doc = "Functional clocks can be switched off and OCP clocks are maintained during wake up period"]
    #[inline(always)]
    pub fn funcoff(self) -> &'a mut crate::W<REG> {
        self.variant(Clockactivity::Funcoff)
    }
    #[doc = "OCP clocks can be switched off and Functional clocks are maintained during wake up period"]
    #[inline(always)]
    pub fn ocpoff(self) -> &'a mut crate::W<REG> {
        self.variant(Clockactivity::Ocpoff)
    }
    #[doc = "OCP and Functional clocks are maintained during wake up period"]
    #[inline(always)]
    pub fn ocpfuncon(self) -> &'a mut crate::W<REG> {
        self.variant(Clockactivity::Ocpfuncon)
    }
}
#[doc = "Field `RESERVED1` reader - 31:10\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:10\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal OCP gating strategy"]
    #[inline(always)]
    pub fn auto_idle(&self) -> AutoIdleR {
        AutoIdleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software reset. Set the bit to 1 to trigger a module reset. The bit is automatically reset by the hw. During reads return 0."]
    #[inline(always)]
    pub fn soft_reset(&self) -> SoftResetR {
        SoftResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Wake-up mode enable bit"]
    #[inline(always)]
    pub fn enwakeup(&self) -> EnwakeupR {
        EnwakeupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Slave interface power management, Idle req/ack control"]
    #[inline(always)]
    pub fn sidlemode(&self) -> SidlemodeR {
        SidlemodeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Clocks activity during wake up mode period"]
    #[inline(always)]
    pub fn clockactivity(&self) -> ClockactivityR {
        ClockactivityR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal OCP gating strategy"]
    #[inline(always)]
    #[must_use]
    pub fn auto_idle(&mut self) -> AutoIdleW<Csi2SysconfigSpec> {
        AutoIdleW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software reset. Set the bit to 1 to trigger a module reset. The bit is automatically reset by the hw. During reads return 0."]
    #[inline(always)]
    #[must_use]
    pub fn soft_reset(&mut self) -> SoftResetW<Csi2SysconfigSpec> {
        SoftResetW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Wake-up mode enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn enwakeup(&mut self) -> EnwakeupW<Csi2SysconfigSpec> {
        EnwakeupW::new(self, 2)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Slave interface power management, Idle req/ack control"]
    #[inline(always)]
    #[must_use]
    pub fn sidlemode(&mut self) -> SidlemodeW<Csi2SysconfigSpec> {
        SidlemodeW::new(self, 3)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Clocks activity during wake up mode period"]
    #[inline(always)]
    #[must_use]
    pub fn clockactivity(&mut self) -> ClockactivityW<Csi2SysconfigSpec> {
        ClockactivityW::new(self, 8)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Csi2SysconfigSpec> {
        Reserved1W::new(self, 10)
    }
}
#[doc = "SYSTEM CONFIGURATION REGISTER This register is the OCP-socket system configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_sysconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_sysconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2SysconfigSpec;
impl crate::RegisterSpec for Csi2SysconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_sysconfig::R`](R) reader structure"]
impl crate::Readable for Csi2SysconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_sysconfig::W`](W) writer structure"]
impl crate::Writable for Csi2SysconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_SYSCONFIG to value 0"]
impl crate::Resettable for Csi2SysconfigSpec {
    const RESET_VALUE: u32 = 0;
}
