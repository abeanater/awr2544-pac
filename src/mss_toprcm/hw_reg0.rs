#[doc = "Register `HW_REG0` reader"]
pub type R = crate::R<HwReg0Spec>;
#[doc = "Register `HW_REG0` writer"]
pub type W = crate::W<HwReg0Spec>;
#[doc = "Field `hwreg` reader - 31:0\\]
HW Reserved regiser. Reserved for HW RnD"]
pub type HwregR = crate::FieldReader<u32>;
#[doc = "Field `hwreg` writer - 31:0\\]
HW Reserved regiser. Reserved for HW RnD"]
pub type HwregW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
HW Reserved regiser. Reserved for HW RnD"]
    #[inline(always)]
    pub fn hwreg(&self) -> HwregR {
        HwregR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
HW Reserved regiser. Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn hwreg(&mut self) -> HwregW<HwReg0Spec> {
        HwregW::new(self, 0)
    }
}
#[doc = "HW_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwReg0Spec;
impl crate::RegisterSpec for HwReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_reg0::R`](R) reader structure"]
impl crate::Readable for HwReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_reg0::W`](W) writer structure"]
impl crate::Writable for HwReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_REG0 to value 0"]
impl crate::Resettable for HwReg0Spec {
    const RESET_VALUE: u32 = 0;
}
