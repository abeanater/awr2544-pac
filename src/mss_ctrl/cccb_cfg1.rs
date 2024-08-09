#[doc = "Register `CCCB_CFG1` reader"]
pub type R = crate::R<CccbCfg1Spec>;
#[doc = "Register `CCCB_CFG1` writer"]
pub type W = crate::W<CccbCfg1Spec>;
#[doc = "Field `cccb_cfg` reader - 31:0\\]
count0_expiry_val Counter 1 is compared for count1_expected_val +/- MARGIN_COUNT when counter0 expires after counting down from count0_expiry_val to 0"]
pub type CccbCfgR = crate::FieldReader<u32>;
#[doc = "Field `cccb_cfg` writer - 31:0\\]
count0_expiry_val Counter 1 is compared for count1_expected_val +/- MARGIN_COUNT when counter0 expires after counting down from count0_expiry_val to 0"]
pub type CccbCfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
count0_expiry_val Counter 1 is compared for count1_expected_val +/- MARGIN_COUNT when counter0 expires after counting down from count0_expiry_val to 0"]
    #[inline(always)]
    pub fn cccb_cfg(&self) -> CccbCfgR {
        CccbCfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
count0_expiry_val Counter 1 is compared for count1_expected_val +/- MARGIN_COUNT when counter0 expires after counting down from count0_expiry_val to 0"]
    #[inline(always)]
    #[must_use]
    pub fn cccb_cfg(&mut self) -> CccbCfgW<CccbCfg1Spec> {
        CccbCfgW::new(self, 0)
    }
}
#[doc = "CCCB_CFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`cccb_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccb_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccbCfg1Spec;
impl crate::RegisterSpec for CccbCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccb_cfg1::R`](R) reader structure"]
impl crate::Readable for CccbCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cccb_cfg1::W`](W) writer structure"]
impl crate::Writable for CccbCfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCB_CFG1 to value 0"]
impl crate::Resettable for CccbCfg1Spec {
    const RESET_VALUE: u32 = 0;
}
