#[doc = "Register `PRIIRQ` reader"]
pub type R = crate::R<PriirqSpec>;
#[doc = "Register `PRIIRQ` writer"]
pub type W = crate::W<PriirqSpec>;
#[doc = "Field `NUM` reader - 9:0\\]
Number of the highest priority pending IRQ. valid only if the valid flag is set."]
pub type NumR = crate::FieldReader<u16>;
#[doc = "Field `NUM` writer - 9:0\\]
Number of the highest priority pending IRQ. valid only if the valid flag is set."]
pub type NumW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RES3` reader - 15:10\\]
RESERVE FIELD"]
pub type Res3R = crate::FieldReader;
#[doc = "Field `RES3` writer - 15:10\\]
RESERVE FIELD"]
pub type Res3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PRI` reader - 19:16\\]
Priority of the highest priority pending IRQ. valid only if the valid flag is set."]
pub type PriR = crate::FieldReader;
#[doc = "Field `PRI` writer - 19:16\\]
Priority of the highest priority pending IRQ. valid only if the valid flag is set."]
pub type PriW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES2` reader - 30:20\\]
RESERVE FIELD"]
pub type Res2R = crate::FieldReader<u16>;
#[doc = "Field `RES2` writer - 30:20\\]
RESERVE FIELD"]
pub type Res2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `VALID` reader - 31:31\\]
Indicates that the num field is valid."]
pub type ValidR = crate::BitReader;
#[doc = "Field `VALID` writer - 31:31\\]
Indicates that the num field is valid."]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Number of the highest priority pending IRQ. valid only if the valid flag is set."]
    #[inline(always)]
    pub fn num(&self) -> NumR {
        NumR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - 15:10\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res3(&self) -> Res3R {
        Res3R::new(((self.bits >> 10) & 0x3f) as u8)
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
    pub fn res2(&self) -> Res2R {
        Res2R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Indicates that the num field is valid."]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Number of the highest priority pending IRQ. valid only if the valid flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn num(&mut self) -> NumW<PriirqSpec> {
        NumW::new(self, 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res3(&mut self) -> Res3W<PriirqSpec> {
        Res3W::new(self, 10)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Priority of the highest priority pending IRQ. valid only if the valid flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PriW<PriirqSpec> {
        PriW::new(self, 16)
    }
    #[doc = "Bits 20:30 - 30:20\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res2(&mut self) -> Res2W<PriirqSpec> {
        Res2W::new(self, 20)
    }
    #[doc = "Bit 31 - 31:31\\]
Indicates that the num field is valid."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<PriirqSpec> {
        ValidW::new(self, 31)
    }
}
#[doc = "The Prioritized IRQ Register shows the number of the highest priority pending IRQ.\n\nYou can [`read`](crate::Reg::read) this register and get [`priirq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priirq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PriirqSpec;
impl crate::RegisterSpec for PriirqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priirq::R`](R) reader structure"]
impl crate::Readable for PriirqSpec {}
#[doc = "`write(|w| ..)` method takes [`priirq::W`](W) writer structure"]
impl crate::Writable for PriirqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIIRQ to value 0"]
impl crate::Resettable for PriirqSpec {
    const RESET_VALUE: u32 = 0;
}
