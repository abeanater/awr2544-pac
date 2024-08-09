#[doc = "Register `CCCA_CFG3` reader"]
pub type R = crate::R<CccaCfg3Spec>;
#[doc = "Register `CCCA_CFG3` writer"]
pub type W = crate::W<CccaCfg3Spec>;
#[doc = "Field `ccca_cfg` reader - 31:0\\]
Timeout Error Counter value in counter1 clock"]
pub type CccaCfgR = crate::FieldReader<u32>;
#[doc = "Field `ccca_cfg` writer - 31:0\\]
Timeout Error Counter value in counter1 clock"]
pub type CccaCfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Timeout Error Counter value in counter1 clock"]
    #[inline(always)]
    pub fn ccca_cfg(&self) -> CccaCfgR {
        CccaCfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Timeout Error Counter value in counter1 clock"]
    #[inline(always)]
    #[must_use]
    pub fn ccca_cfg(&mut self) -> CccaCfgW<CccaCfg3Spec> {
        CccaCfgW::new(self, 0)
    }
}
#[doc = "CCCA_CFG3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccca_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccca_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccaCfg3Spec;
impl crate::RegisterSpec for CccaCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccca_cfg3::R`](R) reader structure"]
impl crate::Readable for CccaCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`ccca_cfg3::W`](W) writer structure"]
impl crate::Writable for CccaCfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCA_CFG3 to value 0"]
impl crate::Resettable for CccaCfg3Spec {
    const RESET_VALUE: u32 = 0;
}
