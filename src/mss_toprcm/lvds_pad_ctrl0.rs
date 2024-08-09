#[doc = "Register `LVDS_PAD_CTRL0` reader"]
pub type R = crate::R<LvdsPadCtrl0Spec>;
#[doc = "Register `LVDS_PAD_CTRL0` writer"]
pub type W = crate::W<LvdsPadCtrl0Spec>;
#[doc = "Field `ctrl` reader - 31:0\\]
LVDS Pad Control 0 Register. Below is the mapping for each bit. Refer the LVDS IO Spec for more details Bit 0 : Power Down Control for LVDS CLK Lane Bit 1: LOPWRA Control for i LVDS CLK Lane Bit 2: LOPWRB Control for LVDS CLK Lane Bit 3 : LPSEL Control for LVDS CLK Lane Bit 4 : SUB_LVDS_EN Control for LVDS CLK Lane Bit 5 : HIZ_DISABLE Control for LVDS CLK Lane Bit 6 : EXT_RES_EN Control for LVDS CLK Lane Bit 7 : Reserved Bit 8 : Power Down Control for LVDS DATA Lane 0 Bit 9: LOPWRA Control for i LVDS DATA Lane 0 Bit 10: LOPWRB Control for LVDS DATA Lane 0 Bit 11: LPSEL Control for LVDS DATA Lane 0 Bit 12: SUB_LVDS_EN Control for LVDS DATA Lane 0 Bit 13: HIZ_DISABLE Control for LVDS DATA Lane 0 Bit 14: EXT_RES_EN Control for LVDS DATA Lane 0 Bit 15: Reserved Bit 16 : Power Down Control for LVDS DATA Lane 1 Bit 17: LOPWRA Control for i LVDS DATA Lane 1 Bit 18: LOPWRB Control for LVDS DATA Lane 1 Bit 18: LPSEL Control for LVDS DATA Lane 1 Bit 20: SUB_LVDS_EN Control for LVDS DATA Lane 1 Bit 21: HIZ_DISABLE Control for LVDS DATA Lane 1 Bit 22: EXT_RES_EN Control for LVDS DATA Lane 1 Bit 23: Reserved Bit 24 : Power Down Control for LVDS DATA Lane 2 Bit 25: LOPWRA Control for i LVDS DATA Lane 2 Bit 26: LOPWRB Control for LVDS DATA Lane 2 Bit 27: LPSEL Control for LVDS DATA Lane 2 Bit 28: SUB_LVDS_EN Control for LVDS DATA Lane 2 Bit 29: HIZ_DISABLE Control for LVDS DATA Lane 2 Bit 30: EXT_RES_EN Control for LVDS DATA Lane 2 Bit 31: Reserved"]
pub type CtrlR = crate::FieldReader<u32>;
#[doc = "Field `ctrl` writer - 31:0\\]
LVDS Pad Control 0 Register. Below is the mapping for each bit. Refer the LVDS IO Spec for more details Bit 0 : Power Down Control for LVDS CLK Lane Bit 1: LOPWRA Control for i LVDS CLK Lane Bit 2: LOPWRB Control for LVDS CLK Lane Bit 3 : LPSEL Control for LVDS CLK Lane Bit 4 : SUB_LVDS_EN Control for LVDS CLK Lane Bit 5 : HIZ_DISABLE Control for LVDS CLK Lane Bit 6 : EXT_RES_EN Control for LVDS CLK Lane Bit 7 : Reserved Bit 8 : Power Down Control for LVDS DATA Lane 0 Bit 9: LOPWRA Control for i LVDS DATA Lane 0 Bit 10: LOPWRB Control for LVDS DATA Lane 0 Bit 11: LPSEL Control for LVDS DATA Lane 0 Bit 12: SUB_LVDS_EN Control for LVDS DATA Lane 0 Bit 13: HIZ_DISABLE Control for LVDS DATA Lane 0 Bit 14: EXT_RES_EN Control for LVDS DATA Lane 0 Bit 15: Reserved Bit 16 : Power Down Control for LVDS DATA Lane 1 Bit 17: LOPWRA Control for i LVDS DATA Lane 1 Bit 18: LOPWRB Control for LVDS DATA Lane 1 Bit 18: LPSEL Control for LVDS DATA Lane 1 Bit 20: SUB_LVDS_EN Control for LVDS DATA Lane 1 Bit 21: HIZ_DISABLE Control for LVDS DATA Lane 1 Bit 22: EXT_RES_EN Control for LVDS DATA Lane 1 Bit 23: Reserved Bit 24 : Power Down Control for LVDS DATA Lane 2 Bit 25: LOPWRA Control for i LVDS DATA Lane 2 Bit 26: LOPWRB Control for LVDS DATA Lane 2 Bit 27: LPSEL Control for LVDS DATA Lane 2 Bit 28: SUB_LVDS_EN Control for LVDS DATA Lane 2 Bit 29: HIZ_DISABLE Control for LVDS DATA Lane 2 Bit 30: EXT_RES_EN Control for LVDS DATA Lane 2 Bit 31: Reserved"]
pub type CtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
LVDS Pad Control 0 Register. Below is the mapping for each bit. Refer the LVDS IO Spec for more details Bit 0 : Power Down Control for LVDS CLK Lane Bit 1: LOPWRA Control for i LVDS CLK Lane Bit 2: LOPWRB Control for LVDS CLK Lane Bit 3 : LPSEL Control for LVDS CLK Lane Bit 4 : SUB_LVDS_EN Control for LVDS CLK Lane Bit 5 : HIZ_DISABLE Control for LVDS CLK Lane Bit 6 : EXT_RES_EN Control for LVDS CLK Lane Bit 7 : Reserved Bit 8 : Power Down Control for LVDS DATA Lane 0 Bit 9: LOPWRA Control for i LVDS DATA Lane 0 Bit 10: LOPWRB Control for LVDS DATA Lane 0 Bit 11: LPSEL Control for LVDS DATA Lane 0 Bit 12: SUB_LVDS_EN Control for LVDS DATA Lane 0 Bit 13: HIZ_DISABLE Control for LVDS DATA Lane 0 Bit 14: EXT_RES_EN Control for LVDS DATA Lane 0 Bit 15: Reserved Bit 16 : Power Down Control for LVDS DATA Lane 1 Bit 17: LOPWRA Control for i LVDS DATA Lane 1 Bit 18: LOPWRB Control for LVDS DATA Lane 1 Bit 18: LPSEL Control for LVDS DATA Lane 1 Bit 20: SUB_LVDS_EN Control for LVDS DATA Lane 1 Bit 21: HIZ_DISABLE Control for LVDS DATA Lane 1 Bit 22: EXT_RES_EN Control for LVDS DATA Lane 1 Bit 23: Reserved Bit 24 : Power Down Control for LVDS DATA Lane 2 Bit 25: LOPWRA Control for i LVDS DATA Lane 2 Bit 26: LOPWRB Control for LVDS DATA Lane 2 Bit 27: LPSEL Control for LVDS DATA Lane 2 Bit 28: SUB_LVDS_EN Control for LVDS DATA Lane 2 Bit 29: HIZ_DISABLE Control for LVDS DATA Lane 2 Bit 30: EXT_RES_EN Control for LVDS DATA Lane 2 Bit 31: Reserved"]
    #[inline(always)]
    pub fn ctrl(&self) -> CtrlR {
        CtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
LVDS Pad Control 0 Register. Below is the mapping for each bit. Refer the LVDS IO Spec for more details Bit 0 : Power Down Control for LVDS CLK Lane Bit 1: LOPWRA Control for i LVDS CLK Lane Bit 2: LOPWRB Control for LVDS CLK Lane Bit 3 : LPSEL Control for LVDS CLK Lane Bit 4 : SUB_LVDS_EN Control for LVDS CLK Lane Bit 5 : HIZ_DISABLE Control for LVDS CLK Lane Bit 6 : EXT_RES_EN Control for LVDS CLK Lane Bit 7 : Reserved Bit 8 : Power Down Control for LVDS DATA Lane 0 Bit 9: LOPWRA Control for i LVDS DATA Lane 0 Bit 10: LOPWRB Control for LVDS DATA Lane 0 Bit 11: LPSEL Control for LVDS DATA Lane 0 Bit 12: SUB_LVDS_EN Control for LVDS DATA Lane 0 Bit 13: HIZ_DISABLE Control for LVDS DATA Lane 0 Bit 14: EXT_RES_EN Control for LVDS DATA Lane 0 Bit 15: Reserved Bit 16 : Power Down Control for LVDS DATA Lane 1 Bit 17: LOPWRA Control for i LVDS DATA Lane 1 Bit 18: LOPWRB Control for LVDS DATA Lane 1 Bit 18: LPSEL Control for LVDS DATA Lane 1 Bit 20: SUB_LVDS_EN Control for LVDS DATA Lane 1 Bit 21: HIZ_DISABLE Control for LVDS DATA Lane 1 Bit 22: EXT_RES_EN Control for LVDS DATA Lane 1 Bit 23: Reserved Bit 24 : Power Down Control for LVDS DATA Lane 2 Bit 25: LOPWRA Control for i LVDS DATA Lane 2 Bit 26: LOPWRB Control for LVDS DATA Lane 2 Bit 27: LPSEL Control for LVDS DATA Lane 2 Bit 28: SUB_LVDS_EN Control for LVDS DATA Lane 2 Bit 29: HIZ_DISABLE Control for LVDS DATA Lane 2 Bit 30: EXT_RES_EN Control for LVDS DATA Lane 2 Bit 31: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl(&mut self) -> CtrlW<LvdsPadCtrl0Spec> {
        CtrlW::new(self, 0)
    }
}
#[doc = "LVDS_PAD_CTRL0\n\nYou can [`read`](crate::Reg::read) this register and get [`lvds_pad_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvds_pad_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvdsPadCtrl0Spec;
impl crate::RegisterSpec for LvdsPadCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lvds_pad_ctrl0::R`](R) reader structure"]
impl crate::Readable for LvdsPadCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`lvds_pad_ctrl0::W`](W) writer structure"]
impl crate::Writable for LvdsPadCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LVDS_PAD_CTRL0 to value 0"]
impl crate::Resettable for LvdsPadCtrl0Spec {
    const RESET_VALUE: u32 = 0;
}
