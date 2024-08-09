#[doc = "Register `MSS_SPIB_CFG` reader"]
pub type R = crate::R<MssSpibCfgSpec>;
#[doc = "Register `MSS_SPIB_CFG` writer"]
pub type W = crate::W<MssSpibCfgSpec>;
#[doc = "Field `spibsync2sen` reader - 2:0\\]
Donot touch the field. Used as Tie-off for IP-config."]
pub type Spibsync2senR = crate::FieldReader;
#[doc = "Field `spibsync2sen` writer - 2:0\\]
Donot touch the field. Used as Tie-off for IP-config."]
pub type Spibsync2senW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `spib_cs_trigsrc_en` reader - 8:8\\]
MIBSPIB CS Trigger SRC enable 1 : Use CS as trigger source"]
pub type SpibCsTrigsrcEnR = crate::BitReader;
#[doc = "Field `spib_cs_trigsrc_en` writer - 8:8\\]
MIBSPIB CS Trigger SRC enable 1 : Use CS as trigger source"]
pub type SpibCsTrigsrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spib_trig_gate_en` reader - 16:16\\]
When set the TRIGGER s are un-gated only when chip-select is active"]
pub type SpibTrigGateEnR = crate::BitReader;
#[doc = "Field `spib_trig_gate_en` writer - 16:16\\]
When set the TRIGGER s are un-gated only when chip-select is active"]
pub type SpibTrigGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spib_int_trig_polarity` reader - 24:24\\]
SPIB trigger source polarity select. 0 - Polarity 0, 1 -Polarity 1"]
pub type SpibIntTrigPolarityR = crate::BitReader;
#[doc = "Field `spib_int_trig_polarity` writer - 24:24\\]
SPIB trigger source polarity select. 0 - Polarity 0, 1 -Polarity 1"]
pub type SpibIntTrigPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Donot touch the field. Used as Tie-off for IP-config."]
    #[inline(always)]
    pub fn spibsync2sen(&self) -> Spibsync2senR {
        Spibsync2senR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
MIBSPIB CS Trigger SRC enable 1 : Use CS as trigger source"]
    #[inline(always)]
    pub fn spib_cs_trigsrc_en(&self) -> SpibCsTrigsrcEnR {
        SpibCsTrigsrcEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
When set the TRIGGER s are un-gated only when chip-select is active"]
    #[inline(always)]
    pub fn spib_trig_gate_en(&self) -> SpibTrigGateEnR {
        SpibTrigGateEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
SPIB trigger source polarity select. 0 - Polarity 0, 1 -Polarity 1"]
    #[inline(always)]
    pub fn spib_int_trig_polarity(&self) -> SpibIntTrigPolarityR {
        SpibIntTrigPolarityR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Donot touch the field. Used as Tie-off for IP-config."]
    #[inline(always)]
    #[must_use]
    pub fn spibsync2sen(&mut self) -> Spibsync2senW<MssSpibCfgSpec> {
        Spibsync2senW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
MIBSPIB CS Trigger SRC enable 1 : Use CS as trigger source"]
    #[inline(always)]
    #[must_use]
    pub fn spib_cs_trigsrc_en(&mut self) -> SpibCsTrigsrcEnW<MssSpibCfgSpec> {
        SpibCsTrigsrcEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
When set the TRIGGER s are un-gated only when chip-select is active"]
    #[inline(always)]
    #[must_use]
    pub fn spib_trig_gate_en(&mut self) -> SpibTrigGateEnW<MssSpibCfgSpec> {
        SpibTrigGateEnW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
SPIB trigger source polarity select. 0 - Polarity 0, 1 -Polarity 1"]
    #[inline(always)]
    #[must_use]
    pub fn spib_int_trig_polarity(&mut self) -> SpibIntTrigPolarityW<MssSpibCfgSpec> {
        SpibIntTrigPolarityW::new(self, 24)
    }
}
#[doc = "MSS_SPIB_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spib_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spib_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssSpibCfgSpec;
impl crate::RegisterSpec for MssSpibCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_spib_cfg::R`](R) reader structure"]
impl crate::Readable for MssSpibCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_spib_cfg::W`](W) writer structure"]
impl crate::Writable for MssSpibCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_SPIB_CFG to value 0"]
impl crate::Resettable for MssSpibCfgSpec {
    const RESET_VALUE: u32 = 0;
}
