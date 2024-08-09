#[doc = "Register `CH0CPREG12` reader"]
pub type R = crate::R<Ch0cpreg12Spec>;
#[doc = "Register `CH0CPREG12` writer"]
pub type W = crate::W<Ch0cpreg12Spec>;
#[doc = "Field `CH0CPREG12` reader - 31:0\\]
Multi Chirp 0 Parameters Register 12. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch0cpreg12R = crate::FieldReader<u32>;
#[doc = "Field `CH0CPREG12` writer - 31:0\\]
Multi Chirp 0 Parameters Register 12. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch0cpreg12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 0 Parameters Register 12. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch0cpreg12(&self) -> Ch0cpreg12R {
        Ch0cpreg12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 0 Parameters Register 12. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cpreg12(&mut self) -> Ch0cpreg12W<Ch0cpreg12Spec> {
        Ch0cpreg12W::new(self, 0)
    }
}
#[doc = "CH0CPREG12\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cpreg12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cpreg12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0cpreg12Spec;
impl crate::RegisterSpec for Ch0cpreg12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0cpreg12::R`](R) reader structure"]
impl crate::Readable for Ch0cpreg12Spec {}
#[doc = "`write(|w| ..)` method takes [`ch0cpreg12::W`](W) writer structure"]
impl crate::Writable for Ch0cpreg12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH0CPREG12 to value 0"]
impl crate::Resettable for Ch0cpreg12Spec {
    const RESET_VALUE: u32 = 0;
}
