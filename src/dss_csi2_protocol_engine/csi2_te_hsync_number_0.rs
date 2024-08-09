#[doc = "Register `CSI2_TE_HSYNC_NUMBER_0` reader"]
pub type R = crate::R<Csi2TeHsyncNumber0Spec>;
#[doc = "Register `CSI2_TE_HSYNC_NUMBER_0` writer"]
pub type W = crate::W<Csi2TeHsyncNumber0Spec>;
#[doc = "Field `LINE_NUMBER` reader - 10:0\\]
Programmable line number Line number from 0 to 2047. Number of HSYNC after the VSYNC occurs before the beginning of the transfer. Any HSYNC before VSYNC is ignored."]
pub type LineNumberR = crate::FieldReader<u16>;
#[doc = "Field `LINE_NUMBER` writer - 10:0\\]
Programmable line number Line number from 0 to 2047. Number of HSYNC after the VSYNC occurs before the beginning of the transfer. Any HSYNC before VSYNC is ignored."]
pub type LineNumberW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `RESERVED_18` reader - "]
pub type Reserved18R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_18` writer - "]
pub type Reserved18W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Programmable line number Line number from 0 to 2047. Number of HSYNC after the VSYNC occurs before the beginning of the transfer. Any HSYNC before VSYNC is ignored."]
    #[inline(always)]
    pub fn line_number(&self) -> LineNumberR {
        LineNumberR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31"]
    #[inline(always)]
    pub fn reserved_18(&self) -> Reserved18R {
        Reserved18R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Programmable line number Line number from 0 to 2047. Number of HSYNC after the VSYNC occurs before the beginning of the transfer. Any HSYNC before VSYNC is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn line_number(&mut self) -> LineNumberW<Csi2TeHsyncNumber0Spec> {
        LineNumberW::new(self, 0)
    }
    #[doc = "Bits 11:31"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_18(&mut self) -> Reserved18W<Csi2TeHsyncNumber0Spec> {
        Reserved18W::new(self, 11)
    }
}
#[doc = "The register configures the number of HSYNC to synchronize the beginning of the transfer on CSI2 link based on the number of HSYNC pulse received on the TE line. The input TE signal is asynchronous and needs to be re-synchronizred to CSI2_CLK clock domain.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_te_hsync_number_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_te_hsync_number_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2TeHsyncNumber0Spec;
impl crate::RegisterSpec for Csi2TeHsyncNumber0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_te_hsync_number_0::R`](R) reader structure"]
impl crate::Readable for Csi2TeHsyncNumber0Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_te_hsync_number_0::W`](W) writer structure"]
impl crate::Writable for Csi2TeHsyncNumber0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_TE_HSYNC_NUMBER_0 to value 0"]
impl crate::Resettable for Csi2TeHsyncNumber0Spec {
    const RESET_VALUE: u32 = 0;
}
