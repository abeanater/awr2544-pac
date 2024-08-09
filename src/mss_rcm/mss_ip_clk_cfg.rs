#[doc = "Register `MSS_IP_CLK_CFG` reader"]
pub type R = crate::R<MssIpClkCfgSpec>;
#[doc = "Register `MSS_IP_CLK_CFG` writer"]
pub type W = crate::W<MssIpClkCfgSpec>;
#[doc = "Field `gate` reader - 31:0\\]
IP clock gating configuration bits. Data should be loaded as multibit. Writing 3'b111 will gate the IP and disable the slave"]
pub type GateR = crate::FieldReader<u32>;
#[doc = "Field `gate` writer - 31:0\\]
IP clock gating configuration bits. Data should be loaded as multibit. Writing 3'b111 will gate the IP and disable the slave"]
pub type GateW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
IP clock gating configuration bits. Data should be loaded as multibit. Writing 3'b111 will gate the IP and disable the slave"]
    #[inline(always)]
    pub fn gate(&self) -> GateR {
        GateR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
IP clock gating configuration bits. Data should be loaded as multibit. Writing 3'b111 will gate the IP and disable the slave"]
    #[inline(always)]
    #[must_use]
    pub fn gate(&mut self) -> GateW<MssIpClkCfgSpec> {
        GateW::new(self, 0)
    }
}
#[doc = "MSS_IP_CLK_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_ip_clk_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_ip_clk_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssIpClkCfgSpec;
impl crate::RegisterSpec for MssIpClkCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_ip_clk_cfg::R`](R) reader structure"]
impl crate::Readable for MssIpClkCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_ip_clk_cfg::W`](W) writer structure"]
impl crate::Writable for MssIpClkCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_IP_CLK_CFG to value 0"]
impl crate::Resettable for MssIpClkCfgSpec {
    const RESET_VALUE: u32 = 0;
}
