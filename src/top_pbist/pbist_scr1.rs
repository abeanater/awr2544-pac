#[doc = "Register `PBIST_SCR1` reader"]
pub type R = crate::R<PbistScr1Spec>;
#[doc = "Register `PBIST_SCR1` writer"]
pub type W = crate::W<PbistScr1Spec>;
#[doc = "Field `SCR0` reader - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr0R = crate::FieldReader;
#[doc = "Field `SCR0` writer - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCR1` reader - 15:8\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr1R = crate::FieldReader;
#[doc = "Field `SCR1` writer - 15:8\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCR2` reader - 23:16\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr2R = crate::FieldReader;
#[doc = "Field `SCR2` writer - 23:16\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCR3` reader - 31:24\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr3R = crate::FieldReader;
#[doc = "Field `SCR3` writer - 31:24\\]
TI Internal Register.Reserved for HW RnD"]
pub type Scr3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn scr0(&self) -> Scr0R {
        Scr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn scr1(&self) -> Scr1R {
        Scr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn scr2(&self) -> Scr2R {
        Scr2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn scr3(&self) -> Scr3R {
        Scr3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn scr0(&mut self) -> Scr0W<PbistScr1Spec> {
        Scr0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn scr1(&mut self) -> Scr1W<PbistScr1Spec> {
        Scr1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn scr2(&mut self) -> Scr2W<PbistScr1Spec> {
        Scr2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn scr3(&mut self) -> Scr3W<PbistScr1Spec> {
        Scr3W::new(self, 24)
    }
}
#[doc = "Address Scramble 0 -3\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_scr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_scr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistScr1Spec;
impl crate::RegisterSpec for PbistScr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_scr1::R`](R) reader structure"]
impl crate::Readable for PbistScr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_scr1::W`](W) writer structure"]
impl crate::Writable for PbistScr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_SCR1 to value 0"]
impl crate::Resettable for PbistScr1Spec {
    const RESET_VALUE: u32 = 0;
}
