#[doc = "Register `REG_CMP_LFSRSEED_1` reader"]
pub type R = crate::R<RegCmpLfsrseed1Spec>;
#[doc = "Register `REG_CMP_LFSRSEED_1` writer"]
pub type W = crate::W<RegCmpLfsrseed1Spec>;
#[doc = "Field `REG_CMP_LFSRSEED_1` reader - 28:0\\]
This register is the second seed for the 29-bit LFSR used for compression"]
pub type RegCmpLfsrseed1R = crate::FieldReader<u32>;
#[doc = "Field `REG_CMP_LFSRSEED_1` writer - 28:0\\]
This register is the second seed for the 29-bit LFSR used for compression"]
pub type RegCmpLfsrseed1W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - 28:0\\]
This register is the second seed for the 29-bit LFSR used for compression"]
    #[inline(always)]
    pub fn reg_cmp_lfsrseed_1(&self) -> RegCmpLfsrseed1R {
        RegCmpLfsrseed1R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - 28:0\\]
This register is the second seed for the 29-bit LFSR used for compression"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cmp_lfsrseed_1(&mut self) -> RegCmpLfsrseed1W<RegCmpLfsrseed1Spec> {
        RegCmpLfsrseed1W::new(self, 0)
    }
}
#[doc = "REG_CMP_LFSRSEED_1\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmp_lfsrseed_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cmp_lfsrseed_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCmpLfsrseed1Spec;
impl crate::RegisterSpec for RegCmpLfsrseed1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cmp_lfsrseed_1::R`](R) reader structure"]
impl crate::Readable for RegCmpLfsrseed1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_cmp_lfsrseed_1::W`](W) writer structure"]
impl crate::Writable for RegCmpLfsrseed1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_CMP_LFSRSEED_1 to value 0"]
impl crate::Resettable for RegCmpLfsrseed1Spec {
    const RESET_VALUE: u32 = 0;
}
