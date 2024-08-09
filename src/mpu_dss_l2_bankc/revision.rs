#[doc = "Register `Revision` reader"]
pub type R = crate::R<RevisionSpec>;
#[doc = "Register `Revision` writer"]
pub type W = crate::W<RevisionSpec>;
#[doc = "Field `revmin` reader - 5:0\\]
Minor revision."]
pub type RevminR = crate::FieldReader;
#[doc = "Field `revmin` writer - 5:0\\]
Minor revision."]
pub type RevminW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `revcustom` reader - 7:6\\]
Custom revision."]
pub type RevcustomR = crate::FieldReader;
#[doc = "Field `revcustom` writer - 7:6\\]
Custom revision."]
pub type RevcustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `revmaj` reader - 10:8\\]
Majo revision."]
pub type RevmajR = crate::FieldReader;
#[doc = "Field `revmaj` writer - 10:8\\]
Majo revision."]
pub type RevmajW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `revrtl` reader - 15:11\\]
RTL revision.Will vary depending on release."]
pub type RevrtlR = crate::FieldReader;
#[doc = "Field `revrtl` writer - 15:11\\]
RTL revision.Will vary depending on release."]
pub type RevrtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `modID` reader - 27:16\\]
Module ID field."]
pub type ModIdR = crate::FieldReader<u16>;
#[doc = "Field `modID` writer - 27:16\\]
Module ID field."]
pub type ModIdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `scheme` reader - 31:30\\]
Scheme."]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `scheme` writer - 31:30\\]
Scheme."]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision."]
    #[inline(always)]
    pub fn revmin(&self) -> RevminR {
        RevminR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom revision."]
    #[inline(always)]
    pub fn revcustom(&self) -> RevcustomR {
        RevcustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Majo revision."]
    #[inline(always)]
    pub fn revmaj(&self) -> RevmajR {
        RevmajR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL revision.Will vary depending on release."]
    #[inline(always)]
    pub fn revrtl(&self) -> RevrtlR {
        RevrtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module ID field."]
    #[inline(always)]
    pub fn mod_id(&self) -> ModIdR {
        ModIdR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Scheme."]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision."]
    #[inline(always)]
    #[must_use]
    pub fn revmin(&mut self) -> RevminW<RevisionSpec> {
        RevminW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom revision."]
    #[inline(always)]
    #[must_use]
    pub fn revcustom(&mut self) -> RevcustomW<RevisionSpec> {
        RevcustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Majo revision."]
    #[inline(always)]
    #[must_use]
    pub fn revmaj(&mut self) -> RevmajW<RevisionSpec> {
        RevmajW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL revision.Will vary depending on release."]
    #[inline(always)]
    #[must_use]
    pub fn revrtl(&mut self) -> RevrtlW<RevisionSpec> {
        RevrtlW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module ID field."]
    #[inline(always)]
    #[must_use]
    pub fn mod_id(&mut self) -> ModIdW<RevisionSpec> {
        ModIdW::new(self, 16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Scheme."]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<RevisionSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "Revision\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`revision::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RevisionSpec;
impl crate::RegisterSpec for RevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`revision::R`](R) reader structure"]
impl crate::Readable for RevisionSpec {}
#[doc = "`write(|w| ..)` method takes [`revision::W`](W) writer structure"]
impl crate::Writable for RevisionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Revision to value 0"]
impl crate::Resettable for RevisionSpec {
    const RESET_VALUE: u32 = 0;
}
