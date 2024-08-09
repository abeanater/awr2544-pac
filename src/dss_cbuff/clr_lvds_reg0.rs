#[doc = "Register `CLR_LVDS_REG0` reader"]
pub type R = crate::R<ClrLvdsReg0Spec>;
#[doc = "Register `CLR_LVDS_REG0` writer"]
pub type W = crate::W<ClrLvdsReg0Spec>;
#[doc = "Field `CLR_LVDS_REG0` reader - 31:0\\]
TI Internal Feature.Clear Register field corresponding to STAT_LVDS_REG0. Write 0x1 to Clear the field"]
pub type ClrLvdsReg0R = crate::FieldReader<u32>;
#[doc = "Field `CLR_LVDS_REG0` writer - 31:0\\]
TI Internal Feature.Clear Register field corresponding to STAT_LVDS_REG0. Write 0x1 to Clear the field"]
pub type ClrLvdsReg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TI Internal Feature.Clear Register field corresponding to STAT_LVDS_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn clr_lvds_reg0(&self) -> ClrLvdsReg0R {
        ClrLvdsReg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TI Internal Feature.Clear Register field corresponding to STAT_LVDS_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn clr_lvds_reg0(&mut self) -> ClrLvdsReg0W<ClrLvdsReg0Spec> {
        ClrLvdsReg0W::new(self, 0)
    }
}
#[doc = "CLR_LVDS_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_lvds_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_lvds_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrLvdsReg0Spec;
impl crate::RegisterSpec for ClrLvdsReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_lvds_reg0::R`](R) reader structure"]
impl crate::Readable for ClrLvdsReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`clr_lvds_reg0::W`](W) writer structure"]
impl crate::Writable for ClrLvdsReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR_LVDS_REG0 to value 0"]
impl crate::Resettable for ClrLvdsReg0Spec {
    const RESET_VALUE: u32 = 0;
}
