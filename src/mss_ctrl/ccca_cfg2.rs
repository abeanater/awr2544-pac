#[doc = "Register `CCCA_CFG2` reader"]
pub type R = crate::R<CccaCfg2Spec>;
#[doc = "Register `CCCA_CFG2` writer"]
pub type W = crate::W<CccaCfg2Spec>;
#[doc = "Field `ccca_cfg` reader - 31:0\\]
count1_expected_val Expected value of counter 1 when counter 0 expires after counting down from count0_expiry value"]
pub type CccaCfgR = crate::FieldReader<u32>;
#[doc = "Field `ccca_cfg` writer - 31:0\\]
count1_expected_val Expected value of counter 1 when counter 0 expires after counting down from count0_expiry value"]
pub type CccaCfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
count1_expected_val Expected value of counter 1 when counter 0 expires after counting down from count0_expiry value"]
    #[inline(always)]
    pub fn ccca_cfg(&self) -> CccaCfgR {
        CccaCfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
count1_expected_val Expected value of counter 1 when counter 0 expires after counting down from count0_expiry value"]
    #[inline(always)]
    #[must_use]
    pub fn ccca_cfg(&mut self) -> CccaCfgW<CccaCfg2Spec> {
        CccaCfgW::new(self, 0)
    }
}
#[doc = "CCCA_CFG2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccca_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccca_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccaCfg2Spec;
impl crate::RegisterSpec for CccaCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccca_cfg2::R`](R) reader structure"]
impl crate::Readable for CccaCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`ccca_cfg2::W`](W) writer structure"]
impl crate::Writable for CccaCfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCA_CFG2 to value 0"]
impl crate::Resettable for CccaCfg2Spec {
    const RESET_VALUE: u32 = 0;
}
