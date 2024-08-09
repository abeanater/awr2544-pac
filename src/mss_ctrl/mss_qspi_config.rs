#[doc = "Register `MSS_QSPI_CONFIG` reader"]
pub type R = crate::R<MssQspiConfigSpec>;
#[doc = "Register `MSS_QSPI_CONFIG` writer"]
pub type W = crate::W<MssQspiConfigSpec>;
#[doc = "Field `ext_clk` reader - 2:0\\]
Write 3'b111 to external clock as QSPI baud clock source needed for DFT IO char."]
pub type ExtClkR = crate::FieldReader;
#[doc = "Field `ext_clk` writer - 2:0\\]
Write 3'b111 to external clock as QSPI baud clock source needed for DFT IO char."]
pub type ExtClkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `clk_loopback` reader - 10:8\\]
Write 3'b111 to take board level loop back clock for QSPI"]
pub type ClkLoopbackR = crate::FieldReader;
#[doc = "Field `clk_loopback` writer - 10:8\\]
Write 3'b111 to take board level loop back clock for QSPI"]
pub type ClkLoopbackW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Write 3'b111 to external clock as QSPI baud clock source needed for DFT IO char."]
    #[inline(always)]
    pub fn ext_clk(&self) -> ExtClkR {
        ExtClkR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Write 3'b111 to take board level loop back clock for QSPI"]
    #[inline(always)]
    pub fn clk_loopback(&self) -> ClkLoopbackR {
        ClkLoopbackR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Write 3'b111 to external clock as QSPI baud clock source needed for DFT IO char."]
    #[inline(always)]
    #[must_use]
    pub fn ext_clk(&mut self) -> ExtClkW<MssQspiConfigSpec> {
        ExtClkW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Write 3'b111 to take board level loop back clock for QSPI"]
    #[inline(always)]
    #[must_use]
    pub fn clk_loopback(&mut self) -> ClkLoopbackW<MssQspiConfigSpec> {
        ClkLoopbackW::new(self, 8)
    }
}
#[doc = "MSS_QSPI_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssQspiConfigSpec;
impl crate::RegisterSpec for MssQspiConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_qspi_config::R`](R) reader structure"]
impl crate::Readable for MssQspiConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_qspi_config::W`](W) writer structure"]
impl crate::Writable for MssQspiConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_QSPI_CONFIG to value 0"]
impl crate::Resettable for MssQspiConfigSpec {
    const RESET_VALUE: u32 = 0;
}
