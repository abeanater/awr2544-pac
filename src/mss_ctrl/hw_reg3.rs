#[doc = "Register `HW_REG3` reader"]
pub type R = crate::R<HwReg3Spec>;
#[doc = "Register `HW_REG3` writer"]
pub type W = crate::W<HwReg3Spec>;
#[doc = "Field `ro` reader - 0:0\\]
This denotes the status of ping_pong_sel bit."]
pub type RoR = crate::BitReader;
#[doc = "Field `ro` writer - 0:0\\]
This denotes the status of ping_pong_sel bit."]
pub type RoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPSW_CLK_STOP_ACK` reader - 4:4\\]
CPSW clock stop ack"]
pub type CpswClkStopAckR = crate::BitReader;
#[doc = "Field `CPSW_CLK_STOP_ACK` writer - 4:4\\]
CPSW clock stop ack"]
pub type CpswClkStopAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPSW_CLK_STOP_IDLE` reader - 8:8\\]
CPSW clock stop idle"]
pub type CpswClkStopIdleR = crate::BitReader;
#[doc = "Field `CPSW_CLK_STOP_IDLE` writer - 8:8\\]
CPSW clock stop idle"]
pub type CpswClkStopIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwreg3` reader - 31:9\\]
HW reserved Register"]
pub type Hwreg3R = crate::FieldReader<u32>;
#[doc = "Field `hwreg3` writer - 31:9\\]
HW reserved Register"]
pub type Hwreg3W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This denotes the status of ping_pong_sel bit."]
    #[inline(always)]
    pub fn ro(&self) -> RoR {
        RoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
CPSW clock stop ack"]
    #[inline(always)]
    pub fn cpsw_clk_stop_ack(&self) -> CpswClkStopAckR {
        CpswClkStopAckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
CPSW clock stop idle"]
    #[inline(always)]
    pub fn cpsw_clk_stop_idle(&self) -> CpswClkStopIdleR {
        CpswClkStopIdleR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
HW reserved Register"]
    #[inline(always)]
    pub fn hwreg3(&self) -> Hwreg3R {
        Hwreg3R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This denotes the status of ping_pong_sel bit."]
    #[inline(always)]
    #[must_use]
    pub fn ro(&mut self) -> RoW<HwReg3Spec> {
        RoW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
CPSW clock stop ack"]
    #[inline(always)]
    #[must_use]
    pub fn cpsw_clk_stop_ack(&mut self) -> CpswClkStopAckW<HwReg3Spec> {
        CpswClkStopAckW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
CPSW clock stop idle"]
    #[inline(always)]
    #[must_use]
    pub fn cpsw_clk_stop_idle(&mut self) -> CpswClkStopIdleW<HwReg3Spec> {
        CpswClkStopIdleW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
HW reserved Register"]
    #[inline(always)]
    #[must_use]
    pub fn hwreg3(&mut self) -> Hwreg3W<HwReg3Spec> {
        Hwreg3W::new(self, 9)
    }
}
#[doc = "HW_REG3\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwReg3Spec;
impl crate::RegisterSpec for HwReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_reg3::R`](R) reader structure"]
impl crate::Readable for HwReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_reg3::W`](W) writer structure"]
impl crate::Writable for HwReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_REG3 to value 0"]
impl crate::Resettable for HwReg3Spec {
    const RESET_VALUE: u32 = 0;
}
