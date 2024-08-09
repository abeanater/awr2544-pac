#[doc = "Register `CSI2_TE_HSYNC_WIDTH_1` reader"]
pub type R = crate::R<Csi2TeHsyncWidth1Spec>;
#[doc = "Register `CSI2_TE_HSYNC_WIDTH_1` writer"]
pub type W = crate::W<Csi2TeHsyncWidth1Spec>;
#[doc = "Field `MIN_HSYNC_PULSE_WIDTH` reader - 19:8\\]
Programmable min HSYNC pulse width Minimum HSYNC pulse width. Number of CSI2_CLK clock cycles times 256 to determine when HSYNC pulse occurs. The value 0 is invalid."]
pub type MinHsyncPulseWidthR = crate::FieldReader<u16>;
#[doc = "Field `MIN_HSYNC_PULSE_WIDTH` writer - 19:8\\]
Programmable min HSYNC pulse width Minimum HSYNC pulse width. Number of CSI2_CLK clock cycles times 256 to determine when HSYNC pulse occurs. The value 0 is invalid."]
pub type MinHsyncPulseWidthW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RESERVED1` reader - 31:20\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED1` writer - 31:20\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 8:19 - 19:8\\]
Programmable min HSYNC pulse width Minimum HSYNC pulse width. Number of CSI2_CLK clock cycles times 256 to determine when HSYNC pulse occurs. The value 0 is invalid."]
    #[inline(always)]
    pub fn min_hsync_pulse_width(&self) -> MinHsyncPulseWidthR {
        MinHsyncPulseWidthR::new(((self.bits >> 8) & 0x0fff) as u16)
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
Programmable min HSYNC pulse width Minimum HSYNC pulse width. Number of CSI2_CLK clock cycles times 256 to determine when HSYNC pulse occurs. The value 0 is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn min_hsync_pulse_width(&mut self) -> MinHsyncPulseWidthW<Csi2TeHsyncWidth1Spec> {
        MinHsyncPulseWidthW::new(self, 8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Csi2TeHsyncWidth1Spec> {
        Reserved1W::new(self, 20)
    }
}
#[doc = "The register configures the TE HSYNC minimum pulse width for TE0 and TE1 CMOS signals The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_te_hsync_width_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_te_hsync_width_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2TeHsyncWidth1Spec;
impl crate::RegisterSpec for Csi2TeHsyncWidth1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_te_hsync_width_1::R`](R) reader structure"]
impl crate::Readable for Csi2TeHsyncWidth1Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_te_hsync_width_1::W`](W) writer structure"]
impl crate::Writable for Csi2TeHsyncWidth1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_TE_HSYNC_WIDTH_1 to value 0"]
impl crate::Resettable for Csi2TeHsyncWidth1Spec {
    const RESET_VALUE: u32 = 0;
}
