#[doc = "Register `MDIO_MDIO_VERSION_REG` reader"]
pub type R = crate::R<MdioMdioVersionRegSpec>;
#[doc = "Register `MDIO_MDIO_VERSION_REG` writer"]
pub type W = crate::W<MdioMdioVersionRegSpec>;
#[doc = "Field `MINOR_VERSION` reader - 5:0\\]
Minor version"]
pub type MinorVersionR = crate::FieldReader;
#[doc = "Field `MINOR_VERSION` writer - 5:0\\]
Minor version"]
pub type MinorVersionW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM_VERSION` reader - 7:6\\]
Custom version"]
pub type CustomVersionR = crate::FieldReader;
#[doc = "Field `CUSTOM_VERSION` writer - 7:6\\]
Custom version"]
pub type CustomVersionW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJOR_VERSION` reader - 10:8\\]
Major version"]
pub type MajorVersionR = crate::FieldReader;
#[doc = "Field `MAJOR_VERSION` writer - 10:8\\]
Major version"]
pub type MajorVersionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL_VERSION` reader - 15:11\\]
RTL version"]
pub type RtlVersionR = crate::FieldReader;
#[doc = "Field `RTL_VERSION` writer - 15:11\\]
RTL version"]
pub type RtlVersionW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MODULE_ID` reader - 27:16\\]
Module ID"]
pub type ModuleIdR = crate::FieldReader<u16>;
#[doc = "Field `MODULE_ID` writer - 27:16\\]
Module ID"]
pub type ModuleIdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BU` reader - 29:28\\]
bu"]
pub type BuR = crate::FieldReader;
#[doc = "Field `BU` writer - 29:28\\]
bu"]
pub type BuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCHEME` reader - 31:30\\]
Scheme"]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
Scheme"]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor version"]
    #[inline(always)]
    pub fn minor_version(&self) -> MinorVersionR {
        MinorVersionR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom version"]
    #[inline(always)]
    pub fn custom_version(&self) -> CustomVersionR {
        CustomVersionR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major version"]
    #[inline(always)]
    pub fn major_version(&self) -> MajorVersionR {
        MajorVersionR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version"]
    #[inline(always)]
    pub fn rtl_version(&self) -> RtlVersionR {
        RtlVersionR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module ID"]
    #[inline(always)]
    pub fn module_id(&self) -> ModuleIdR {
        ModuleIdR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
bu"]
    #[inline(always)]
    pub fn bu(&self) -> BuR {
        BuR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Scheme"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor version"]
    #[inline(always)]
    #[must_use]
    pub fn minor_version(&mut self) -> MinorVersionW<MdioMdioVersionRegSpec> {
        MinorVersionW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom version"]
    #[inline(always)]
    #[must_use]
    pub fn custom_version(&mut self) -> CustomVersionW<MdioMdioVersionRegSpec> {
        CustomVersionW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major version"]
    #[inline(always)]
    #[must_use]
    pub fn major_version(&mut self) -> MajorVersionW<MdioMdioVersionRegSpec> {
        MajorVersionW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version"]
    #[inline(always)]
    #[must_use]
    pub fn rtl_version(&mut self) -> RtlVersionW<MdioMdioVersionRegSpec> {
        RtlVersionW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module ID"]
    #[inline(always)]
    #[must_use]
    pub fn module_id(&mut self) -> ModuleIdW<MdioMdioVersionRegSpec> {
        ModuleIdW::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
bu"]
    #[inline(always)]
    #[must_use]
    pub fn bu(&mut self) -> BuW<MdioMdioVersionRegSpec> {
        BuW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Scheme"]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<MdioMdioVersionRegSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "MDIO Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_mdio_version_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_mdio_version_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioMdioVersionRegSpec;
impl crate::RegisterSpec for MdioMdioVersionRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_mdio_version_reg::R`](R) reader structure"]
impl crate::Readable for MdioMdioVersionRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_mdio_version_reg::W`](W) writer structure"]
impl crate::Writable for MdioMdioVersionRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_MDIO_VERSION_REG to value 0"]
impl crate::Resettable for MdioMdioVersionRegSpec {
    const RESET_VALUE: u32 = 0;
}
