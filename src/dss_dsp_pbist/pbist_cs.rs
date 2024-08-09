#[doc = "Register `PBIST_CS` reader"]
pub type R = crate::R<PbistCsSpec>;
#[doc = "Register `PBIST_CS` writer"]
pub type W = crate::W<PbistCsSpec>;
#[doc = "Field `CS0` reader - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type Cs0R = crate::FieldReader;
#[doc = "Field `CS0` writer - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type Cs0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CS1` reader - 15:8\\]
TI Internal Register.Reserved for HW RnD"]
pub type Cs1R = crate::FieldReader;
#[doc = "Field `CS1` writer - 15:8\\]
TI Internal Register.Reserved for HW RnD"]
pub type Cs1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CS2` reader - 23:16\\]
TI Internal Register.Reserved for HW RnD"]
pub type Cs2R = crate::FieldReader;
#[doc = "Field `CS2` writer - 23:16\\]
TI Internal Register.Reserved for HW RnD"]
pub type Cs2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CS3` reader - 31:24\\]
TI Internal Register.Reserved for HW RnD"]
pub type Cs3R = crate::FieldReader;
#[doc = "Field `CS3` writer - 31:24\\]
TI Internal Register.Reserved for HW RnD"]
pub type Cs3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn cs0(&self) -> Cs0R {
        Cs0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn cs1(&self) -> Cs1R {
        Cs1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn cs2(&self) -> Cs2R {
        Cs2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn cs3(&self) -> Cs3R {
        Cs3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn cs0(&mut self) -> Cs0W<PbistCsSpec> {
        Cs0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn cs1(&mut self) -> Cs1W<PbistCsSpec> {
        Cs1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn cs2(&mut self) -> Cs2W<PbistCsSpec> {
        Cs2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn cs3(&mut self) -> Cs3W<PbistCsSpec> {
        Cs3W::new(self, 24)
    }
}
#[doc = "Chip Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistCsSpec;
impl crate::RegisterSpec for PbistCsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_cs::R`](R) reader structure"]
impl crate::Readable for PbistCsSpec {}
#[doc = "`write(|w| ..)` method takes [`pbist_cs::W`](W) writer structure"]
impl crate::Writable for PbistCsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_CS to value 0"]
impl crate::Resettable for PbistCsSpec {
    const RESET_VALUE: u32 = 0;
}
