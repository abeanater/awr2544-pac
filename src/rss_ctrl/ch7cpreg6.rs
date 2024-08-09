#[doc = "Register `CH7CPREG6` reader"]
pub type R = crate::R<Ch7cpreg6Spec>;
#[doc = "Register `CH7CPREG6` writer"]
pub type W = crate::W<Ch7cpreg6Spec>;
#[doc = "Field `CH7CPREG6` reader - 31:0\\]
Multi Chirp 7 Parameters Register 6. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch7cpreg6R = crate::FieldReader<u32>;
#[doc = "Field `CH7CPREG6` writer - 31:0\\]
Multi Chirp 7 Parameters Register 6. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch7cpreg6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 7 Parameters Register 6. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch7cpreg6(&self) -> Ch7cpreg6R {
        Ch7cpreg6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 7 Parameters Register 6. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch7cpreg6(&mut self) -> Ch7cpreg6W<Ch7cpreg6Spec> {
        Ch7cpreg6W::new(self, 0)
    }
}
#[doc = "CH7CPREG6\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch7cpreg6Spec;
impl crate::RegisterSpec for Ch7cpreg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7cpreg6::R`](R) reader structure"]
impl crate::Readable for Ch7cpreg6Spec {}
#[doc = "`write(|w| ..)` method takes [`ch7cpreg6::W`](W) writer structure"]
impl crate::Writable for Ch7cpreg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH7CPREG6 to value 0"]
impl crate::Resettable for Ch7cpreg6Spec {
    const RESET_VALUE: u32 = 0;
}
