#[doc = "Register `REG_CMP_LFSRLOAD_0` reader"]
pub type R = crate::R<RegCmpLfsrload0Spec>;
#[doc = "Register `REG_CMP_LFSRLOAD_0` writer"]
pub type W = crate::W<RegCmpLfsrload0Spec>;
#[doc = "Field `REG_CMP_LFSRLOAD_0` reader - 0:0\\]
Resets CMP LFSR 0 with the programmed value. This is a self clearing bit"]
pub type RegCmpLfsrload0R = crate::BitReader;
#[doc = "Field `REG_CMP_LFSRLOAD_0` writer - 0:0\\]
Resets CMP LFSR 0 with the programmed value. This is a self clearing bit"]
pub type RegCmpLfsrload0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Resets CMP LFSR 0 with the programmed value. This is a self clearing bit"]
    #[inline(always)]
    pub fn reg_cmp_lfsrload_0(&self) -> RegCmpLfsrload0R {
        RegCmpLfsrload0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Resets CMP LFSR 0 with the programmed value. This is a self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cmp_lfsrload_0(&mut self) -> RegCmpLfsrload0W<RegCmpLfsrload0Spec> {
        RegCmpLfsrload0W::new(self, 0)
    }
}
#[doc = "REG_CMP_LFSRLOAD_0\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmp_lfsrload_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cmp_lfsrload_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCmpLfsrload0Spec;
impl crate::RegisterSpec for RegCmpLfsrload0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cmp_lfsrload_0::R`](R) reader structure"]
impl crate::Readable for RegCmpLfsrload0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_cmp_lfsrload_0::W`](W) writer structure"]
impl crate::Writable for RegCmpLfsrload0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_CMP_LFSRLOAD_0 to value 0"]
impl crate::Resettable for RegCmpLfsrload0Spec {
    const RESET_VALUE: u32 = 0;
}
