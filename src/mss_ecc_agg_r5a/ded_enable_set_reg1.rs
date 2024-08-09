#[doc = "Register `ded_enable_set_reg1` reader"]
pub type R = crate::R<DedEnableSetReg1Spec>;
#[doc = "Register `ded_enable_set_reg1` writer"]
pub type W = crate::W<DedEnableSetReg1Spec>;
#[doc = "Field `INTERRUPT_ENABLE_SET_1` reader - 0:0\\]
Interrupt Enable Set Register for b1tcm1_bank0_pend"]
pub type InterruptEnableSet1R = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_SET_1` writer - 0:0\\]
Interrupt Enable Set Register for b1tcm1_bank0_pend"]
pub type InterruptEnableSet1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_SET` reader - 1:1\\]
Interrupt Enable Set Register for b1tcm1_bank1_pend"]
pub type InterruptEnableSetR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_SET` writer - 1:1\\]
Interrupt Enable Set Register for b1tcm1_bank1_pend"]
pub type InterruptEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for b1tcm1_bank0_pend"]
    #[inline(always)]
    pub fn interrupt_enable_set_1(&self) -> InterruptEnableSet1R {
        InterruptEnableSet1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for b1tcm1_bank1_pend"]
    #[inline(always)]
    pub fn interrupt_enable_set(&self) -> InterruptEnableSetR {
        InterruptEnableSetR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for b1tcm1_bank0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_set_1(&mut self) -> InterruptEnableSet1W<DedEnableSetReg1Spec> {
        InterruptEnableSet1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for b1tcm1_bank1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_set(&mut self) -> InterruptEnableSetW<DedEnableSetReg1Spec> {
        InterruptEnableSetW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Set Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_enable_set_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_enable_set_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DedEnableSetReg1Spec;
impl crate::RegisterSpec for DedEnableSetReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ded_enable_set_reg1::R`](R) reader structure"]
impl crate::Readable for DedEnableSetReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`ded_enable_set_reg1::W`](W) writer structure"]
impl crate::Writable for DedEnableSetReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ded_enable_set_reg1 to value 0"]
impl crate::Resettable for DedEnableSetReg1Spec {
    const RESET_VALUE: u32 = 0;
}
