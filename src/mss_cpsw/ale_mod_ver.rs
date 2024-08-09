#[doc = "Register `ALE_MOD_VER` reader"]
pub type R = crate::R<AleModVerSpec>;
#[doc = "Register `ALE_MOD_VER` writer"]
pub type W = crate::W<AleModVerSpec>;
#[doc = "Field `MINOR_REVISION_` reader - 5:0\\]
Minor Revision."]
pub type MinorRevision_R = crate::FieldReader;
#[doc = "Field `MINOR_REVISION_` writer - 5:0\\]
Minor Revision."]
pub type MinorRevision_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM_REVISION_` reader - 7:6\\]
Custom Revision."]
pub type CustomRevision_R = crate::FieldReader;
#[doc = "Field `CUSTOM_REVISION_` writer - 7:6\\]
Custom Revision."]
pub type CustomRevision_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJOR_REVISION_` reader - 10:8\\]
Major Revision."]
pub type MajorRevision_R = crate::FieldReader;
#[doc = "Field `MAJOR_REVISION_` writer - 10:8\\]
Major Revision."]
pub type MajorRevision_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL_VERSION_` reader - 15:11\\]
RTL Version."]
pub type RtlVersion_R = crate::FieldReader;
#[doc = "Field `RTL_VERSION_` writer - 15:11\\]
RTL Version."]
pub type RtlVersion_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ALE_2G32_MODULE_ID_` reader - 31:16\\]
ALE_2g32 module ID."]
pub type Ale2g32ModuleId_R = crate::FieldReader<u16>;
#[doc = "Field `ALE_2G32_MODULE_ID_` writer - 31:16\\]
ALE_2g32 module ID."]
pub type Ale2g32ModuleId_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor Revision."]
    #[inline(always)]
    pub fn minor_revision_(&self) -> MinorRevision_R {
        MinorRevision_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom Revision."]
    #[inline(always)]
    pub fn custom_revision_(&self) -> CustomRevision_R {
        CustomRevision_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Revision."]
    #[inline(always)]
    pub fn major_revision_(&self) -> MajorRevision_R {
        MajorRevision_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL Version."]
    #[inline(always)]
    pub fn rtl_version_(&self) -> RtlVersion_R {
        RtlVersion_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
ALE_2g32 module ID."]
    #[inline(always)]
    pub fn ale_2g32_module_id_(&self) -> Ale2g32ModuleId_R {
        Ale2g32ModuleId_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor Revision."]
    #[inline(always)]
    #[must_use]
    pub fn minor_revision_(&mut self) -> MinorRevision_W<AleModVerSpec> {
        MinorRevision_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom Revision."]
    #[inline(always)]
    #[must_use]
    pub fn custom_revision_(&mut self) -> CustomRevision_W<AleModVerSpec> {
        CustomRevision_W::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Revision."]
    #[inline(always)]
    #[must_use]
    pub fn major_revision_(&mut self) -> MajorRevision_W<AleModVerSpec> {
        MajorRevision_W::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL Version."]
    #[inline(always)]
    #[must_use]
    pub fn rtl_version_(&mut self) -> RtlVersion_W<AleModVerSpec> {
        RtlVersion_W::new(self, 11)
    }
    #[doc = "Bits 16:31 - 31:16\\]
ALE_2g32 module ID."]
    #[inline(always)]
    #[must_use]
    pub fn ale_2g32_module_id_(&mut self) -> Ale2g32ModuleId_W<AleModVerSpec> {
        Ale2g32ModuleId_W::new(self, 16)
    }
}
#[doc = "The Module and Version Register identifies the module identifier and revision of the ALE_2g32 module.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_mod_ver::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_mod_ver::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleModVerSpec;
impl crate::RegisterSpec for AleModVerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_mod_ver::R`](R) reader structure"]
impl crate::Readable for AleModVerSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_mod_ver::W`](W) writer structure"]
impl crate::Writable for AleModVerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_MOD_VER to value 0"]
impl crate::Resettable for AleModVerSpec {
    const RESET_VALUE: u32 = 0;
}
