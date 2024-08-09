#[doc = "Register `REG_CMP_LFSRLOAD_1` reader"]
pub type R = crate::R<RegCmpLfsrload1Spec>;
#[doc = "Register `REG_CMP_LFSRLOAD_1` writer"]
pub type W = crate::W<RegCmpLfsrload1Spec>;
#[doc = "Field `REG_CMP_LFSRLOAD_1` reader - 0:0\\]
Resets CMP LFSR 1 with the programmed value. This is a self clearing bit"]
pub type RegCmpLfsrload1R = crate::BitReader;
#[doc = "Field `REG_CMP_LFSRLOAD_1` writer - 0:0\\]
Resets CMP LFSR 1 with the programmed value. This is a self clearing bit"]
pub type RegCmpLfsrload1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Resets CMP LFSR 1 with the programmed value. This is a self clearing bit"]
    #[inline(always)]
    pub fn reg_cmp_lfsrload_1(&self) -> RegCmpLfsrload1R {
        RegCmpLfsrload1R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Resets CMP LFSR 1 with the programmed value. This is a self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cmp_lfsrload_1(&mut self) -> RegCmpLfsrload1W<RegCmpLfsrload1Spec> {
        RegCmpLfsrload1W::new(self, 0)
    }
}
#[doc = "REG_CMP_LFSRLOAD_1\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmp_lfsrload_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cmp_lfsrload_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCmpLfsrload1Spec;
impl crate::RegisterSpec for RegCmpLfsrload1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cmp_lfsrload_1::R`](R) reader structure"]
impl crate::Readable for RegCmpLfsrload1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_cmp_lfsrload_1::W`](W) writer structure"]
impl crate::Writable for RegCmpLfsrload1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_CMP_LFSRLOAD_1 to value 0"]
impl crate::Resettable for RegCmpLfsrload1Spec {
    const RESET_VALUE: u32 = 0;
}
