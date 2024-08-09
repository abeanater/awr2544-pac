#[doc = "Register `sec_status_reg0` reader"]
pub type R = crate::R<SecStatusReg0Spec>;
#[doc = "Register `sec_status_reg0` writer"]
pub type W = crate::W<SecStatusReg0Spec>;
#[doc = "Field `INTERRUPT_PENDING_STATUS_7` reader - 0:0\\]
Interrupt Pending Status for mss_l2slv0_pend"]
pub type InterruptPendingStatus7R = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS_7` writer - 0:0\\]
Interrupt Pending Status for mss_l2slv0_pend"]
pub type InterruptPendingStatus7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_PENDING_STATUS_6` reader - 1:1\\]
Interrupt Pending Status for mss_l2slv1_pend"]
pub type InterruptPendingStatus6R = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS_6` writer - 1:1\\]
Interrupt Pending Status for mss_l2slv1_pend"]
pub type InterruptPendingStatus6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_PENDING_STATUS_5` reader - 2:2\\]
Interrupt Pending Status for mss_mbox_pend"]
pub type InterruptPendingStatus5R = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS_5` writer - 2:2\\]
Interrupt Pending Status for mss_mbox_pend"]
pub type InterruptPendingStatus5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_PENDING_STATUS_4` reader - 3:3\\]
Interrupt Pending Status for mss_retram_pend"]
pub type InterruptPendingStatus4R = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS_4` writer - 3:3\\]
Interrupt Pending Status for mss_retram_pend"]
pub type InterruptPendingStatus4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_PENDING_STATUS_3` reader - 4:4\\]
Interrupt Pending Status for gpadc_pend"]
pub type InterruptPendingStatus3R = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS_3` writer - 4:4\\]
Interrupt Pending Status for gpadc_pend"]
pub type InterruptPendingStatus3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_PENDING_STATUS_2` reader - 5:5\\]
Interrupt Pending Status for tptc_a0_pend"]
pub type InterruptPendingStatus2R = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS_2` writer - 5:5\\]
Interrupt Pending Status for tptc_a0_pend"]
pub type InterruptPendingStatus2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_PENDING_STATUS_1` reader - 6:6\\]
Interrupt Pending Status for tptc_a1_pend"]
pub type InterruptPendingStatus1R = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS_1` writer - 6:6\\]
Interrupt Pending Status for tptc_a1_pend"]
pub type InterruptPendingStatus1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_PENDING_STATUS` reader - 7:7\\]
Interrupt Pending Status for mss_l2_bankc_pend"]
pub type InterruptPendingStatusR = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS` writer - 7:7\\]
Interrupt Pending Status for mss_l2_bankc_pend"]
pub type InterruptPendingStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for mss_l2slv0_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status_7(&self) -> InterruptPendingStatus7R {
        InterruptPendingStatus7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for mss_l2slv1_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status_6(&self) -> InterruptPendingStatus6R {
        InterruptPendingStatus6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for mss_mbox_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status_5(&self) -> InterruptPendingStatus5R {
        InterruptPendingStatus5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for mss_retram_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status_4(&self) -> InterruptPendingStatus4R {
        InterruptPendingStatus4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Pending Status for gpadc_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status_3(&self) -> InterruptPendingStatus3R {
        InterruptPendingStatus3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Pending Status for tptc_a0_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status_2(&self) -> InterruptPendingStatus2R {
        InterruptPendingStatus2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Pending Status for tptc_a1_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status_1(&self) -> InterruptPendingStatus1R {
        InterruptPendingStatus1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Pending Status for mss_l2_bankc_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status(&self) -> InterruptPendingStatusR {
        InterruptPendingStatusR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for mss_l2slv0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status_7(&mut self) -> InterruptPendingStatus7W<SecStatusReg0Spec> {
        InterruptPendingStatus7W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for mss_l2slv1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status_6(&mut self) -> InterruptPendingStatus6W<SecStatusReg0Spec> {
        InterruptPendingStatus6W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for mss_mbox_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status_5(&mut self) -> InterruptPendingStatus5W<SecStatusReg0Spec> {
        InterruptPendingStatus5W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for mss_retram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status_4(&mut self) -> InterruptPendingStatus4W<SecStatusReg0Spec> {
        InterruptPendingStatus4W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Pending Status for gpadc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status_3(&mut self) -> InterruptPendingStatus3W<SecStatusReg0Spec> {
        InterruptPendingStatus3W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Pending Status for tptc_a0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status_2(&mut self) -> InterruptPendingStatus2W<SecStatusReg0Spec> {
        InterruptPendingStatus2W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Pending Status for tptc_a1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status_1(&mut self) -> InterruptPendingStatus1W<SecStatusReg0Spec> {
        InterruptPendingStatus1W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Pending Status for mss_l2_bankc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status(&mut self) -> InterruptPendingStatusW<SecStatusReg0Spec> {
        InterruptPendingStatusW::new(self, 7)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_status_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_status_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecStatusReg0Spec;
impl crate::RegisterSpec for SecStatusReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_status_reg0::R`](R) reader structure"]
impl crate::Readable for SecStatusReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_status_reg0::W`](W) writer structure"]
impl crate::Writable for SecStatusReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sec_status_reg0 to value 0"]
impl crate::Resettable for SecStatusReg0Spec {
    const RESET_VALUE: u32 = 0;
}
