#[doc = "Register `SOC_TO_BSS_SW_INT` reader"]
pub type R = crate::R<SocToBssSwIntSpec>;
#[doc = "Register `SOC_TO_BSS_SW_INT` writer"]
pub type W = crate::W<SocToBssSwIntSpec>;
#[doc = "Field `trig` reader - 7:0\\]
Write Pulse Bit field writing to each bit field &lt;0-7>: 1'b1:triggers BSS_SW_INT_RSS_CTRL&lt;0-7> to BSS"]
pub type TrigR = crate::FieldReader;
#[doc = "Field `trig` writer - 7:0\\]
Write Pulse Bit field writing to each bit field &lt;0-7>: 1'b1:triggers BSS_SW_INT_RSS_CTRL&lt;0-7> to BSS"]
pub type TrigW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Write Pulse Bit field writing to each bit field &lt;0-7>: 1'b1:triggers BSS_SW_INT_RSS_CTRL&lt;0-7> to BSS"]
    #[inline(always)]
    pub fn trig(&self) -> TrigR {
        TrigR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Write Pulse Bit field writing to each bit field &lt;0-7>: 1'b1:triggers BSS_SW_INT_RSS_CTRL&lt;0-7> to BSS"]
    #[inline(always)]
    #[must_use]
    pub fn trig(&mut self) -> TrigW<SocToBssSwIntSpec> {
        TrigW::new(self, 0)
    }
}
#[doc = "SOC_TO_BSS_SW_INT\n\nYou can [`read`](crate::Reg::read) this register and get [`soc_to_bss_sw_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soc_to_bss_sw_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocToBssSwIntSpec;
impl crate::RegisterSpec for SocToBssSwIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_to_bss_sw_int::R`](R) reader structure"]
impl crate::Readable for SocToBssSwIntSpec {}
#[doc = "`write(|w| ..)` method takes [`soc_to_bss_sw_int::W`](W) writer structure"]
impl crate::Writable for SocToBssSwIntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOC_TO_BSS_SW_INT to value 0"]
impl crate::Resettable for SocToBssSwIntSpec {
    const RESET_VALUE: u32 = 0;
}
