#[doc = "Register `sec_enable_set_reg0` reader"]
pub type R = crate::R<SecEnableSetReg0Spec>;
#[doc = "Register `sec_enable_set_reg0` writer"]
pub type W = crate::W<SecEnableSetReg0Spec>;
#[doc = "Field `INTERRUPT_ENABLE_SET_7` reader - 0:0\\]
Interrupt Enable Set Register for mss_l2slv0_pend"]
pub type InterruptEnableSet7R = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_SET_7` writer - 0:0\\]
Interrupt Enable Set Register for mss_l2slv0_pend"]
pub type InterruptEnableSet7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_SET_6` reader - 1:1\\]
Interrupt Enable Set Register for mss_l2slv1_pend"]
pub type InterruptEnableSet6R = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_SET_6` writer - 1:1\\]
Interrupt Enable Set Register for mss_l2slv1_pend"]
pub type InterruptEnableSet6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_SET_5` reader - 2:2\\]
Interrupt Enable Set Register for mss_mbox_pend"]
pub type InterruptEnableSet5R = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_SET_5` writer - 2:2\\]
Interrupt Enable Set Register for mss_mbox_pend"]
pub type InterruptEnableSet5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_SET_4` reader - 3:3\\]
Interrupt Enable Set Register for mss_retram_pend"]
pub type InterruptEnableSet4R = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_SET_4` writer - 3:3\\]
Interrupt Enable Set Register for mss_retram_pend"]
pub type InterruptEnableSet4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_SET_3` reader - 4:4\\]
Interrupt Enable Set Register for gpadc_pend"]
pub type InterruptEnableSet3R = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_SET_3` writer - 4:4\\]
Interrupt Enable Set Register for gpadc_pend"]
pub type InterruptEnableSet3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_SET_2` reader - 5:5\\]
Interrupt Enable Set Register for tptc_a0_pend"]
pub type InterruptEnableSet2R = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_SET_2` writer - 5:5\\]
Interrupt Enable Set Register for tptc_a0_pend"]
pub type InterruptEnableSet2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_SET_1` reader - 6:6\\]
Interrupt Enable Set Register for tptc_a1_pend"]
pub type InterruptEnableSet1R = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_SET_1` writer - 6:6\\]
Interrupt Enable Set Register for tptc_a1_pend"]
pub type InterruptEnableSet1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_SET` reader - 7:7\\]
Interrupt Enable Set Register for mss_l2_bankc_pend"]
pub type InterruptEnableSetR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_SET` writer - 7:7\\]
Interrupt Enable Set Register for mss_l2_bankc_pend"]
pub type InterruptEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for mss_l2slv0_pend"]
    #[inline(always)]
    pub fn interrupt_enable_set_7(&self) -> InterruptEnableSet7R {
        InterruptEnableSet7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for mss_l2slv1_pend"]
    #[inline(always)]
    pub fn interrupt_enable_set_6(&self) -> InterruptEnableSet6R {
        InterruptEnableSet6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Set Register for mss_mbox_pend"]
    #[inline(always)]
    pub fn interrupt_enable_set_5(&self) -> InterruptEnableSet5R {
        InterruptEnableSet5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Set Register for mss_retram_pend"]
    #[inline(always)]
    pub fn interrupt_enable_set_4(&self) -> InterruptEnableSet4R {
        InterruptEnableSet4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Set Register for gpadc_pend"]
    #[inline(always)]
    pub fn interrupt_enable_set_3(&self) -> InterruptEnableSet3R {
        InterruptEnableSet3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Set Register for tptc_a0_pend"]
    #[inline(always)]
    pub fn interrupt_enable_set_2(&self) -> InterruptEnableSet2R {
        InterruptEnableSet2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Set Register for tptc_a1_pend"]
    #[inline(always)]
    pub fn interrupt_enable_set_1(&self) -> InterruptEnableSet1R {
        InterruptEnableSet1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Set Register for mss_l2_bankc_pend"]
    #[inline(always)]
    pub fn interrupt_enable_set(&self) -> InterruptEnableSetR {
        InterruptEnableSetR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for mss_l2slv0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_set_7(&mut self) -> InterruptEnableSet7W<SecEnableSetReg0Spec> {
        InterruptEnableSet7W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for mss_l2slv1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_set_6(&mut self) -> InterruptEnableSet6W<SecEnableSetReg0Spec> {
        InterruptEnableSet6W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Set Register for mss_mbox_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_set_5(&mut self) -> InterruptEnableSet5W<SecEnableSetReg0Spec> {
        InterruptEnableSet5W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Set Register for mss_retram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_set_4(&mut self) -> InterruptEnableSet4W<SecEnableSetReg0Spec> {
        InterruptEnableSet4W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Set Register for gpadc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_set_3(&mut self) -> InterruptEnableSet3W<SecEnableSetReg0Spec> {
        InterruptEnableSet3W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Set Register for tptc_a0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_set_2(&mut self) -> InterruptEnableSet2W<SecEnableSetReg0Spec> {
        InterruptEnableSet2W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Set Register for tptc_a1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_set_1(&mut self) -> InterruptEnableSet1W<SecEnableSetReg0Spec> {
        InterruptEnableSet1W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Set Register for mss_l2_bankc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_set(&mut self) -> InterruptEnableSetW<SecEnableSetReg0Spec> {
        InterruptEnableSetW::new(self, 7)
    }
}
#[doc = "Interrupt Enable Set Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_set_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_set_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnableSetReg0Spec;
impl crate::RegisterSpec for SecEnableSetReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_enable_set_reg0::R`](R) reader structure"]
impl crate::Readable for SecEnableSetReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_enable_set_reg0::W`](W) writer structure"]
impl crate::Writable for SecEnableSetReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sec_enable_set_reg0 to value 0"]
impl crate::Resettable for SecEnableSetReg0Spec {
    const RESET_VALUE: u32 = 0;
}
