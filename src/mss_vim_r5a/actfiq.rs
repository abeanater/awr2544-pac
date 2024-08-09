#[doc = "Register `ACTFIQ` reader"]
pub type R = crate::R<ActfiqSpec>;
#[doc = "Register `ACTFIQ` writer"]
pub type W = crate::W<ActfiqSpec>;
#[doc = "Field `NUM` reader - 9:0\\]
Number of the currently active FIQ. Loaded from teh Prioritized FIQ Register whenever the FIQ Vector Address is read. Valid only if the valid flag is set."]
pub type NumR = crate::FieldReader<u16>;
#[doc = "Field `NUM` writer - 9:0\\]
Number of the currently active FIQ. Loaded from teh Prioritized FIQ Register whenever the FIQ Vector Address is read. Valid only if the valid flag is set."]
pub type NumW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RES9` reader - 15:10\\]
RESERVE FIELD"]
pub type Res9R = crate::FieldReader;
#[doc = "Field `RES9` writer - 15:10\\]
RESERVE FIELD"]
pub type Res9W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PRI` reader - 19:16\\]
Priority of the highest priority pending IRQ. valid only if the valid flag is set."]
pub type PriR = crate::FieldReader;
#[doc = "Field `PRI` writer - 19:16\\]
Priority of the highest priority pending IRQ. valid only if the valid flag is set."]
pub type PriW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES8` reader - 30:20\\]
RESERVE FIELD"]
pub type Res8R = crate::FieldReader<u16>;
#[doc = "Field `RES8` writer - 30:20\\]
RESERVE FIELD"]
pub type Res8W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `VALID` reader - 31:31\\]
Indicates that the num field is valid. Set when the FIQ Vector Address Register is read and cleared whenever the FIQ Vector Address Register is written."]
pub type ValidR = crate::BitReader;
#[doc = "Field `VALID` writer - 31:31\\]
Indicates that the num field is valid. Set when the FIQ Vector Address Register is read and cleared whenever the FIQ Vector Address Register is written."]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Number of the currently active FIQ. Loaded from teh Prioritized FIQ Register whenever the FIQ Vector Address is read. Valid only if the valid flag is set."]
    #[inline(always)]
    pub fn num(&self) -> NumR {
        NumR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - 15:10\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res9(&self) -> Res9R {
        Res9R::new(((self.bits >> 10) & 0x3f) as u8)
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
    pub fn res8(&self) -> Res8R {
        Res8R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Indicates that the num field is valid. Set when the FIQ Vector Address Register is read and cleared whenever the FIQ Vector Address Register is written."]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Number of the currently active FIQ. Loaded from teh Prioritized FIQ Register whenever the FIQ Vector Address is read. Valid only if the valid flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn num(&mut self) -> NumW<ActfiqSpec> {
        NumW::new(self, 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res9(&mut self) -> Res9W<ActfiqSpec> {
        Res9W::new(self, 10)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Priority of the highest priority pending IRQ. valid only if the valid flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PriW<ActfiqSpec> {
        PriW::new(self, 16)
    }
    #[doc = "Bits 20:30 - 30:20\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res8(&mut self) -> Res8W<ActfiqSpec> {
        Res8W::new(self, 20)
    }
    #[doc = "Bit 31 - 31:31\\]
Indicates that the num field is valid. Set when the FIQ Vector Address Register is read and cleared whenever the FIQ Vector Address Register is written."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<ActfiqSpec> {
        ValidW::new(self, 31)
    }
}
#[doc = "The Active FIQ Register shows the number of the currently active FIQ.\n\nYou can [`read`](crate::Reg::read) this register and get [`actfiq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actfiq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActfiqSpec;
impl crate::RegisterSpec for ActfiqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actfiq::R`](R) reader structure"]
impl crate::Readable for ActfiqSpec {}
#[doc = "`write(|w| ..)` method takes [`actfiq::W`](W) writer structure"]
impl crate::Writable for ActfiqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACTFIQ to value 0"]
impl crate::Resettable for ActfiqSpec {
    const RESET_VALUE: u32 = 0;
}
