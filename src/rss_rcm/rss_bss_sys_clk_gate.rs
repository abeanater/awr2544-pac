#[doc = "Register `RSS_BSS_SYS_CLK_GATE` reader"]
pub type R = crate::R<RssBssSysClkGateSpec>;
#[doc = "Register `RSS_BSS_SYS_CLK_GATE` writer"]
pub type W = crate::W<RssBssSysClkGateSpec>;
#[doc = "Field `gated` reader - 2:0\\]
Clock gatring config for RSS BSS Data should be loaded as multibit. Write 3'b000 : Clock is ungated Write 3'b111 : Clock is gated"]
pub type GatedR = crate::FieldReader;
#[doc = "Field `gated` writer - 2:0\\]
Clock gatring config for RSS BSS Data should be loaded as multibit. Write 3'b000 : Clock is ungated Write 3'b111 : Clock is gated"]
pub type GatedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Clock gatring config for RSS BSS Data should be loaded as multibit. Write 3'b000 : Clock is ungated Write 3'b111 : Clock is gated"]
    #[inline(always)]
    pub fn gated(&self) -> GatedR {
        GatedR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Clock gatring config for RSS BSS Data should be loaded as multibit. Write 3'b000 : Clock is ungated Write 3'b111 : Clock is gated"]
    #[inline(always)]
    #[must_use]
    pub fn gated(&mut self) -> GatedW<RssBssSysClkGateSpec> {
        GatedW::new(self, 0)
    }
}
#[doc = "RSS_BSS_SYS_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_bss_sys_clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_bss_sys_clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssBssSysClkGateSpec;
impl crate::RegisterSpec for RssBssSysClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_bss_sys_clk_gate::R`](R) reader structure"]
impl crate::Readable for RssBssSysClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_bss_sys_clk_gate::W`](W) writer structure"]
impl crate::Writable for RssBssSysClkGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_BSS_SYS_CLK_GATE to value 0"]
impl crate::Resettable for RssBssSysClkGateSpec {
    const RESET_VALUE: u32 = 0;
}
