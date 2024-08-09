#[doc = "Register `LVDS_PAD_CTRL1` reader"]
pub type R = crate::R<LvdsPadCtrl1Spec>;
#[doc = "Register `LVDS_PAD_CTRL1` writer"]
pub type W = crate::W<LvdsPadCtrl1Spec>;
#[doc = "Field `ctlr` reader - 31:0\\]
LVDS Pad Control 1 Register. Below is the mapping for each bit. Refer the LVDS IO Spec for more details Bit 0 : Power Down Control for LVDS DATA Lane 0 Bit 1: LOPWRA Control for i LVDS DATA Lane 0 Bit 2: LOPWRB Control for LVDS DATA Lane 0 Bit 3: LPSEL Control for LVDS DATA Lane 0 Bit 4: SUB_LVDS_EN Control for LVDS DATA Lane 0 Bit 5: HIZ_DISABLE Control for LVDS DATA Lane 0 Bit 6: EXT_RES_EN Control for LVDS DATA Lane 0 Bit 7: Reserved Bit 8 : Power Down Control for LVDS FRME CLK Lane Bit 9 : LOPWRA Control for i LVDS FRAME CLK Lane Bit 10: LOPWRB Control for LVDS FRAME CLK Lane Bit 11 : LPSEL Control for LVDS FRAME CLK Lane Bit 12 : SUB_LVDS_EN Control for LVDS FRAME CLK Lane Bit 13 : HIZ_DISABLE Control for LVDS FRAME CLK Lane Bit 14 : EXT_RES_EN Control for LVDS FRAME CLK Lane Bit 15 -23: Reserved Bit 24 : Power Down Control for LVDS Bias cell Bit 25 : eEfuse Set Control for LVDS Bias cell"]
pub type CtlrR = crate::FieldReader<u32>;
#[doc = "Field `ctlr` writer - 31:0\\]
LVDS Pad Control 1 Register. Below is the mapping for each bit. Refer the LVDS IO Spec for more details Bit 0 : Power Down Control for LVDS DATA Lane 0 Bit 1: LOPWRA Control for i LVDS DATA Lane 0 Bit 2: LOPWRB Control for LVDS DATA Lane 0 Bit 3: LPSEL Control for LVDS DATA Lane 0 Bit 4: SUB_LVDS_EN Control for LVDS DATA Lane 0 Bit 5: HIZ_DISABLE Control for LVDS DATA Lane 0 Bit 6: EXT_RES_EN Control for LVDS DATA Lane 0 Bit 7: Reserved Bit 8 : Power Down Control for LVDS FRME CLK Lane Bit 9 : LOPWRA Control for i LVDS FRAME CLK Lane Bit 10: LOPWRB Control for LVDS FRAME CLK Lane Bit 11 : LPSEL Control for LVDS FRAME CLK Lane Bit 12 : SUB_LVDS_EN Control for LVDS FRAME CLK Lane Bit 13 : HIZ_DISABLE Control for LVDS FRAME CLK Lane Bit 14 : EXT_RES_EN Control for LVDS FRAME CLK Lane Bit 15 -23: Reserved Bit 24 : Power Down Control for LVDS Bias cell Bit 25 : eEfuse Set Control for LVDS Bias cell"]
pub type CtlrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
LVDS Pad Control 1 Register. Below is the mapping for each bit. Refer the LVDS IO Spec for more details Bit 0 : Power Down Control for LVDS DATA Lane 0 Bit 1: LOPWRA Control for i LVDS DATA Lane 0 Bit 2: LOPWRB Control for LVDS DATA Lane 0 Bit 3: LPSEL Control for LVDS DATA Lane 0 Bit 4: SUB_LVDS_EN Control for LVDS DATA Lane 0 Bit 5: HIZ_DISABLE Control for LVDS DATA Lane 0 Bit 6: EXT_RES_EN Control for LVDS DATA Lane 0 Bit 7: Reserved Bit 8 : Power Down Control for LVDS FRME CLK Lane Bit 9 : LOPWRA Control for i LVDS FRAME CLK Lane Bit 10: LOPWRB Control for LVDS FRAME CLK Lane Bit 11 : LPSEL Control for LVDS FRAME CLK Lane Bit 12 : SUB_LVDS_EN Control for LVDS FRAME CLK Lane Bit 13 : HIZ_DISABLE Control for LVDS FRAME CLK Lane Bit 14 : EXT_RES_EN Control for LVDS FRAME CLK Lane Bit 15 -23: Reserved Bit 24 : Power Down Control for LVDS Bias cell Bit 25 : eEfuse Set Control for LVDS Bias cell"]
    #[inline(always)]
    pub fn ctlr(&self) -> CtlrR {
        CtlrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
LVDS Pad Control 1 Register. Below is the mapping for each bit. Refer the LVDS IO Spec for more details Bit 0 : Power Down Control for LVDS DATA Lane 0 Bit 1: LOPWRA Control for i LVDS DATA Lane 0 Bit 2: LOPWRB Control for LVDS DATA Lane 0 Bit 3: LPSEL Control for LVDS DATA Lane 0 Bit 4: SUB_LVDS_EN Control for LVDS DATA Lane 0 Bit 5: HIZ_DISABLE Control for LVDS DATA Lane 0 Bit 6: EXT_RES_EN Control for LVDS DATA Lane 0 Bit 7: Reserved Bit 8 : Power Down Control for LVDS FRME CLK Lane Bit 9 : LOPWRA Control for i LVDS FRAME CLK Lane Bit 10: LOPWRB Control for LVDS FRAME CLK Lane Bit 11 : LPSEL Control for LVDS FRAME CLK Lane Bit 12 : SUB_LVDS_EN Control for LVDS FRAME CLK Lane Bit 13 : HIZ_DISABLE Control for LVDS FRAME CLK Lane Bit 14 : EXT_RES_EN Control for LVDS FRAME CLK Lane Bit 15 -23: Reserved Bit 24 : Power Down Control for LVDS Bias cell Bit 25 : eEfuse Set Control for LVDS Bias cell"]
    #[inline(always)]
    #[must_use]
    pub fn ctlr(&mut self) -> CtlrW<LvdsPadCtrl1Spec> {
        CtlrW::new(self, 0)
    }
}
#[doc = "LVDS_PAD_CTRL1\n\nYou can [`read`](crate::Reg::read) this register and get [`lvds_pad_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvds_pad_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvdsPadCtrl1Spec;
impl crate::RegisterSpec for LvdsPadCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lvds_pad_ctrl1::R`](R) reader structure"]
impl crate::Readable for LvdsPadCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`lvds_pad_ctrl1::W`](W) writer structure"]
impl crate::Writable for LvdsPadCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LVDS_PAD_CTRL1 to value 0"]
impl crate::Resettable for LvdsPadCtrl1Spec {
    const RESET_VALUE: u32 = 0;
}
