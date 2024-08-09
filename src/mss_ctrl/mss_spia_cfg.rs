#[doc = "Register `MSS_SPIA_CFG` reader"]
pub type R = crate::R<MssSpiaCfgSpec>;
#[doc = "Register `MSS_SPIA_CFG` writer"]
pub type W = crate::W<MssSpiaCfgSpec>;
#[doc = "Field `spiasync2sen` reader - 2:0\\]
RESERVED:Dont Use"]
pub type Spiasync2senR = crate::FieldReader;
#[doc = "Field `spiasync2sen` writer - 2:0\\]
RESERVED:Dont Use"]
pub type Spiasync2senW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `spia_cs_trigsrc_en` reader - 8:8\\]
RESERVED:Dont Use"]
pub type SpiaCsTrigsrcEnR = crate::BitReader;
#[doc = "Field `spia_cs_trigsrc_en` writer - 8:8\\]
RESERVED:Dont Use"]
pub type SpiaCsTrigsrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spia_trig_gate_en` reader - 16:16\\]
RESERVED:Dont Use"]
pub type SpiaTrigGateEnR = crate::BitReader;
#[doc = "Field `spia_trig_gate_en` writer - 16:16\\]
RESERVED:Dont Use"]
pub type SpiaTrigGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spia_int_trig_polarity` reader - 24:24\\]
RESERVED:Dont Use"]
pub type SpiaIntTrigPolarityR = crate::BitReader;
#[doc = "Field `spia_int_trig_polarity` writer - 24:24\\]
RESERVED:Dont Use"]
pub type SpiaIntTrigPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn spiasync2sen(&self) -> Spiasync2senR {
        Spiasync2senR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn spia_cs_trigsrc_en(&self) -> SpiaCsTrigsrcEnR {
        SpiaCsTrigsrcEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn spia_trig_gate_en(&self) -> SpiaTrigGateEnR {
        SpiaTrigGateEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn spia_int_trig_polarity(&self) -> SpiaIntTrigPolarityR {
        SpiaIntTrigPolarityR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn spiasync2sen(&mut self) -> Spiasync2senW<MssSpiaCfgSpec> {
        Spiasync2senW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn spia_cs_trigsrc_en(&mut self) -> SpiaCsTrigsrcEnW<MssSpiaCfgSpec> {
        SpiaCsTrigsrcEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn spia_trig_gate_en(&mut self) -> SpiaTrigGateEnW<MssSpiaCfgSpec> {
        SpiaTrigGateEnW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn spia_int_trig_polarity(&mut self) -> SpiaIntTrigPolarityW<MssSpiaCfgSpec> {
        SpiaIntTrigPolarityW::new(self, 24)
    }
}
#[doc = "RESERVED: Dont Use\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spia_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spia_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssSpiaCfgSpec;
impl crate::RegisterSpec for MssSpiaCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_spia_cfg::R`](R) reader structure"]
impl crate::Readable for MssSpiaCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_spia_cfg::W`](W) writer structure"]
impl crate::Writable for MssSpiaCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_SPIA_CFG to value 0"]
impl crate::Resettable for MssSpiaCfgSpec {
    const RESET_VALUE: u32 = 0;
}
