#[doc = "Register `PBIST_RAMT` reader"]
pub type R = crate::R<PbistRamtSpec>;
#[doc = "Register `PBIST_RAMT` writer"]
pub type W = crate::W<PbistRamtSpec>;
#[doc = "Field `RAM` reader - 7:0\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
pub type RamR = crate::FieldReader;
#[doc = "Field `RAM` writer - 7:0\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
pub type RamW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DWR` reader - 15:8\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
pub type DwrR = crate::FieldReader;
#[doc = "Field `DWR` writer - 15:8\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
pub type DwrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RDS` reader - 23:16\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
pub type RdsR = crate::FieldReader;
#[doc = "Field `RDS` writer - 23:16\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
pub type RdsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGS` reader - 31:24\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
pub type RgsR = crate::FieldReader;
#[doc = "Field `RGS` writer - 31:24\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
pub type RgsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
    #[inline(always)]
    pub fn dwr(&self) -> DwrR {
        DwrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
    #[inline(always)]
    pub fn rds(&self) -> RdsR {
        RdsR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
    #[inline(always)]
    pub fn rgs(&self) -> RgsR {
        RgsR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
    #[inline(always)]
    #[must_use]
    pub fn ram(&mut self) -> RamW<PbistRamtSpec> {
        RamW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
    #[inline(always)]
    #[must_use]
    pub fn dwr(&mut self) -> DwrW<PbistRamtSpec> {
        DwrW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
    #[inline(always)]
    #[must_use]
    pub fn rds(&mut self) -> RdsW<PbistRamtSpec> {
        RdsW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
    #[inline(always)]
    #[must_use]
    pub fn rgs(&mut self) -> RgsW<PbistRamtSpec> {
        RgsW::new(self, 24)
    }
}
#[doc = "RAM Configuration (RAMT -RAM)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ramt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ramt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistRamtSpec;
impl crate::RegisterSpec for PbistRamtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_ramt::R`](R) reader structure"]
impl crate::Readable for PbistRamtSpec {}
#[doc = "`write(|w| ..)` method takes [`pbist_ramt::W`](W) writer structure"]
impl crate::Writable for PbistRamtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_RAMT to value 0"]
impl crate::Resettable for PbistRamtSpec {
    const RESET_VALUE: u32 = 0;
}
