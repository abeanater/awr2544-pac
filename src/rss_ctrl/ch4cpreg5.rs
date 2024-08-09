#[doc = "Register `CH4CPREG5` reader"]
pub type R = crate::R<Ch4cpreg5Spec>;
#[doc = "Register `CH4CPREG5` writer"]
pub type W = crate::W<Ch4cpreg5Spec>;
#[doc = "Field `CH4CPREG5` reader - 31:0\\]
Multi Chirp 4 Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch4cpreg5R = crate::FieldReader<u32>;
#[doc = "Field `CH4CPREG5` writer - 31:0\\]
Multi Chirp 4 Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch4cpreg5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 4 Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch4cpreg5(&self) -> Ch4cpreg5R {
        Ch4cpreg5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 4 Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch4cpreg5(&mut self) -> Ch4cpreg5W<Ch4cpreg5Spec> {
        Ch4cpreg5W::new(self, 0)
    }
}
#[doc = "CH4CPREG5\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cpreg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cpreg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4cpreg5Spec;
impl crate::RegisterSpec for Ch4cpreg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4cpreg5::R`](R) reader structure"]
impl crate::Readable for Ch4cpreg5Spec {}
#[doc = "`write(|w| ..)` method takes [`ch4cpreg5::W`](W) writer structure"]
impl crate::Writable for Ch4cpreg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH4CPREG5 to value 0"]
impl crate::Resettable for Ch4cpreg5Spec {
    const RESET_VALUE: u32 = 0;
}
