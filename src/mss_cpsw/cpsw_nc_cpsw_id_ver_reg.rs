#[doc = "Register `CPSW_NC_CPSW_ID_VER_REG` reader"]
pub type R = crate::R<CpswNcCpswIdVerRegSpec>;
#[doc = "Register `CPSW_NC_CPSW_ID_VER_REG` writer"]
pub type W = crate::W<CpswNcCpswIdVerRegSpec>;
#[doc = "Field `MINOR_VERSION_VALUE` reader - 7:0\\]
Minor Version Value"]
pub type MinorVersionValueR = crate::FieldReader;
#[doc = "Field `MINOR_VERSION_VALUE` writer - 7:0\\]
Minor Version Value"]
pub type MinorVersionValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAJOR_VERSION_VALUE` reader - 10:8\\]
Major Version Value"]
pub type MajorVersionValueR = crate::FieldReader;
#[doc = "Field `MAJOR_VERSION_VALUE` writer - 10:8\\]
Major Version Value"]
pub type MajorVersionValueW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL_VERSION_VALUE` reader - 15:11\\]
RTL Version Value"]
pub type RtlVersionValueR = crate::FieldReader;
#[doc = "Field `RTL_VERSION_VALUE` writer - 15:11\\]
RTL Version Value"]
pub type RtlVersionValueW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IDENTIFICATION_VALUE` reader - 31:16\\]
Identification Value"]
pub type IdentificationValueR = crate::FieldReader<u16>;
#[doc = "Field `IDENTIFICATION_VALUE` writer - 31:16\\]
Identification Value"]
pub type IdentificationValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Minor Version Value"]
    #[inline(always)]
    pub fn minor_version_value(&self) -> MinorVersionValueR {
        MinorVersionValueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Version Value"]
    #[inline(always)]
    pub fn major_version_value(&self) -> MajorVersionValueR {
        MajorVersionValueR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL Version Value"]
    #[inline(always)]
    pub fn rtl_version_value(&self) -> RtlVersionValueR {
        RtlVersionValueR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Identification Value"]
    #[inline(always)]
    pub fn identification_value(&self) -> IdentificationValueR {
        IdentificationValueR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Minor Version Value"]
    #[inline(always)]
    #[must_use]
    pub fn minor_version_value(&mut self) -> MinorVersionValueW<CpswNcCpswIdVerRegSpec> {
        MinorVersionValueW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Version Value"]
    #[inline(always)]
    #[must_use]
    pub fn major_version_value(&mut self) -> MajorVersionValueW<CpswNcCpswIdVerRegSpec> {
        MajorVersionValueW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL Version Value"]
    #[inline(always)]
    #[must_use]
    pub fn rtl_version_value(&mut self) -> RtlVersionValueW<CpswNcCpswIdVerRegSpec> {
        RtlVersionValueW::new(self, 11)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Identification Value"]
    #[inline(always)]
    #[must_use]
    pub fn identification_value(&mut self) -> IdentificationValueW<CpswNcCpswIdVerRegSpec> {
        IdentificationValueW::new(self, 16)
    }
}
#[doc = "CPSW ID Version\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cpsw_id_ver_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cpsw_id_ver_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCpswIdVerRegSpec;
impl crate::RegisterSpec for CpswNcCpswIdVerRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cpsw_id_ver_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCpswIdVerRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cpsw_id_ver_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCpswIdVerRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPSW_ID_VER_REG to value 0"]
impl crate::Resettable for CpswNcCpswIdVerRegSpec {
    const RESET_VALUE: u32 = 0;
}
