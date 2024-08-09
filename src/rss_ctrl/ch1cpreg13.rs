#[doc = "Register `CH1CPREG13` reader"]
pub type R = crate::R<Ch1cpreg13Spec>;
#[doc = "Register `CH1CPREG13` writer"]
pub type W = crate::W<Ch1cpreg13Spec>;
#[doc = "Field `CH1CPREG13` reader - 31:0\\]
Multi Chirp 1 Parameters Register 13. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch1cpreg13R = crate::FieldReader<u32>;
#[doc = "Field `CH1CPREG13` writer - 31:0\\]
Multi Chirp 1 Parameters Register 13. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch1cpreg13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 1 Parameters Register 13. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch1cpreg13(&self) -> Ch1cpreg13R {
        Ch1cpreg13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 1 Parameters Register 13. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cpreg13(&mut self) -> Ch1cpreg13W<Ch1cpreg13Spec> {
        Ch1cpreg13W::new(self, 0)
    }
}
#[doc = "CH1CPREG13\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1cpreg13Spec;
impl crate::RegisterSpec for Ch1cpreg13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1cpreg13::R`](R) reader structure"]
impl crate::Readable for Ch1cpreg13Spec {}
#[doc = "`write(|w| ..)` method takes [`ch1cpreg13::W`](W) writer structure"]
impl crate::Writable for Ch1cpreg13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1CPREG13 to value 0"]
impl crate::Resettable for Ch1cpreg13Spec {
    const RESET_VALUE: u32 = 0;
}
