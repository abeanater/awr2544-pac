#[doc = "Register `CMP_EGE_K4567` reader"]
pub type R = crate::R<CmpEgeK4567Spec>;
#[doc = "Register `CMP_EGE_K4567` writer"]
pub type W = crate::W<CmpEgeK4567Spec>;
#[doc = "Field `cmp_ege_k4` reader - 4:0\\]
4th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
pub type CmpEgeK4R = crate::FieldReader;
#[doc = "Field `cmp_ege_k4` writer - 4:0\\]
4th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
pub type CmpEgeK4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `cmp_ege_k5` reader - 12:8\\]
5th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
pub type CmpEgeK5R = crate::FieldReader;
#[doc = "Field `cmp_ege_k5` writer - 12:8\\]
5th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
pub type CmpEgeK5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `cmp_ege_k6` reader - 20:16\\]
6th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
pub type CmpEgeK6R = crate::FieldReader;
#[doc = "Field `cmp_ege_k6` writer - 20:16\\]
6th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
pub type CmpEgeK6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `cmp_ege_k7` reader - 28:24\\]
7th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
pub type CmpEgeK7R = crate::FieldReader;
#[doc = "Field `cmp_ege_k7` writer - 28:24\\]
7th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
pub type CmpEgeK7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
4th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
    #[inline(always)]
    pub fn cmp_ege_k4(&self) -> CmpEgeK4R {
        CmpEgeK4R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
5th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
    #[inline(always)]
    pub fn cmp_ege_k5(&self) -> CmpEgeK5R {
        CmpEgeK5R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
6th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
    #[inline(always)]
    pub fn cmp_ege_k6(&self) -> CmpEgeK6R {
        CmpEgeK6R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
7th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
    #[inline(always)]
    pub fn cmp_ege_k7(&self) -> CmpEgeK7R {
        CmpEgeK7R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
4th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ege_k4(&mut self) -> CmpEgeK4W<CmpEgeK4567Spec> {
        CmpEgeK4W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
5th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ege_k5(&mut self) -> CmpEgeK5W<CmpEgeK4567Spec> {
        CmpEgeK5W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
6th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ege_k6(&mut self) -> CmpEgeK6W<CmpEgeK4567Spec> {
        CmpEgeK6W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
7th K-param value should be loaded here which would be used in the First-pass of EGE Compression"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ege_k7(&mut self) -> CmpEgeK7W<CmpEgeK4567Spec> {
        CmpEgeK7W::new(self, 24)
    }
}
#[doc = "CMP_EGE_K4567\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_ege_k4567::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_ege_k4567::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpEgeK4567Spec;
impl crate::RegisterSpec for CmpEgeK4567Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp_ege_k4567::R`](R) reader structure"]
impl crate::Readable for CmpEgeK4567Spec {}
#[doc = "`write(|w| ..)` method takes [`cmp_ege_k4567::W`](W) writer structure"]
impl crate::Writable for CmpEgeK4567Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP_EGE_K4567 to value 0"]
impl crate::Resettable for CmpEgeK4567Spec {
    const RESET_VALUE: u32 = 0;
}
