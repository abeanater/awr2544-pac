#[doc = "Register `PBIST_SCR4` reader"]
pub type R = crate::R<PbistScr4Spec>;
#[doc = "Register `PBIST_SCR4` writer"]
pub type W = crate::W<PbistScr4Spec>;
#[doc = "Field `SCR4` reader - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr4R = crate::FieldReader;
#[doc = "Field `SCR4` writer - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCR5` reader - 15:8\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr5R = crate::FieldReader;
#[doc = "Field `SCR5` writer - 15:8\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCR6` reader - 23:16\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr6R = crate::FieldReader;
#[doc = "Field `SCR6` writer - 23:16\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCR7` reader - 31:24\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr7R = crate::FieldReader;
#[doc = "Field `SCR7` writer - 31:24\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn scr4(&self) -> Scr4R {
        Scr4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn scr5(&self) -> Scr5R {
        Scr5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn scr6(&self) -> Scr6R {
        Scr6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn scr7(&self) -> Scr7R {
        Scr7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn scr4(&mut self) -> Scr4W<PbistScr4Spec> {
        Scr4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn scr5(&mut self) -> Scr5W<PbistScr4Spec> {
        Scr5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn scr6(&mut self) -> Scr6W<PbistScr4Spec> {
        Scr6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn scr7(&mut self) -> Scr7W<PbistScr4Spec> {
        Scr7W::new(self, 24)
    }
}
#[doc = "Address Scramble 4-7\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_scr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_scr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistScr4Spec;
impl crate::RegisterSpec for PbistScr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_scr4::R`](R) reader structure"]
impl crate::Readable for PbistScr4Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_scr4::W`](W) writer structure"]
impl crate::Writable for PbistScr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_SCR4 to value 0"]
impl crate::Resettable for PbistScr4Spec {
    const RESET_VALUE: u32 = 0;
}
