#[doc = "Register `STAT_LVDS_REG3` reader"]
pub type R = crate::R<StatLvdsReg3Spec>;
#[doc = "Register `STAT_LVDS_REG3` writer"]
pub type W = crate::W<StatLvdsReg3Spec>;
#[doc = "Field `STAT_LVDS_REG3` reader - 31:0\\]
RESERVED"]
pub type StatLvdsReg3R = crate::FieldReader<u32>;
#[doc = "Field `STAT_LVDS_REG3` writer - 31:0\\]
RESERVED"]
pub type StatLvdsReg3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
RESERVED"]
    #[inline(always)]
    pub fn stat_lvds_reg3(&self) -> StatLvdsReg3R {
        StatLvdsReg3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn stat_lvds_reg3(&mut self) -> StatLvdsReg3W<StatLvdsReg3Spec> {
        StatLvdsReg3W::new(self, 0)
    }
}
#[doc = "STAT_LVDS_REG3\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_lvds_reg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_lvds_reg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatLvdsReg3Spec;
impl crate::RegisterSpec for StatLvdsReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_lvds_reg3::R`](R) reader structure"]
impl crate::Readable for StatLvdsReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`stat_lvds_reg3::W`](W) writer structure"]
impl crate::Writable for StatLvdsReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT_LVDS_REG3 to value 0"]
impl crate::Resettable for StatLvdsReg3Spec {
    const RESET_VALUE: u32 = 0;
}
