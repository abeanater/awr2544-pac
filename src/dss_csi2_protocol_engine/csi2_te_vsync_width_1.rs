#[doc = "Register `CSI2_TE_VSYNC_WIDTH_1` reader"]
pub type R = crate::R<Csi2TeVsyncWidth1Spec>;
#[doc = "Register `CSI2_TE_VSYNC_WIDTH_1` writer"]
pub type W = crate::W<Csi2TeVsyncWidth1Spec>;
#[doc = "Field `MIN_VSYNC_PULSE_WIDTH` reader - 19:8\\]
Programmable min VSYNC pulse width Minimum VSYNC pulse width. Number of CSI2_CLK cycles times 256 to determine when VSYNC pulse occurs. The value 0 is invalid. The value shall be greater than MIN_HSYNC_PULSE_WIDTH when CSI2_TE_HSYNC_NUMBER is greather than 0"]
pub type MinVsyncPulseWidthR = crate::FieldReader<u16>;
#[doc = "Field `MIN_VSYNC_PULSE_WIDTH` writer - 19:8\\]
Programmable min VSYNC pulse width Minimum VSYNC pulse width. Number of CSI2_CLK cycles times 256 to determine when VSYNC pulse occurs. The value 0 is invalid. The value shall be greater than MIN_HSYNC_PULSE_WIDTH when CSI2_TE_HSYNC_NUMBER is greather than 0"]
pub type MinVsyncPulseWidthW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RESERVED1` reader - 31:20\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED1` writer - 31:20\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 8:19 - 19:8\\]
Programmable min VSYNC pulse width Minimum VSYNC pulse width. Number of CSI2_CLK cycles times 256 to determine when VSYNC pulse occurs. The value 0 is invalid. The value shall be greater than MIN_HSYNC_PULSE_WIDTH when CSI2_TE_HSYNC_NUMBER is greather than 0"]
    #[inline(always)]
    pub fn min_vsync_pulse_width(&self) -> MinVsyncPulseWidthR {
        MinVsyncPulseWidthR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:19 - 19:8\\]
Programmable min VSYNC pulse width Minimum VSYNC pulse width. Number of CSI2_CLK cycles times 256 to determine when VSYNC pulse occurs. The value 0 is invalid. The value shall be greater than MIN_HSYNC_PULSE_WIDTH when CSI2_TE_HSYNC_NUMBER is greather than 0"]
    #[inline(always)]
    #[must_use]
    pub fn min_vsync_pulse_width(&mut self) -> MinVsyncPulseWidthW<Csi2TeVsyncWidth1Spec> {
        MinVsyncPulseWidthW::new(self, 8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Csi2TeVsyncWidth1Spec> {
        Reserved1W::new(self, 20)
    }
}
#[doc = "The register configures the TE VSYNC minimum pulse width for TE0 and TE1 CMOS signals The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_te_vsync_width_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_te_vsync_width_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2TeVsyncWidth1Spec;
impl crate::RegisterSpec for Csi2TeVsyncWidth1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_te_vsync_width_1::R`](R) reader structure"]
impl crate::Readable for Csi2TeVsyncWidth1Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_te_vsync_width_1::W`](W) writer structure"]
impl crate::Writable for Csi2TeVsyncWidth1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_TE_VSYNC_WIDTH_1 to value 0"]
impl crate::Resettable for Csi2TeVsyncWidth1Spec {
    const RESET_VALUE: u32 = 0;
}
