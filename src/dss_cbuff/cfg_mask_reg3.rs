#[doc = "Register `CFG_MASK_REG3` reader"]
pub type R = crate::R<CfgMaskReg3Spec>;
#[doc = "Register `CFG_MASK_REG3` writer"]
pub type W = crate::W<CfgMaskReg3Spec>;
#[doc = "Field `CFG_MASK_REG3` reader - 31:0\\]
RESERVED"]
pub type CfgMaskReg3R = crate::FieldReader<u32>;
#[doc = "Field `CFG_MASK_REG3` writer - 31:0\\]
RESERVED"]
pub type CfgMaskReg3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
RESERVED"]
    #[inline(always)]
    pub fn cfg_mask_reg3(&self) -> CfgMaskReg3R {
        CfgMaskReg3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_mask_reg3(&mut self) -> CfgMaskReg3W<CfgMaskReg3Spec> {
        CfgMaskReg3W::new(self, 0)
    }
}
#[doc = "CFG_MASK_REG3\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_mask_reg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_mask_reg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgMaskReg3Spec;
impl crate::RegisterSpec for CfgMaskReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_mask_reg3::R`](R) reader structure"]
impl crate::Readable for CfgMaskReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_mask_reg3::W`](W) writer structure"]
impl crate::Writable for CfgMaskReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_MASK_REG3 to value 0"]
impl crate::Resettable for CfgMaskReg3Spec {
    const RESET_VALUE: u32 = 0;
}
