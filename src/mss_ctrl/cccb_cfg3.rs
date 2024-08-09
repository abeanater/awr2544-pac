#[doc = "Register `CCCB_CFG3` reader"]
pub type R = crate::R<CccbCfg3Spec>;
#[doc = "Register `CCCB_CFG3` writer"]
pub type W = crate::W<CccbCfg3Spec>;
#[doc = "Field `cccb_cfg` reader - 31:0\\]
Timeout Error Counter value in counter1 clock"]
pub type CccbCfgR = crate::FieldReader<u32>;
#[doc = "Field `cccb_cfg` writer - 31:0\\]
Timeout Error Counter value in counter1 clock"]
pub type CccbCfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Timeout Error Counter value in counter1 clock"]
    #[inline(always)]
    pub fn cccb_cfg(&self) -> CccbCfgR {
        CccbCfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Timeout Error Counter value in counter1 clock"]
    #[inline(always)]
    #[must_use]
    pub fn cccb_cfg(&mut self) -> CccbCfgW<CccbCfg3Spec> {
        CccbCfgW::new(self, 0)
    }
}
#[doc = "CCCB_CFG3\n\nYou can [`read`](crate::Reg::read) this register and get [`cccb_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccb_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccbCfg3Spec;
impl crate::RegisterSpec for CccbCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccb_cfg3::R`](R) reader structure"]
impl crate::Readable for CccbCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`cccb_cfg3::W`](W) writer structure"]
impl crate::Writable for CccbCfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCB_CFG3 to value 0"]
impl crate::Resettable for CccbCfg3Spec {
    const RESET_VALUE: u32 = 0;
}
