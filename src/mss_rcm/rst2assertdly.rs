#[doc = "Register `RST2ASSERTDLY` reader"]
pub type R = crate::R<Rst2assertdlySpec>;
#[doc = "Register `RST2ASSERTDLY` writer"]
pub type W = crate::W<Rst2assertdlySpec>;
#[doc = "Field `r5ssa` reader - 7:0\\]
Value decides number of cycles should be held before asserting reset for r5ss global reset for CR5A."]
pub type R5ssaR = crate::FieldReader;
#[doc = "Field `r5ssa` writer - 7:0\\]
Value decides number of cycles should be held before asserting reset for r5ss global reset for CR5A."]
pub type R5ssaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r5ssb` reader - 15:8\\]
RESERVED: Dont Use"]
pub type R5ssbR = crate::FieldReader;
#[doc = "Field `r5ssb` writer - 15:8\\]
RESERVED: Dont Use"]
pub type R5ssbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r5a` reader - 23:16\\]
Value decides number of cycles should be held before asserting reset for r5ss local reset for CR5A"]
pub type R5aR = crate::FieldReader;
#[doc = "Field `r5a` writer - 23:16\\]
Value decides number of cycles should be held before asserting reset for r5ss local reset for CR5A"]
pub type R5aW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r5b` reader - 31:24\\]
RESERVED: Dont Use"]
pub type R5bR = crate::FieldReader;
#[doc = "Field `r5b` writer - 31:24\\]
RESERVED: Dont Use"]
pub type R5bW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Value decides number of cycles should be held before asserting reset for r5ss global reset for CR5A."]
    #[inline(always)]
    pub fn r5ssa(&self) -> R5ssaR {
        R5ssaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn r5ssb(&self) -> R5ssbR {
        R5ssbR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Value decides number of cycles should be held before asserting reset for r5ss local reset for CR5A"]
    #[inline(always)]
    pub fn r5a(&self) -> R5aR {
        R5aR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn r5b(&self) -> R5bR {
        R5bR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Value decides number of cycles should be held before asserting reset for r5ss global reset for CR5A."]
    #[inline(always)]
    #[must_use]
    pub fn r5ssa(&mut self) -> R5ssaW<Rst2assertdlySpec> {
        R5ssaW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn r5ssb(&mut self) -> R5ssbW<Rst2assertdlySpec> {
        R5ssbW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Value decides number of cycles should be held before asserting reset for r5ss local reset for CR5A"]
    #[inline(always)]
    #[must_use]
    pub fn r5a(&mut self) -> R5aW<Rst2assertdlySpec> {
        R5aW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn r5b(&mut self) -> R5bW<Rst2assertdlySpec> {
        R5bW::new(self, 24)
    }
}
#[doc = "RST2ASSERTDLY\n\nYou can [`read`](crate::Reg::read) this register and get [`rst2assertdly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst2assertdly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rst2assertdlySpec;
impl crate::RegisterSpec for Rst2assertdlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst2assertdly::R`](R) reader structure"]
impl crate::Readable for Rst2assertdlySpec {}
#[doc = "`write(|w| ..)` method takes [`rst2assertdly::W`](W) writer structure"]
impl crate::Writable for Rst2assertdlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST2ASSERTDLY to value 0"]
impl crate::Resettable for Rst2assertdlySpec {
    const RESET_VALUE: u32 = 0;
}
