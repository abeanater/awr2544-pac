#[doc = "Register `PRIFIQ` reader"]
pub type R = crate::R<PrifiqSpec>;
#[doc = "Register `PRIFIQ` writer"]
pub type W = crate::W<PrifiqSpec>;
#[doc = "Field `NUM` reader - 9:0\\]
Number of the highest priority pending FIQ. valid only if the valid flag is set."]
pub type NumR = crate::FieldReader<u16>;
#[doc = "Field `NUM` writer - 9:0\\]
Number of the highest priority pending FIQ. valid only if the valid flag is set."]
pub type NumW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RES5` reader - 15:10\\]
RESERVE FIELD"]
pub type Res5R = crate::FieldReader;
#[doc = "Field `RES5` writer - 15:10\\]
RESERVE FIELD"]
pub type Res5W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PRI` reader - 19:16\\]
Priority of the highest priority pending FIQ. valid only if the valid flag is set."]
pub type PriR = crate::FieldReader;
#[doc = "Field `PRI` writer - 19:16\\]
Priority of the highest priority pending FIQ. valid only if the valid flag is set."]
pub type PriW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES4` reader - 30:20\\]
RESERVE FIELD"]
pub type Res4R = crate::FieldReader<u16>;
#[doc = "Field `RES4` writer - 30:20\\]
RESERVE FIELD"]
pub type Res4W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `VALID` reader - 31:31\\]
Indicates that the num field is valid."]
pub type ValidR = crate::BitReader;
#[doc = "Field `VALID` writer - 31:31\\]
Indicates that the num field is valid."]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Number of the highest priority pending FIQ. valid only if the valid flag is set."]
    #[inline(always)]
    pub fn num(&self) -> NumR {
        NumR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - 15:10\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res5(&self) -> Res5R {
        Res5R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Priority of the highest priority pending FIQ. valid only if the valid flag is set."]
    #[inline(always)]
    pub fn pri(&self) -> PriR {
        PriR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:30 - 30:20\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res4(&self) -> Res4R {
        Res4R::new(((self.bits >> 20) & 0x07ff) as u16)
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
Number of the highest priority pending FIQ. valid only if the valid flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn num(&mut self) -> NumW<PrifiqSpec> {
        NumW::new(self, 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res5(&mut self) -> Res5W<PrifiqSpec> {
        Res5W::new(self, 10)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Priority of the highest priority pending FIQ. valid only if the valid flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PriW<PrifiqSpec> {
        PriW::new(self, 16)
    }
    #[doc = "Bits 20:30 - 30:20\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res4(&mut self) -> Res4W<PrifiqSpec> {
        Res4W::new(self, 20)
    }
    #[doc = "Bit 31 - 31:31\\]
Indicates that the num field is valid."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<PrifiqSpec> {
        ValidW::new(self, 31)
    }
}
#[doc = "The Prioritized FIQ Register shows the number of the highest priority pending FIQ.\n\nYou can [`read`](crate::Reg::read) this register and get [`prifiq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prifiq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrifiqSpec;
impl crate::RegisterSpec for PrifiqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prifiq::R`](R) reader structure"]
impl crate::Readable for PrifiqSpec {}
#[doc = "`write(|w| ..)` method takes [`prifiq::W`](W) writer structure"]
impl crate::Writable for PrifiqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIFIQ to value 0"]
impl crate::Resettable for PrifiqSpec {
    const RESET_VALUE: u32 = 0;
}
