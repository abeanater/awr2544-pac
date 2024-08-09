#[doc = "Register `ANA_REG_CLK_STATUS_REG` reader"]
pub type R = crate::R<AnaRegClkStatusRegSpec>;
#[doc = "Register `ANA_REG_CLK_STATUS_REG` writer"]
pub type W = crate::W<AnaRegClkStatusRegSpec>;
#[doc = "Field `SLICER_LDO_SC_OUT` reader - 0:0\\]
SLICER LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type SlicerLdoScOutR = crate::BitReader;
#[doc = "Field `SLICER_LDO_SC_OUT` writer - 0:0\\]
SLICER LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type SlicerLdoScOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APLL_VCO_LDO_SC_OUT` reader - 1:1\\]
APLL VCO LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type ApllVcoLdoScOutR = crate::BitReader;
#[doc = "Field `APLL_VCO_LDO_SC_OUT` writer - 1:1\\]
APLL VCO LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type ApllVcoLdoScOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKTOP_IOBUF_APLL_LDO_SC_OUT` reader - 2:2\\]
CLKTOP IOBUF APLL LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type ClktopIobufApllLdoScOutR = crate::BitReader;
#[doc = "Field `CLKTOP_IOBUF_APLL_LDO_SC_OUT` writer - 2:2\\]
CLKTOP IOBUF APLL LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type ClktopIobufApllLdoScOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDM_LDO_SC_OUT` reader - 3:3\\]
SDM LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type SdmLdoScOutR = crate::BitReader;
#[doc = "Field `SDM_LDO_SC_OUT` writer - 3:3\\]
SDM LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type SdmLdoScOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNTH_VCO_LDO_SC_OUT` reader - 4:4\\]
SYNTH VCO LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type SynthVcoLdoScOutR = crate::BitReader;
#[doc = "Field `SYNTH_VCO_LDO_SC_OUT` writer - 4:4\\]
SYNTH VCO LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type SynthVcoLdoScOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNTH_DIV_LDO_SC_OUT` reader - 5:5\\]
SYNTH DIV LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type SynthDivLdoScOutR = crate::BitReader;
#[doc = "Field `SYNTH_DIV_LDO_SC_OUT` writer - 5:5\\]
SYNTH DIV LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type SynthDivLdoScOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKTOP_IOBUF_ROUTE_LDO_SC_OUT` reader - 6:6\\]
CLKTOP IOBUF ROUTE LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type ClktopIobufRouteLdoScOutR = crate::BitReader;
#[doc = "Field `CLKTOP_IOBUF_ROUTE_LDO_SC_OUT` writer - 6:6\\]
CLKTOP IOBUF ROUTE LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type ClktopIobufRouteLdoScOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_20G_LDO_SC_OUT` reader - 7:7\\]
SYNC 20G LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type Sync20gLdoScOutR = crate::BitReader;
#[doc = "Field `SYNC_20G_LDO_SC_OUT` writer - 7:7\\]
SYNC 20G LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type Sync20gLdoScOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TEST_PATH_LDO_SC_OUT` reader - 8:8\\]
CLK TEST PATH LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type ClkTestPathLdoScOutR = crate::BitReader;
#[doc = "Field `CLK_TEST_PATH_LDO_SC_OUT` writer - 8:8\\]
CLK TEST PATH LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
pub type ClkTestPathLdoScOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED0` reader - 31:9\\]
Reserved Reads return 0x0 and writes have no effect."]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED0` writer - 31:9\\]
Reserved Reads return 0x0 and writes have no effect."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SLICER LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    pub fn slicer_ldo_sc_out(&self) -> SlicerLdoScOutR {
        SlicerLdoScOutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
