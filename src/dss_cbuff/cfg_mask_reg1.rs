#[doc = "Register `CFG_MASK_REG1` reader"]
pub type R = crate::R<CfgMaskReg1Spec>;
#[doc = "Register `CFG_MASK_REG1` writer"]
pub type W = crate::W<CfgMaskReg1Spec>;
#[doc = "Field `CFG_MASK_REG1` reader - 31:0\\]
Mask Register field corresponding to STAT_CBUFF_REG1. Refer STAT_CBUFF_REG1 for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
pub type CfgMaskReg1R = crate::FieldReader<u32>;
#[doc = "Field `CFG_MASK_REG1` writer - 31:0\\]
Mask Register field corresponding to STAT_CBUFF_REG1. Refer STAT_CBUFF_REG1 for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
pub type CfgMaskReg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Mask Register field corresponding to STAT_CBUFF_REG1. Refer STAT_CBUFF_REG1 for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
    #[inline(always)]
    pub fn cfg_mask_reg1(&self) -> CfgMaskReg1R {
        CfgMaskReg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Mask Register field corresponding to STAT_CBUFF_REG1. Refer STAT_CBUFF_REG1 for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_mask_reg1(&mut self) -> CfgMaskReg1W<CfgMaskReg1Spec> {
        CfgMaskReg1W::new(self, 0)
    }
}
#[doc = "CFG_MASK_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_mask_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_mask_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgMaskReg1Spec;
impl crate::RegisterSpec for CfgMaskReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_mask_reg1::R`](R) reader structure"]
impl crate::Readable for CfgMaskReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_mask_reg1::W`](W) writer structure"]
impl crate::Writable for CfgMaskReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_MASK_REG1 to value 0"]
impl crate::Resettable for CfgMaskReg1Spec {
    const RESET_VALUE: u32 = 0;
}
