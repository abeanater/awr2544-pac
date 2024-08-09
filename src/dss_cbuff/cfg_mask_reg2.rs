#[doc = "Register `CFG_MASK_REG2` reader"]
pub type R = crate::R<CfgMaskReg2Spec>;
#[doc = "Register `CFG_MASK_REG2` writer"]
pub type W = crate::W<CfgMaskReg2Spec>;
#[doc = "Field `CFG_MASK_REG2` reader - 31:0\\]
Mask Register field corresponding to STAT_LVDS_REG0. Refer STAT_LVDS_REG0 for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
pub type CfgMaskReg2R = crate::FieldReader<u32>;
#[doc = "Field `CFG_MASK_REG2` writer - 31:0\\]
Mask Register field corresponding to STAT_LVDS_REG0. Refer STAT_LVDS_REG0 for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
pub type CfgMaskReg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Mask Register field corresponding to STAT_LVDS_REG0. Refer STAT_LVDS_REG0 for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
    #[inline(always)]
    pub fn cfg_mask_reg2(&self) -> CfgMaskReg2R {
        CfgMaskReg2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Mask Register field corresponding to STAT_LVDS_REG0. Refer STAT_LVDS_REG0 for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_mask_reg2(&mut self) -> CfgMaskReg2W<CfgMaskReg2Spec> {
        CfgMaskReg2W::new(self, 0)
    }
}
#[doc = "CFG_MASK_REG2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_mask_reg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_mask_reg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgMaskReg2Spec;
impl crate::RegisterSpec for CfgMaskReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_mask_reg2::R`](R) reader structure"]
impl crate::Readable for CfgMaskReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_mask_reg2::W`](W) writer structure"]
impl crate::Writable for CfgMaskReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_MASK_REG2 to value 0"]
impl crate::Resettable for CfgMaskReg2Spec {
    const RESET_VALUE: u32 = 0;
}
