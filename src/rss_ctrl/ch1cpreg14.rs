#[doc = "Register `CH1CPREG14` reader"]
pub type R = crate::R<Ch1cpreg14Spec>;
#[doc = "Register `CH1CPREG14` writer"]
pub type W = crate::W<Ch1cpreg14Spec>;
#[doc = "Field `CH1CPREG14` reader - 31:0\\]
Multi Chirp 1 Parameters Register 14. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch1cpreg14R = crate::FieldReader<u32>;
#[doc = "Field `CH1CPREG14` writer - 31:0\\]
Multi Chirp 1 Parameters Register 14. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch1cpreg14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 1 Parameters Register 14. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch1cpreg14(&self) -> Ch1cpreg14R {
        Ch1cpreg14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 1 Parameters Register 14. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cpreg14(&mut self) -> Ch1cpreg14W<Ch1cpreg14Spec> {
        Ch1cpreg14W::new(self, 0)
    }
}
#[doc = "CH1CPREG14\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1cpreg14Spec;
impl crate::RegisterSpec for Ch1cpreg14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1cpreg14::R`](R) reader structure"]
impl crate::Readable for Ch1cpreg14Spec {}
#[doc = "`write(|w| ..)` method takes [`ch1cpreg14::W`](W) writer structure"]
impl crate::Writable for Ch1cpreg14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1CPREG14 to value 0"]
impl crate::Resettable for Ch1cpreg14Spec {
    const RESET_VALUE: u32 = 0;
}
