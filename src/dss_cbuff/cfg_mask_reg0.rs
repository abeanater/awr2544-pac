#[doc = "Register `CFG_MASK_REG0` reader"]
pub type R = crate::R<CfgMaskReg0Spec>;
#[doc = "Register `CFG_MASK_REG0` writer"]
pub type W = crate::W<CfgMaskReg0Spec>;
#[doc = "Field `CFG_MASK_REG0` reader - 31:0\\]
Mask Register field corresponding to STAT_CBUFF_REG0. Refer STAT_CBUFF_REG0 for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
pub type CfgMaskReg0R = crate::FieldReader<u32>;
#[doc = "Field `CFG_MASK_REG0` writer - 31:0\\]
Mask Register field corresponding to STAT_CBUFF_REG0. Refer STAT_CBUFF_REG0 for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
pub type CfgMaskReg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Mask Register field corresponding to STAT_CBUFF_REG0. Refer STAT_CBUFF_REG0 for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
    #[inline(always)]
    pub fn cfg_mask_reg0(&self) -> CfgMaskReg0R {
        CfgMaskReg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Mask Register field corresponding to STAT_CBUFF_REG0. Refer STAT_CBUFF_REG0 for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_mask_reg0(&mut self) -> CfgMaskReg0W<CfgMaskReg0Spec> {
        CfgMaskReg0W::new(self, 0)
    }
}
#[doc = "CFG_MASK_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_mask_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_mask_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgMaskReg0Spec;
impl crate::RegisterSpec for CfgMaskReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_mask_reg0::R`](R) reader structure"]
impl crate::Readable for CfgMaskReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_mask_reg0::W`](W) writer structure"]
impl crate::Writable for CfgMaskReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_MASK_REG0 to value 0"]
impl crate::Resettable for CfgMaskReg0Spec {
    const RESET_VALUE: u32 = 0;
}
