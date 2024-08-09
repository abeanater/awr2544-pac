#[doc = "Register `REGISTER13` reader"]
pub type R = crate::R<Register13Spec>;
#[doc = "Register `REGISTER13` writer"]
pub type W = crate::W<Register13Spec>;
#[doc = "Field `EMPTY` reader - 7:0\\]
Reserved"]
pub type EmptyR = crate::FieldReader;
#[doc = "Field `EMPTY` writer - 7:0\\]
Reserved"]
pub type EmptyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ANALOGTESTMODES` reader - 31:8\\]
Default:0"]
pub type AnalogtestmodesR = crate::FieldReader<u32>;
#[doc = "Field `ANALOGTESTMODES` writer - 31:8\\]
Default:0"]
pub type AnalogtestmodesW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Default:0"]
    #[inline(always)]
    pub fn analogtestmodes(&self) -> AnalogtestmodesR {
        AnalogtestmodesR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<Register13Spec> {
        EmptyW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Default:0"]
    #[inline(always)]
    #[must_use]
    pub fn analogtestmodes(&mut self) -> AnalogtestmodesW<Register13Spec> {
        AnalogtestmodesW::new(self, 8)
    }
}
#[doc = "REGISTER13\n\nYou can [`read`](crate::Reg::read) this register and get [`register13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register13Spec;
impl crate::RegisterSpec for Register13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register13::R`](R) reader structure"]
impl crate::Readable for Register13Spec {}
#[doc = "`write(|w| ..)` method takes [`register13::W`](W) writer structure"]
impl crate::Writable for Register13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER13 to value 0"]
impl crate::Resettable for Register13Spec {
    const RESET_VALUE: u32 = 0;
}
