#[doc = "Register `CH6CPREG15` reader"]
pub type R = crate::R<Ch6cpreg15Spec>;
#[doc = "Register `CH6CPREG15` writer"]
pub type W = crate::W<Ch6cpreg15Spec>;
#[doc = "Field `CH6CPREG15` reader - 31:0\\]
Multi Chirp 6 Parameters Register 15. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch6cpreg15R = crate::FieldReader<u32>;
#[doc = "Field `CH6CPREG15` writer - 31:0\\]
Multi Chirp 6 Parameters Register 15. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch6cpreg15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 6 Parameters Register 15. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch6cpreg15(&self) -> Ch6cpreg15R {
        Ch6cpreg15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 6 Parameters Register 15. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch6cpreg15(&mut self) -> Ch6cpreg15W<Ch6cpreg15Spec> {
        Ch6cpreg15W::new(self, 0)
    }
}
#[doc = "CH6CPREG15\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch6cpreg15Spec;
impl crate::RegisterSpec for Ch6cpreg15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6cpreg15::R`](R) reader structure"]
impl crate::Readable for Ch6cpreg15Spec {}
#[doc = "`write(|w| ..)` method takes [`ch6cpreg15::W`](W) writer structure"]
impl crate::Writable for Ch6cpreg15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH6CPREG15 to value 0"]
impl crate::Resettable for Ch6cpreg15Spec {
    const RESET_VALUE: u32 = 0;
}