APLL VCO LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    pub fn apll_vco_ldo_sc_out(&self) -> ApllVcoLdoScOutR {
        ApllVcoLdoScOutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
CLKTOP IOBUF APLL LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    pub fn clktop_iobuf_apll_ldo_sc_out(&self) -> ClktopIobufApllLdoScOutR {
        ClktopIobufApllLdoScOutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
SDM LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    pub fn sdm_ldo_sc_out(&self) -> SdmLdoScOutR {
        SdmLdoScOutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
SYNTH VCO LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    pub fn synth_vco_ldo_sc_out(&self) -> SynthVcoLdoScOutR {
        SynthVcoLdoScOutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
SYNTH DIV LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    pub fn synth_div_ldo_sc_out(&self) -> SynthDivLdoScOutR {
        SynthDivLdoScOutR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
CLKTOP IOBUF ROUTE LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    pub fn clktop_iobuf_route_ldo_sc_out(&self) -> ClktopIobufRouteLdoScOutR {
        ClktopIobufRouteLdoScOutR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
SYNC 20G LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    pub fn sync_20g_ldo_sc_out(&self) -> Sync20gLdoScOutR {
        Sync20gLdoScOutR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
CLK TEST PATH LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    pub fn clk_test_path_ldo_sc_out(&self) -> ClkTestPathLdoScOutR {
        ClkTestPathLdoScOutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Reserved Reads return 0x0 and writes have no effect."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SLICER LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    #[must_use]
    pub fn slicer_ldo_sc_out(&mut self) -> SlicerLdoScOutW<AnaRegClkStatusRegSpec> {
        SlicerLdoScOutW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
APLL VCO LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    #[must_use]
    pub fn apll_vco_ldo_sc_out(&mut self) -> ApllVcoLdoScOutW<AnaRegClkStatusRegSpec> {
        ApllVcoLdoScOutW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
CLKTOP IOBUF APLL LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    #[must_use]
    pub fn clktop_iobuf_apll_ldo_sc_out(
        &mut self,
    ) -> ClktopIobufApllLdoScOutW<AnaRegClkStatusRegSpec> {
        ClktopIobufApllLdoScOutW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
SDM LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    #[must_use]
    pub fn sdm_ldo_sc_out(&mut self) -> SdmLdoScOutW<AnaRegClkStatusRegSpec> {
        SdmLdoScOutW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
SYNTH VCO LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    #[must_use]
    pub fn synth_vco_ldo_sc_out(&mut self) -> SynthVcoLdoScOutW<AnaRegClkStatusRegSpec> {
        SynthVcoLdoScOutW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
SYNTH DIV LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    #[must_use]
    pub fn synth_div_ldo_sc_out(&mut self) -> SynthDivLdoScOutW<AnaRegClkStatusRegSpec> {
        SynthDivLdoScOutW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
CLKTOP IOBUF ROUTE LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    #[must_use]
    pub fn clktop_iobuf_route_ldo_sc_out(
        &mut self,
    ) -> ClktopIobufRouteLdoScOutW<AnaRegClkStatusRegSpec> {
        ClktopIobufRouteLdoScOutW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
SYNC 20G LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    #[must_use]
    pub fn sync_20g_ldo_sc_out(&mut self) -> Sync20gLdoScOutW<AnaRegClkStatusRegSpec> {
        Sync20gLdoScOutW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
CLK TEST PATH LDO SHORT CIRCUIT INDICATOR 0 = Normal operation 1 = LDO Output Short Circuit Detected"]
    #[inline(always)]
    #[must_use]
    pub fn clk_test_path_ldo_sc_out(&mut self) -> ClkTestPathLdoScOutW<AnaRegClkStatusRegSpec> {
        ClkTestPathLdoScOutW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Reserved Reads return 0x0 and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AnaRegClkStatusRegSpec> {
        Reserved0W::new(self, 9)
    }
}
#[doc = "ANA_REG_CLK_STATUS_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_clk_status_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_clk_status_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaRegClkStatusRegSpec;
impl crate::RegisterSpec for AnaRegClkStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_reg_clk_status_reg::R`](R) reader structure"]
impl crate::Readable for AnaRegClkStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_reg_clk_status_reg::W`](W) writer structure"]
impl crate::Writable for AnaRegClkStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_REG_CLK_STATUS_REG to value 0"]
impl crate::Resettable for AnaRegClkStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
