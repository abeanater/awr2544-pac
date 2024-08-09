#[doc = "Register `ACTIRQ` reader"]
pub type R = crate::R<ActirqSpec>;
#[doc = "Register `ACTIRQ` writer"]
pub type W = crate::W<ActirqSpec>;
#[doc = "Field `NUM` reader - 9:0\\]
Number of the currently active IRQ. Loaded from teh Prioritized IRQ Register whenever the IRQ Vector Address is read. Valid only if the valid flag is set."]
pub type NumR = crate::FieldReader<u16>;
#[doc = "Field `NUM` writer - 9:0\\]
Number of the currently active IRQ. Loaded from teh Prioritized IRQ Register whenever the IRQ Vector Address is read. Valid only if the valid flag is set."]
pub type NumW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RES7` reader - 15:10\\]
RESERVE FIELD"]
pub type Res7R = crate::FieldReader;
#[doc = "Field `RES7` writer - 15:10\\]
RESERVE FIELD"]
pub type Res7W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PRI` reader - 19:16\\]
Priority of the highest priority pending IRQ. valid only if the valid flag is set."]
pub type PriR = crate::FieldReader;
#[doc = "Field `PRI` writer - 19:16\\]
Priority of the highest priority pending IRQ. valid only if the valid flag is set."]
pub type PriW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES6` reader - 30:20\\]
RESERVE FIELD"]
pub type Res6R = crate::FieldReader<u16>;
#[doc = "Field `RES6` writer - 30:20\\]
RESERVE FIELD"]
pub type Res6W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `VALID` reader - 31:31\\]
Indicates that the num field is valid. Set when the IRQ Vector Address Register is read and cleared whenever the IRQ Vector Address Register is written."]
pub type ValidR = crate::BitReader;
#[doc = "Field `VALID` writer - 31:31\\]
Indicates that the num field is valid. Set when the IRQ Vector Address Register is read and cleared whenever the IRQ Vector Address Register is written."]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Number of the currently active IRQ. Loaded from teh Prioritized IRQ Register whenever the IRQ Vector Address is read. Valid only if the valid flag is set."]
    #[inline(always)]
    pub fn num(&self) -> NumR {
        NumR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - 15:10\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res7(&self) -> Res7R {
        Res7R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Priority of the highest priority pending IRQ. valid only if the valid flag is set."]
    #[inline(always)]
    pub fn pri(&self) -> PriR {
        PriR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:30 - 30:20\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res6(&self) -> Res6R {
        Res6R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Indicates that the num field is valid. Set when the IRQ Vector Address Register is read and cleared whenever the IRQ Vector Address Register is written."]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Number of the currently active IRQ. Loaded from teh Prioritized IRQ Register whenever the IRQ Vector Address is read. Valid only if the valid flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn num(&mut self) -> NumW<ActirqSpec> {
        NumW::new(self, 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res7(&mut self) -> Res7W<ActirqSpec> {
        Res7W::new(self, 10)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Priority of the highest priority pending IRQ. valid only if the valid flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PriW<ActirqSpec> {
        PriW::new(self, 16)
    }
    #[doc = "Bits 20:30 - 30:20\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res6(&mut self) -> Res6W<ActirqSpec> {
        Res6W::new(self, 20)
    }
    #[doc = "Bit 31 - 31:31\\]
Indicates that the num field is valid. Set when the IRQ Vector Address Register is read and cleared whenever the IRQ Vector Address Register is written."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<ActirqSpec> {
        ValidW::new(self, 31)
    }
}
#[doc = "The Active IRQ Register shows the number of the currently active IRQ.\n\nYou can [`read`](crate::Reg::read) this register and get [`actirq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actirq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActirqSpec;
impl crate::RegisterSpec for ActirqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actirq::R`](R) reader structure"]
impl crate::Readable for ActirqSpec {}
#[doc = "`write(|w| ..)` method takes [`actirq::W`](W) writer structure"]
impl crate::Writable for ActirqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACTIRQ to value 0"]
impl crate::Resettable for ActirqSpec {
    const RESET_VALUE: u32 = 0;
}
