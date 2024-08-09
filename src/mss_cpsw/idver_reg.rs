#[doc = "Register `IDVER_REG` reader"]
pub type R = crate::R<IdverRegSpec>;
#[doc = "Register `IDVER_REG` writer"]
pub type W = crate::W<IdverRegSpec>;
#[doc = "Field `MINOR_VERSION_VALUE` reader - 7:0\\]
Minor version value"]
pub type MinorVersionValueR = crate::FieldReader;
#[doc = "Field `MINOR_VERSION_VALUE` writer - 7:0\\]
Minor version value"]
pub type MinorVersionValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAJOR_VERSION_VALUE` reader - 10:8\\]
Major version value"]
pub type MajorVersionValueR = crate::FieldReader;
#[doc = "Field `MAJOR_VERSION_VALUE` writer - 10:8\\]
Major version value"]
pub type MajorVersionValueW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL_VERSION_VALUE` reader - 15:11\\]
RTL version value"]
pub type RtlVersionValueR = crate::FieldReader;
#[doc = "Field `RTL_VERSION_VALUE` writer - 15:11\\]
RTL version value"]
pub type RtlVersionValueW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IDENTIFICATION_VALUE` reader - 31:16\\]
Identification value"]
pub type IdentificationValueR = crate::FieldReader<u16>;
#[doc = "Field `IDENTIFICATION_VALUE` writer - 31:16\\]
Identification value"]
pub type IdentificationValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Minor version value"]
    #[inline(always)]
    pub fn minor_version_value(&self) -> MinorVersionValueR {
        MinorVersionValueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major version value"]
    #[inline(always)]
    pub fn major_version_value(&self) -> MajorVersionValueR {
        MajorVersionValueR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version value"]
    #[inline(always)]
    pub fn rtl_version_value(&self) -> RtlVersionValueR {
        RtlVersionValueR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Identification value"]
    #[inline(always)]
    pub fn identification_value(&self) -> IdentificationValueR {
        IdentificationValueR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Minor version value"]
    #[inline(always)]
    #[must_use]
    pub fn minor_version_value(&mut self) -> MinorVersionValueW<IdverRegSpec> {
        MinorVersionValueW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major version value"]
    #[inline(always)]
    #[must_use]
    pub fn major_version_value(&mut self) -> MajorVersionValueW<IdverRegSpec> {
        MajorVersionValueW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version value"]
    #[inline(always)]
    #[must_use]
    pub fn rtl_version_value(&mut self) -> RtlVersionValueW<IdverRegSpec> {
        RtlVersionValueW::new(self, 11)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Identification value"]
    #[inline(always)]
    #[must_use]
    pub fn identification_value(&mut self) -> IdentificationValueW<IdverRegSpec> {
        IdentificationValueW::new(self, 16)
    }
}
#[doc = "Identification and Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idver_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idver_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdverRegSpec;
impl crate::RegisterSpec for IdverRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idver_reg::R`](R) reader structure"]
impl crate::Readable for IdverRegSpec {}
#[doc = "`write(|w| ..)` method takes [`idver_reg::W`](W) writer structure"]
impl crate::Writable for IdverRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDVER_REG to value 0"]
impl crate::Resettable for IdverRegSpec {
    const RESET_VALUE: u32 = 0;
}
