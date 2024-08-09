#[doc = "Register `CH6CPREG5` reader"]
pub type R = crate::R<Ch6cpreg5Spec>;
#[doc = "Register `CH6CPREG5` writer"]
pub type W = crate::W<Ch6cpreg5Spec>;
#[doc = "Field `CH6CPREG5` reader - 31:0\\]
Multi Chirp 6 Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch6cpreg5R = crate::FieldReader<u32>;
#[doc = "Field `CH6CPREG5` writer - 31:0\\]
Multi Chirp 6 Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch6cpreg5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 6 Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch6cpreg5(&self) -> Ch6cpreg5R {
        Ch6cpreg5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 6 Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch6cpreg5(&mut self) -> Ch6cpreg5W<Ch6cpreg5Spec> {
        Ch6cpreg5W::new(self, 0)
    }
}
#[doc = "CH6CPREG5\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch6cpreg5Spec;
impl crate::RegisterSpec for Ch6cpreg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6cpreg5::R`](R) reader structure"]
impl crate::Readable for Ch6cpreg5Spec {}
#[doc = "`write(|w| ..)` method takes [`ch6cpreg5::W`](W) writer structure"]
impl crate::Writable for Ch6cpreg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH6CPREG5 to value 0"]
impl crate::Resettable for Ch6cpreg5Spec {
    const RESET_VALUE: u32 = 0;
}
